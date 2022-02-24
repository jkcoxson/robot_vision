////////////////////////////////////////////////////////
///   Vision Processing for the ALC Robotics Team    ///
/// Jackson Coxson, Josh Something, Wesley Something ///
///                     2022                         ///
////////////////////////////////////////////////////////

use opencv::prelude::*;
use opencv::videoio::VideoCapture;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tokio::spawn( async {
        let mut cam = VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();
        let mut cam2 = VideoCapture::new(1, opencv::videoio::CAP_ANY).unwrap();
        opencv::highgui::named_window("your mom", opencv::highgui::WINDOW_AUTOSIZE).unwrap();
        opencv::highgui::named_window("my mom", opencv::highgui::WINDOW_AUTOSIZE).unwrap();
        let mut frame = Mat::default();
        loop {
            cam.read(&mut frame).unwrap();
            match opencv::highgui::imshow("your mom", &frame) {
                Ok(()) => {},
                Err(e) => println!("{}", e)
            }
            cam2.read(&mut frame).unwrap();
            match opencv::highgui::imshow("my mom", &frame) {
                Ok(()) => {},
                Err(e) => println!("{}", e)
            }
            let key = opencv::highgui::wait_key(1).unwrap();
            if key == 27 {
                break;
            }
        }
    });
    

    // Create a TCP listener on port 6969
    let listener = TcpListener::bind("0.0.0.0:6969").await.unwrap();
    loop {
        // Accept a new connection
        let (_, _) = listener.accept().await.unwrap();
        println!("Got a connection!");
    }
}
