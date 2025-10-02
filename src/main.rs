use std::{error::Error, fs::File, io::Read, process::Command};

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
    let mut stream = File::open("/dev/input/event3")?;
    assert_eq!(size_of::<InputEvent>(), 24);
    let mut buffer = [0; 24];

    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [ref home_cmd, ref vol_up_cmd] = args[..] else {
        return Err("Needs two arguments:\n\
            1. Bash command to be executed on Home button press\n\
            2. Bash command to be executed on Volume up button input"
            .into());
    };

    loop {
        stream.read_exact(&mut buffer)?;
        // SAFETY: buffer is exactly the size of InputEvent
        let event: InputEvent = unsafe { std::ptr::read(buffer.as_ptr() as *const _) };
        let type_ = EventType::from(event.type_);

        if type_ == EventType::Key && event.value == 1 {
            // some key was pressed
            let mut cmd = None;
            match event.code {
                115 => {
                    println!("Volume up button was pressed");
                    cmd = Some(vol_up_cmd);
                }
                172 => {
                    println!("Home button was pressed");
                    cmd = Some(home_cmd);
                }
                n => println!("{n}: unknown key value"),
            }
            if let Some(cmd) = cmd
                && let Err(e) = Command::new("bash").arg("-c").arg(cmd).spawn()
            {
                eprintln!("{e}");
            }
        }

        let mut code_fmt = type_.code(event.code).to_string();
        if !code_fmt.is_empty() {
            code_fmt = format!("\ncode: {code_fmt}");
        }
        println!("\n");
        println!("{:?}\ntype: {:?}{code_fmt}", event, type_,);
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
