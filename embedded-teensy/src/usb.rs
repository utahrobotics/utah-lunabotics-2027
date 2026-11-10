/// How the board shows up on the host. Keeping this the same as the prime Pico's
/// so the robot's computer can find the Teensy the same way.
pub struct Identity {
    pub vid: u16,
    pub pid: u16,
    pub manufacturer: &'static str,
    pub product: &'static str,
    pub serial_number: &'static str,
}
