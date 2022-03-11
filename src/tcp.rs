// ALC gang

use std::sync::Arc;

use tokio::{sync::{mpsc::UnboundedSender, Mutex}, net::TcpListener, io::{AsyncWriteExt, AsyncReadExt}};


pub struct Tcp {
    pub sender: UnboundedSender<serde_json::Value>,

    // Robot values
    accel: Arc<Mutex<(f64, f64, f64)>>,
    gyro: Arc<Mutex<(f64, f64, f64)>>,
    mag: Arc<Mutex<(f64, f64, f64)>>,
    yaw: Arc<Mutex<f64>>,
    pitch: Arc<Mutex<f64>>,
    roll: Arc<Mutex<f64>>,
}

impl Tcp {
    /// Starts a new TCP server and gives Arc reactor access to the values
    pub fn new() -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let accel = Arc::new(Mutex::new((0.0, 0.0, 0.0)));
        let gyro = Arc::new(Mutex::new((0.0, 0.0, 0.0)));
        let mag = Arc::new(Mutex::new((0.0, 0.0, 0.0)));
        let yaw = Arc::new(Mutex::new(0.0));
        let pitch = Arc::new(Mutex::new(0.0));
        let roll = Arc::new(Mutex::new(0.0));
        let accel_clone = accel.clone();
        let gyro_clone = gyro.clone();
        let mag_clone = mag.clone();
        let yaw_clone = yaw.clone();
        let pitch_clone = pitch.clone();
        let roll_clone = roll.clone();
        tokio::spawn(async move {
            let accel = accel_clone;
            let gyro = gyro_clone;
            let mag = mag_clone;
            let yaw = yaw_clone;
            let pitch = pitch_clone;
            let roll = roll_clone;
            let listener = TcpListener::bind("0.0.0.0:6969").await.expect("Starting tcp server failure!!");

            println!("Network loop started, awaiting connection");
            // Connect Loop
            loop {
                let (mut stream, addr) = match listener.accept().await {
                    Ok((a, b)) => (a, b),
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
                println!("Recieved connection from {:?}", addr);
                // We aren't waiting for any other connections at the same time so we will keep this in the same thread.
                let (mut reader, mut writer) = stream.split();
                // let read_stream = tokio::io::BufReader::new(reader);
                
                loop {
                    let mut buf = vec![];
                    tokio::select! {
                        msg = rx.recv() => {
                            let msg = msg.unwrap();
                            match writer.write_all((serde_json::to_string(&msg).unwrap() + "\n").as_bytes()).await {
                                Ok(_) => {},
                                Err(_) => println!("bad idk"),
                            }
                            writer.flush().await.unwrap();
                        },
                        _ = reader.read_buf(&mut buf) => {
                            let msg = match String::from_utf8(buf) {
                                Ok(msg) => msg,
                                Err(e) => {
                                    println!("Error reading from buffer: {}", e);
                                    break;
                                }
                            };
                            if msg.len() < 5 {
                                println!("Robot disconnected, listening for new connection...");
                                break;
                            }
                            let msg: serde_json::Value = match serde_json::from_str(&msg) {
                                Ok(msg) => msg,
                                Err(e) => {
                                    println!("Error parsing JSON: {}", e);
                                    continue;
                                }
                            };
                            
                            // Parse message
                            let msg_type = match msg["name"].as_str() {
                                Some(msg_type) => msg_type,
                                None => {
                                    println!("Error parsing JSON: No name field");
                                    continue;
                                }
                            };

                            match msg_type {
                                "update" => {
                                    let accel_val = msg["accel"].as_array().unwrap();
                                    let gyro_val = msg["gyro"].as_array().unwrap();
                                    let mag_val = msg["mag"].as_array().unwrap();
                                    let yaw_val = msg["yaw"].as_f64().unwrap();
                                    let pitch_val = msg["pitch"].as_f64().unwrap();
                                    let roll_val = msg["roll"].as_f64().unwrap();

                                    *accel.lock().await = (accel_val[0].as_f64().unwrap(), accel_val[1].as_f64().unwrap(), accel_val[2].as_f64().unwrap());
                                    *gyro.lock().await = (gyro_val[0].as_f64().unwrap(), gyro_val[1].as_f64().unwrap(), gyro_val[2].as_f64().unwrap());
                                    *mag.lock().await = (mag_val[0].as_f64().unwrap(), mag_val[1].as_f64().unwrap(), mag_val[2].as_f64().unwrap());
                                    *yaw.lock().await = yaw_val;
                                    *pitch.lock().await = pitch_val;
                                    *roll.lock().await = roll_val;
                                },
                                _ => {
                                    println!("Unknown message type: {}", msg_type);
                                }
                            }
                        }
                    }
                }
                
            }
        });
        Tcp {
            sender: tx,
            accel,
            gyro,
            mag,
            yaw,
            pitch,
            roll,
        }
    }
    
    pub fn spin_motor(&self, motor: u8, spin: f64, duration: u8) {
        let msg = serde_json::json!({
            "name": "motor_control",
            "motor": motor,
            "spin": spin,
            "duration": duration,
        });
        let _ = self.sender.send(msg);
    }

    pub fn send(&self, msg: serde_json::Value) {
        let _ = self.sender.send(msg);
    }

    pub fn send_string(&self, msg: String) {
        let _ = self.sender.send(serde_json::from_str(&msg).unwrap());
    }

    pub async fn get_accel(&self) -> (f64, f64, f64) {
        *self.accel.lock().await
    }
    pub async fn get_gyro(&self) -> (f64, f64, f64) {
        *self.gyro.lock().await
    }
    pub async fn get_mag(&self) -> (f64, f64, f64) {
        *self.mag.lock().await
    }
    pub async fn get_yaw(&self) -> f64 {
        *self.yaw.lock().await
    }
    pub async fn get_pitch(&self) -> f64 {
        *self.pitch.lock().await
    }
    pub async fn get_roll(&self) -> f64 {
        *self.roll.lock().await
    }
}

impl Clone for Tcp {
    fn clone(&self) -> Self {
        Tcp {
            sender: self.sender.clone(),
            accel: self.accel.clone(),
            gyro: self.gyro.clone(),
            mag: self.mag.clone(),
            yaw: self.yaw.clone(),
            pitch: self.pitch.clone(),
            roll: self.roll.clone(),
        }
    }
}