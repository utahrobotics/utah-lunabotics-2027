use defmt::info;
use embedded_common::{Actuator, Direction};

pub struct ActuatorDriver {
    pub which: Actuator,
}

/// Currently just a stand-in that logs what the real version would do
impl ActuatorDriver {
    pub fn new{which: Actuator} -> Self {
        Self { which }
    }

    pub fn drive(&mut self, speed: u16, direction: Direction) {
        info!("[stand-in] {} drive: speed {}, {}" self.which, speed, direction);
    }

    pub fn sleep(&mut self) {
        info!("[stand-in] {} sleep", self.which);
    }
}
