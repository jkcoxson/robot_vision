////////////////////////////////////////////////////////
///   Vision Processing for the ALC Robotics Team    ///
/// Jackson Coxson, Josh Something, Wesley Something ///
///                     2022                         ///
////////////////////////////////////////////////////////

use opencv::prelude::*;
use opencv::videoio::VideoCapture;

#[tokio::main]
async fn main() {
    let mut cam = VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();
    opencv::highgui::named_window("your mom", opencv::highgui::WINDOW_AUTOSIZE).unwrap();
    let mut frame = Mat::default();
    loop {
        cam.read(&mut frame).unwrap();
        opencv::highgui::imshow("your mom", &frame).unwrap();
        let key = opencv::highgui::wait_key(1).unwrap();
        if key == 27 {
            break;
        }
    }
}
