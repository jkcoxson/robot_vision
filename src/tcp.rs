// ALC gang

use std::sync::Arc;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::{mpsc::UnboundedSender, Mutex},
};

pub struct Tcp {
    pub sender: UnboundedSender<serde_json::Value>,

    // Robot values
    robot_values: Arc<Mutex<Robot_Values>>,
}

pub struct Robot_Values {
    pub accel: (f64, f64, f64),
    pub gyro: (f64, f64, f64),
    pub mag: (f64, f64, f64),
    pub yaw: f64,
    pub pitch: f64,
    pub roll: f64,
}

impl Tcp {
    /// Starts a new TCP server and gives Arc reactor access to the values
    pub fn new() -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let robot_values = Arc::new(Mutex::new(Robot_Values {
            accel: (0.0, 0.0, 0.0),
            gyro: (0.0, 0.0, 0.0),
            mag: (0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
        }));
        let robot_values_return = robot_values.clone();
        tokio::spawn(async move {
            let listener = TcpListener::bind("0.0.0.0:6969")
                .await
                .expect("Starting tcp server failure!!");

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

                                    let mut lock = robot_values.lock().await;
                                    lock.accel = (accel_val[0].as_f64().unwrap(), accel_val[1].as_f64().unwrap(), accel_val[2].as_f64().unwrap());
                                    lock.gyro = (gyro_val[0].as_f64().unwrap(), gyro_val[1].as_f64().unwrap(), gyro_val[2].as_f64().unwrap());
                                    lock.mag = (mag_val[0].as_f64().unwrap(), mag_val[1].as_f64().unwrap(), mag_val[2].as_f64().unwrap());
                                    lock.yaw = yaw_val;
                                    lock.pitch = pitch_val;
                                    lock.roll = roll_val;
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
            robot_values: robot_values_return,
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
        self.robot_values.lock().await.accel
    }
    pub async fn get_gyro(&self) -> (f64, f64, f64) {
        self.robot_values.lock().await.gyro
    }
    pub async fn get_mag(&self) -> (f64, f64, f64) {
        self.robot_values.lock().await.mag
    }
    pub async fn get_yaw(&self) -> f64 {
        self.robot_values.lock().await.yaw
    }
    pub async fn get_pitch(&self) -> f64 {
        self.robot_values.lock().await.pitch
    }
    pub async fn get_roll(&self) -> f64 {
        self.robot_values.lock().await.roll
    }
}

impl Clone for Tcp {
    fn clone(&self) -> Self {
        Tcp {
            sender: self.sender.clone(),
            robot_values: self.robot_values.clone(),
        }
    }
}
