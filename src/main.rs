/*
Light Follower Robot Rust example
Author: David Laurent Reinhardt
*/

use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{LightSensor, UltrasonicSensor, SensorPort};
use ev3dev_lang_rust::Ev3Result;
use std::thread;
use std::time::Duration;

mod pid_controller;
use pid_controller::PIDController;

mod statemachine;
use statemachine::{STATE, StateMachine};


//  "task main()" is executed at programm start
fn main() -> Ev3Result<()> {

    // Constants
    let light_left = LightSensor::get(SensorPort::In1)?;
    let light_right = LightSensor::get(SensorPort::In2)?;
    let us_sensor = UltrasonicSensor::get(SensorPort::In3)?;
    let motor_left = LargeMotor::get(MotorPort::OutA)?;
    let motor_right = LargeMotor::get(MotorPort::OutB)?;

    // PID constants
    // for some reason when using rust the sensors goes from 0 to 1000
    //  while in Python they go from 0 to 100
    //  so we need to adjust the the terms accordingly
    let kp: f32 = 0.1;
    let ki: f32 = 0.1;
    let kd: f32 = 0.1;

    // variable to store pidController output
    motor_left.run_direct()?;
    motor_right.run_direct()?;
    light_left.set_mode_reflect()?;
    light_right.set_mode_reflect()?;
    let pid_controller = PIDController::new(kp, ki, kd, light_left, light_right, motor_left, motor_right);
    let mut state_machine = StateMachine::new(pid_controller);
    loop {
        state_machine.execute_state();

        if state_machine.current_state == STATE::START && us_sensor.get_distance().expect("cannot read ultrasonic sensor") < 15 {
            state_machine.set_state(STATE::TURNING);
        } else if state_machine.current_state == STATE::TURNING {

            state_machine.set_state(STATE::BARRIER);
        } else if state_machine.current_state == STATE::BARRIER {

            state_machine.set_state(STATE::WAIT);
        } else if state_machine.current_state == STATE::WAIT {

            state_machine.set_state(STATE::BARCODE);
        } else if state_machine.current_state == STATE::BARCODE {

            state_machine.set_state(STATE::END);
        }


        thread::sleep(Duration::from_millis(100));
    }
}