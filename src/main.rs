use std::{error::Error, thread::sleep, time::Duration};

mod cli_parse;
mod listener;

#[derive(Debug)]
#[repr(C)]
struct Time {
    sec: i64,
    usec: i64,
}

#[derive(Debug)]
#[repr(C)]
struct InputEvent {
    timeval: Time,
    type_: u16,
    code: u16,
    value: u32,
}

#[derive(Debug, PartialEq)]
#[repr(u16)]
pub enum EventType {
    Syn = 0,
    Key = 1,
    Rel = 2,
    Abs = 3,
    Msc = 4,
    Sw = 5,
    Led = 6,
    Snd = 7,
    Rep = 8,
    Ff = 9,
    Pwr = 10,
    FfStatus = 11,
}

fn main() -> Result<(), Box<dyn Error>> {
    let poll_interval = std::env::args()
        .nth(1)
        .expect("Provide a poll interval")
        .parse()
        .expect("Poll interval needs to be a u64 (unsigned long integer)");

    assert_eq!(size_of::<InputEvent>(), 24);
    let mut listeners = cli_parse::cli_parse()?;

    let mut buf = [0; 24];
    loop {
        for listener in &mut listeners {
            if let Err(e) = listener::poll(listener, &mut buf) {
                eprintln!("{e}");
            }
        }

        sleep(Duration::from_millis(poll_interval));
    }
}

impl From<u16> for EventType {
    fn from(n: u16) -> Self {
        match n {
            0 => EventType::Syn,
            1 => EventType::Key,
            2 => EventType::Rel,
            3 => EventType::Abs,
            4 => EventType::Msc,
            5 => EventType::Sw,
            6 => EventType::Led,
            7 => EventType::Snd,
            8 => EventType::Rep,
            9 => EventType::Ff,
            10 => EventType::Pwr,
            11 => EventType::FfStatus,
            n => panic!(
                "unknown type: {n}, should be listed in \
                https://www.kernel.org/doc/html/v5.0/input/event-codes.html#event-types"
            ),
        }
    }
}

impl EventType {
    fn code(&self, code: u16) -> &'static str {
        match self {
            EventType::Syn => match code {
                0 => {
                    "SYN_REPORT \
                   Used to synchronize and separate events into packets of input data changes \
                   occurring at the same moment in time."
                }
                1 => "SYN_CONFIG",
                2 => {
                    "SYN_MT_REPORT \
                   Used to synchronize and separate touch events."
                }
                3 => {
                    "SYN_DROPPED \
                   Used to indicate buffer overrun in the evdev client’s event queue."
                }
                _ => "unknown code",
            },
            _ => "",
        }
    }
}
