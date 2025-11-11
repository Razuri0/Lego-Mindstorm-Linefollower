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
    let pid_controller = PIDController::new(kp, ki, kd, light_left.clone(), light_right.clone(), motor_left, motor_right);
    let mut state_machine = StateMachine::new(pid_controller);

    // for detecting black line after turning
    let threshold: f32 = 100.0;
    let mut current_light_value: f32 = 0.0;
    let mut old_light_value: f32 = 0.0 ;

    loop {
        state_machine.execute_state();

        // follow line until hitting the wall
        if state_machine.current_state == STATE::START && us_sensor.get_distance().expect("cannot read ultrasonic sensor") < 15 {
            state_machine.set_state(STATE::TURNING);

        // turning around until right sensor touches black line
        } else if state_machine.current_state == STATE::TURNING {
            // detect black line after turning
            current_light_value = light_right.get_reflected_light_intensity().expect("cannot read light sensor");
            if current_light_value - old_light_value > threshold {
                state_machine.set_state(STATE::BARRIER);
            }
            old_light_value = current_light_value;

        // follow line until ultrasonic detects the barrier
        } else if state_machine.current_state == STATE::BARRIER && us_sensor.get_distance().expect("cannot read ultrasonic sensor") < 10 {
            state_machine.set_state(STATE::WAIT);

        // wait until barrier goes away
        } else if state_machine.current_state == STATE::WAIT && us_sensor.get_distance().expect("cannot read ultrasonic sensor") > 20 {
            state_machine.set_state(STATE::BARCODE);

        // TODO
        // follow line until detect barcode 
        } else if state_machine.current_state == STATE::BARCODE {

            state_machine.set_state(STATE::END);
        }


        thread::sleep(Duration::from_millis(100));
    }
}