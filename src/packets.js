// ALC gang

// This file will serve as a template for all packets

///////////////////////
/// Jetson to Robot ///
///////////////////////

// Control a specific motor
_ = {
    name: "motor_control",
    motor: 0, // Which motor to control
    spin: 1, // How much to spin the motor, negative numbers accepted
    duration: 1, // How many 20ms ticks to spin the motor for
}

///////////////////////
/// Robot to Jetson ///
///////////////////////

_ = {
    name: "update",

    // These are fetched from the pigeon sensor
    accel: [
        0,
        0,
        0,
    ],

    gyro: [
        0,
        0,
        0,
    ],

    mag: [
        0,
        0,
        0,
    ],

    yaw: 0,
    pitch: 0,
    roll: 0,
}