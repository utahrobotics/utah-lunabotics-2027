use core::cell::RefCell;
use core::future::poll_fn;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::Poll;

use critical_section::Mutex;
use defmt::info;
use embassy_sync::waitqueue::AtomicWaker;
use embassy_time::Timer;
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
pub fn init(usb: Instances<1>, id: Identity) -> (CdcSender, CdcReceiver) {
    static BUS: StaticCell<UsbBusAllocator<BusAdapter>> = StaticCell::new();

    // full-speed capped at 64-bytes for now. we could faster if we'd like to but
    // for now I'm going to keep this at the same speed as the Pico to maintain the
    // same behavior for the time being
    let adapter = BusAdapter::with_speed(usb, &EP_MEMORY, &EP_STATE, Speed::LowFull);
    adapter.set_interrupts(true);
    let bus = BUS.init(UsbBusAllocator::new(adapter));

    let serial = SerialPort::new(bus);

    let serial = SerialPort::new(bus);
    let device = UsbDeviceBuilder::new(bus, UsbVidPid(id.vid, id.pid))
        .strings(&[StringDescriptors::new(LangID::EN_US)
            .manufacturer(id.manufacturer)
            .product(id.product)
            .serial_number(id.serial_number)])
        .expect("USB string descriptors")
        .device_class(usbd_serial::USB_CLASS_CDC)
        .max_packet_size_0(64)
        .expect("USB contol packet size")
        .max_power(100)
        .expect("USB max power")
        .build();

    critical_section::with(|cs| {
        STACK.borrow(cs).replace(Some(Stack {
            device,
            serial,
            configured: false,
        }))
    });
    // Safety: handler only touches state behind critical sections.
    unsafe { cortex_m::peripheral::NVIC::unmask(interrupt::USB_OTG1) };
    (CdcSender { _private: () }, CdcReceiver { _private: () })
}

#[cortex_m_rt::interrupt]
fn USB_OTG1() {
    critical_section::with(|cs| {
        let mut stack = STACK.borrow(cs).borrow_mut();
        let Some(stack) = stack.as_mut() else {
            return;
        };

        stack.device.poll(&mut [&mut stack.serial]);

        let configured = stack.device.state() == UsbDeviceState::Configured;
        if configured != stack.configured {
            if configured {
                // needed once after all imxrt-usbd reconfiguration
                stack.device.bus().configure();
                info!("USB configured");
            } else {
                info! {"USB no longer configured"};
            }
            stack.configured = configured;
        }

        // host setting 134 baud means "reboot into the bootloader", so reflashing
        // doesn't need the Program button on the Teensy board
        if stack.serial.line_coding().data_rate() == 134 {
            REBOOT_REQUESTED.store(true, Ordering::Relaxed);
        }
    });
    RX_WAKER.wake();
    TX_WAKER.wake();
}

pub struct CdcSender {
    _private: (),
}

fn with_stack<R>(f: impl FnOnce(&mut Stack) -> R) -> Option<R> {
    critical_section::with(|cs| STACK.borrow(cs).borrow_mut().as_mut().map(f))
}

fn is_configured() -> bool {
    with_stack(|s| s.configured).unwrap_or(false)
}

/// The attempt a read came to
enum Attempt<T> {
    Done(T),
    /// Backoff instead of retrying in a tight loop as this would stave other tasks
    Backoff,
}

impl CdcSender {
    /// For waiting til the host has configured the device.
    pub async fn wait_connection(&mut self) {
        poll_fn(|cx| {
            TX_WAKER.register(cx.waker());
            if is_configured() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }
}

pub struct CdcReceiver {
    _private: (),
}

impl CdcReceiver {
    /// For waiting til the host has configurd the device
    pub async fn wait_connection(&mut self) {
        poll_fn(|cx| {
            RX_WAKER.register(cx.waker());
            if is_configured() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }

    pub async fn read_packet(&mut self, buf: &mut [u8]) -> Result<usize, Disconnected> {
        loop {
            let attempt = poll_fn(|cx| {
                RX_WAKER.register(cx.waker());
                with_stack(|s| {
                    if !s.configured {
                        return Poll::Ready(Err(Disconnected));
                    }
                    match s.serial.read(buf) {
                        Ok(n) if n > 0 => Poll::Ready(Ok(Attempt::Done(n))),
                        Ok(_) | Err(UsbError::WouldBlock) => Poll::Pending,
                        Err(_) => Poll::Ready(Ok(Attempt::Backoff)),
                    }
                })
                .unwrap_or(Poll::Ready(Err(Disconnected)))
            })
            .await?;
            match attempt {
                Attempt::Done(n) => return Ok(n),
                Attempt::Backoff => Timer::after_millis(1).await,
            }
        }
    }
}
