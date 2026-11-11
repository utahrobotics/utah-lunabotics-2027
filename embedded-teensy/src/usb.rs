use core::cell::RefCell;
use core::future::poll_fn;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::Poll;

use critical_section::Mutex;
use teensy4_bsp::usbd::{BusAdapter, EndpointMemory, EndpointState, Instances, Speed};
use usb_device::{
    LangID, UsbError,
    bus::UsbBusAllocator,
    device::{StringDescriptors, UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
};
use usbd_serial::SerialPort;

/// How the board shows up on the host. Keeping this the same as the prime Pico's
/// so the robot's computer can find the Teensy the same way.
pub struct Identity {
    pub vid: u16,
    pub pid: u16,
    pub manufacturer: &'static str,
    pub product: &'static str,
    pub serial_number: &'static str,
}

struct Stack {
    device: UsbDevice<'static, BusAdapter>,
    serial: SerialPort<'static, BusAdapter>,
    configured: bool,
}

/// The device isn't configured: it's unplugged, or  the host hasn't set it up yet
#[derive(Debug, defmt::Format)]
pub struct Disconnected;

// Shared by the USB interrupt and the tasks. It's only touched inside a critical
// section; nothing running while it's borrowed borrows it again.
static STACK: Mutex<RefCell<Option<Stack>>> = Mutex::new(RefCell::new(None));

pub fn init(usb: Instances<1>, id: Identity) {}
