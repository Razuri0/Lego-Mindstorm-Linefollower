use ev3dev_lang_rust::Ev3Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PIDController {
    kp: f32,
    ki: f32,
    kd: f32,
    prev_error: f32,
    integral: f32,
    current_time: i64,
    last_time: i64,
}

impl PIDController {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        PIDController {
            kp,
            ki,
            kd,
            prev_error: 0.0,
            integral: 0.0,
            current_time: Self::current_time_millis(),
            last_time: Self::current_time_millis(),
        }
    }

    pub fn compute(&mut self, sensor_left: i32, sensor_right: i32) -> Ev3Result<i32> {
        self.current_time = Self::current_time_millis();
        let dt = (self.current_time - self.last_time) as f32 / 1000.0; // in seconds
        self.last_time = self.current_time;
        let error: f32 = (sensor_left - sensor_right) as f32;
        self.integral += error * dt;
        let mut derivative: f32 = 0.0;
        if dt > 0.0 {derivative = (error - self.prev_error) / dt;}
        let output: f32 = self.kp * error + self.ki * self.integral + self.kd * derivative;
        self.prev_error = error;
        Ok(output as i32)
    }

    fn current_time_millis() -> i64 {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_the_epoch.as_millis() as i64
    }
}

