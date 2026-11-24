use embassy_futures::select::{Either, select};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embedded_common::{Actuator, ActuatorCommand, Direction, MAX_MESSAGE_SIZE};

use crate::actuators::ActuatorDriver;
use crate::usb;

/// One full-speed USB packet, as on the Pico.
const PACKET_SIZE: usize = 64;

static PID_COMMANDS: Channel<CriticalSectionRawMutex, (u16, Actuator, Direction), 4> =
    Channel::new();

/// Reads commands fromt the host and executes the against the actuators
#[embassy_executor::task]
pub async fn usb_rx_loop(
    mut reader: usb::CdcReceiver,
    actuators: &'static mut [ActuatorDriver; 4],
) {
    // sets the fan to high for the whole time
    actuators[3].drive(65530, Direction::Forward);
    let mut decoded_buf = [0u8; MAX_MESSAGE_SIZE];
    let mut encoded_buf = [0u8; PACKET_SIZE];
    loop {
        reader.wait_connection().await;
        'connected: loop {
            // Wait for either a USB packet or a PID drive command
            match select(reader.read_packet(&mut encoded_buf), PID_COMMANDS.receive()).await {
                // USB packet arrived
                Either::First(Ok(n)) => {}
                // USB read error - disconnected
                Either::First(Err(_)) => {}
                // PID command from the poll loop
                Either::Second((speed, actuator, direction)) => {}
            }
        }
    }
}
