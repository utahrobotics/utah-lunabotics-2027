use defmt::{info, warn};
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
    let mut decoder = cobs::CobsDecoder::new(&mut decoded_buf);
    loop {
        reader.wait_connection().await;
        'connected: loop {
            // Wait for either a USB packet or a PID drive command
            match select(reader.read_packet(&mut encoded_buf), PID_COMMANDS.receive()).await {
                // USB packet arrived
                Either::First(Ok(n)) => {
                    let mut remaining = &encoded_buf[..n];
                    while !remaining.is_empty() {
                        match decoder.push(remaining) {
                            Ok(Some(report)) => {
                                let frame = &decoder.dest()[..report.frame_size()];
                                if let Ok(frame_bytes) =
                                    <[u8; ActuatorCommand::SIZE]>::try_from(frame)
                                {
                                    match ActuatorCommand::deserialize(frame_bytes) {
                                        Ok(cmd) => {
                                            info!("received command: {}", cmd);
                                            match cmd {
                                                ActuatorCommand::SetSpeed(
                                                    speed,
                                                    actuator,
                                                    direction,
                                                ) => {
                                                    // Manual speed cancels angle target
                                                }
                                                ActuatorCommand::SetAngle(actuator, angle) => {}
                                                ActuatorCommand::Shake => {}
                                                ActuatorCommand::StartPercuss => {}
                                                ActuatorCommand::StopPercuss => {}
                                                ActuatorCommand::StopAll => {}
                                            }
                                        }
                                        Err(e) => {
                                            warn!("failed to deserialize ActuatorCommand: {}", e)
                                        }
                                    }
                                } else {
                                    warn!(
                                        "unexpected frame size: {} (expected {})",
                                        report.frame_size(),
                                        ActuatorCommand::SIZE
                                    );
                                }
                                remaining = &remaining[report.parsed_size()..];
                                decoder = cobs::CobsDecoder::new(&mut decoded_buf);
                            }
                            Ok(None) => break,
                            Err(_) => {
                                warn!("COBS decode error, resetting decoder");
                                decoder = cobs::CobsDecoder::new(&mut decoded_buf);
                                break;
                            }
                        }
                    }
                }
                // USB read error - disconnected
                Either::First(Err(_)) => {
                    break 'connected;
                }
                // PID command from the poll loop
                Either::Second((speed, actuator, direction)) => {
                    if let Some(a) = actuators.iter_mut().find(|a| a.which == actuator) {
                        a.drive(speed, direction);
                    }
                }
            }
        }
    }
}
