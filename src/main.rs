/*
Light Follower Robot Rust example
Author: David Laurent Reinhardt
*/

use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{LightSensor, SensorPort};
use ev3dev_lang_rust::Ev3Result;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

mod pid_controller;
use pid_controller::PIDController;

mod statemachine;
use statemachine::*;

pub enum STATE {
    START,  // starting position until turning point
    LINEFOLLOWING,  // following the line until barrier detection
    BARRIER,  // waiting at the barrier
    BARCODE,  // detecting a barcode and pushing block
    END  // ending position and depositing ball
}







// clamps a value between min and max
fn clamp(v: i32, min: i32, max: i32) -> i32 {
    if v < min { min } else if v > max { max } else { v }
}


//  "task main()" is executed at programm start
fn main() -> Ev3Result<()> {

    // Constants
    let light_left = LightSensor::get(SensorPort::In1)?;
    let light_right = LightSensor::get(SensorPort::In2)?;
    let motor = LargeMotor::get(MotorPort::OutA)?;

    // PID constants
    // for some reason when using rust the sensors goes from 0 to 1000
    //  while in Python they go from 0 to 100
    //  so we need to adjust the the terms accordingly
    let kp: f32 = 0.1;
    let ki: f32 = 0.1;
    let kd: f32 = 0.1;

    // variable to store pidController output
    motor.run_direct()?;
    light_left.set_mode_reflect()?;
    light_right.set_mode_reflect()?;
    let mut motor_power: i32;
    let mut pid_controller = PIDController::new(kp, ki, kd);

    loop {


        motor_power = pid_controller.compute(light_left.get_light_intensity()?, light_right.get_light_intensity()?)?;

        motor.set_duty_cycle_sp(clamp(motor_power, -100, 100))?;

        thread::sleep(Duration::from_millis(100));
    }
}