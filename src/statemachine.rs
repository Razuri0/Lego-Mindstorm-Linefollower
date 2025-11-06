use crate::pid_controller::PIDController;
use std::thread;

pub enum STATE {
    START,  // starting position until turning point
    LINEFOLLOWING,  // following the line until barrier detection
    BARRIER,  // waiting at the barrier
    BARCODE,  // detecting a barcode and pushing block
    END  // ending position and depositing ball
}

struct StateMachine {
    pub current_state: STATE,
    pub pid_controller: PIDController,
    drive_thread: Option<thread::JoinHandle<()>>,
}

impl StateMachine {
    pub fn new(pid_controller: PIDController) -> Self {
        StateMachine {
            current_state: STATE::START,
            pid_controller,
            drive: None,
        }
    }

    pub fn set_state(&mut self, new_state: STATE) {
        match new_state {
            STATE::START => {
                println!("Transitioning to START state");
                self.drive();
            },
            STATE::LINEFOLLOWING => {

            },
            STATE::BARRIER => {

            },
            STATE::BARCODE => {

            },
            STATE::END => {

            },
        };
        self.current_state = new_state;
    }

    fn drive(&self) {
        thread::spawn(||){
            self.pid_controller.compute();
        }
    }


}