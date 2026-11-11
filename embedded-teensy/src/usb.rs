use core::cell::RefCell;
use core::future::poll_fn;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::Poll;

use critical_section::Mutex;
use embassy_sync::waitqueue::AtomicWaker;
use static_cell::StaticCell;
use teensy4_bsp::interrupt;
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

/// The device isn't configured: it's unplugged, or  the host hasn't set it up yet
#[derive(Debug, defmt::Format)]
pub struct Disconnected;

struct Stack {
    device: UsbDevice<'static, BusAdapter>,
    serial: SerialPort<'static, BusAdapter>,
    configured: bool,
}

// Shared by the USB interrupt and the tasks. It's only touched inside a critical
// section; nothing running while it's borrowed borrows it again.
static STACK: Mutex<RefCell<Option<Stack>>> = Mutex::new(RefCell::new(None));
static RX_WAKER: AtomicWaker = AtomicWaker::new();
static TX_WAKER: AtomicWaker = AtomicWaker::new();
static REBOOT_REQUESTED: AtomicBool = AtomicBool::new(false);

// keep these two in ordinary statics to keep them uncached in memory
// since imxrt-usbd(the rust crate for driving the USB controller)'s
// cache maintenance doesn't work because imxrt-usbd plugs into
// usb-device and not embassy-usb. moving them int OCRAM breaks USB
static EP_MEMORY: EndpointMemory<2048> = EndpointMemory::new();
static EP_STATE: EndpointState = EndpointState::max_endpoints();

// Initialize USB device and start its interrupt; provides a USB Communications
// Device Class(CDC) Sender and receiver
pub fn init(usb: Instances<1>, id: Identity) -> (CDCSender, CDCReceiver) {
    static BUS: StaticCell<UsbBusAllocator<BusAdapter>> = StaticCell::new();

    // full-speed capped at 64-bytes for now. we could faster if we'd like to but
    // for now I'm going to keep this at the same speed as the Pico to maintain the
    // same behavior for the time being
    let adapter = BusAdapter::with_speed(usb, &EP_MEMORY, &EP_STATE, Speed::LowFull);
    adapter.set_interrupts(true);

    // Safety: handler only touches state behind critical sections.
    unsafe { cortex_m::peripheral::NVIC::unmask(interrupt::USB_OTG1) };
    (CDCSender { _private: () }, CDCReceiver { _private: () })
}

pub struct CDCSender {
    _private: (),
}

pub struct CDCReceiver {
    _private: (),
}
