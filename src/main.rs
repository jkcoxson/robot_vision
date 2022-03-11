////////////////////////////////////////////////////////
///   Vision Processing for the ALC Robotics Team    ///
/// Jackson Coxson, Josh Something, Wesley Something ///
///                     2022                         ///
////////////////////////////////////////////////////////
use nt::{EntryData, NetworkTables};
mod constants;
mod robot;
mod tcp;

#[tokio::main]
async fn main() {
    tokio::spawn(async {});

    let mut nt = NetworkTables::connect(constants::TABLE_IP, constants::TABLE_CLIENT_NAME)
        .await
        .unwrap();
    let mut ty = None;
    let entries = nt.entries();
    for i in entries.values().into_iter() {
        if i.name == "/limelight/ty" {
            ty = Some(i);
        }
    }
    println!("{:?}", ty);

    // let tcp = tcp::Tcp::new();
    // loop {
    //     tcp.spin_motor(2, 0.8, 0);
    //     // tcp.spin_motor(1, 50.0, 0);
    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    //     tcp.spin_motor(2, 0.0, 0);
    //     // tcp.spin_motor(1, 0.0, 0);
    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    // }
}
