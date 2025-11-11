use ev3dev_lang_rust::sensors::LightSensor;
use ev3dev_lang_rust::motors::LargeMotor;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PIDController {
    kp: f32,
    ki: f32,
    kd: f32,
    left_light_sensor: LightSensor,
    right_light_sensor: LightSensor,
    left_motor: LargeMotor,
    right_motor: LargeMotor,
    prev_error: f32,
    integral: f32,
    current_time: i64,
    last_time: i64,
}

impl PIDController {
    pub fn new(kp: f32, ki: f32, kd: f32, left_light_sensor: LightSensor, right_light_sensor: LightSensor, left_motor: LargeMotor, right_motor: LargeMotor) -> Self {
        
        PIDController {
            kp,
            ki,
            kd,
            left_light_sensor,
            right_light_sensor,
            left_motor,
            right_motor,
            prev_error: 0.0,
            integral: 0.0,
            current_time: Self::current_time_millis(),
            last_time: Self::current_time_millis(),
        }

    }

    pub fn drive(&mut self) {
        self.left_motor.run_direct().expect("error inizialising left motor");
        self.right_motor.run_direct().expect("error inizialising right motor");
        let mut motor_power: i32;
        loop {
            motor_power = self.compute();

            self.left_motor.set_duty_cycle_sp(self.clamp(100 + motor_power, -100, 100));
            self.right_motor.set_duty_cycle_sp(self.clamp(100 - motor_power, -100, 100));
        }

    }

        // clamps a value between min and max
    fn clamp(&self, v: i32, min: i32, max: i32) -> i32 {
        if v < min { min } else if v > max { max } else { v }
    }

    fn compute(&mut self) -> i32 {
        self.current_time = Self::current_time_millis();
        let dt = (self.current_time - self.last_time) as f32 / 1000.0; // in seconds
        self.last_time = self.current_time;
        let left_sensor_value = self.left_light_sensor.get_light_intensity().expect("error reading left sensor");
        let right_sensor_value = self.right_light_sensor.get_light_intensity().expect("error reading right sensor");
        let error: f32 = (left_sensor_value - right_sensor_value) as f32;
        self.integral += error * dt;
        let mut derivative: f32 = 0.0;
        if dt > 0.0 {derivative = (error - self.prev_error) / dt;}
        let output: f32 = self.kp * error + self.ki * self.integral + self.kd * derivative;
        self.prev_error = error;
        output as i32
    }

    pub fn stop(&self) {
        self.left_motor.set_duty_cycle_sp(0).expect("left motor problems");
        self.right_motor.set_duty_cycle_sp(0).expect("right motor problems");
    }

    pub fn turning(&mut self) {
            self.left_motor.set_duty_cycle_sp(-100).expect("left motor problems");
            self.right_motor.set_duty_cycle_sp(100).expect("right motor problems");
    }

    fn current_time_millis() -> i64 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_the_epoch.as_millis() as i64
    }
}

