use crate::pid_controller::PIDController;
pub enum STATE {
    START,      // starting position until turning point
    TURNING,    // turning at the turning point
    BARRIER,    // driving to the barrier
    WAIT,       // waiting for barrier to open
    BARCODE,    // driving and detecting a barcode and pushing block
    END         // ending position and depositing ball
}

impl PartialEq for STATE {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (STATE::START, STATE::START) => true,
            (STATE::TURNING, STATE::TURNING) => true,
            (STATE::BARRIER, STATE::BARRIER) => true,
            (STATE::WAIT, STATE::WAIT) => true,
            (STATE::BARCODE, STATE::BARCODE) => true,
            (STATE::END, STATE::END) => true,
            _ => false,
        }
    }
}

pub struct StateMachine {
    pub current_state: STATE,
    pub pid_controller: PIDController,
}

impl StateMachine {
    pub fn new(pid_controller: PIDController) -> Self {
        StateMachine {
            current_state: STATE::START,
            pid_controller,
        }
    }

    pub fn set_state(&mut self, new_state: STATE) {
        self.current_state = new_state;

    }

    pub fn execute_state(&mut self) {
        match self.current_state {
            STATE::START => {
                println!("Transitioning to START state");
                self.drive();
            },
            STATE::TURNING => {
                println!("Transitioning to TURNING state");
                self.turning();
            },
            STATE::BARRIER => {
                println!("Transitioning to BARRIER state");
                    self.drive();
            },
            STATE::WAIT => {
                println!("Transitioning to WAIT state");
                self.stop();
            },
            STATE::BARCODE => {
                println!("Transitioning to BARCODE state");
                todo!("Implement BARCODE recogniction and block pushing");
            },
            STATE::END => {
                println!("Transitioning to END state");
                todo!("Implement ball depositing");
            },
        };
    }

    fn drive(&mut self) {
        self.pid_controller.drive();
    }

    fn stop(&mut self) {
        self.pid_controller.stop();
    }

    fn turning(&mut self) {
        // implement turning logic here
        self.pid_controller.turning();
    }


}