use crate::actuators::ActuatorDriver;

/// Reads commands fromt the host and executes the against the actuators
#[embassy_executor::task]
pub async fn usb_rx_loop(
    mut reader: usb::CdcReceiver,
    actuators: &'static mut [ActuatorDriver; 4],
) {
}
