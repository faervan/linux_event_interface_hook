use std::{error::Error, ffi::OsStr, fs::File, io::Read as _, process::Command};

use crate::{EventType, InputEvent, cli_parse::FileListener};

pub fn listen(listener: FileListener) -> Result<(), Box<dyn Error>> {
    let mut stream = File::open(listener.file)?;
    let mut buffer = [0; 24];

    loop {
        stream.read_exact(&mut buffer)?;
        // SAFETY: buffer is exactly the size of InputEvent
        let event: InputEvent = unsafe { std::ptr::read(buffer.as_ptr() as *const _) };
        let type_ = EventType::from(event.type_);

        if type_ == EventType::Key && event.value == 1 {
            // some key was pressed
            for action in &listener.actions {
                if event.code == action.value {
                    if let Some((_duration, _delayed_cmd)) = &action.delayed_cmd {
                        todo!();
                    }
                    command(&action.cmd);
                    break;
                }
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

fn command<S>(cmd: S)
where
    S: AsRef<OsStr>,
{
    if let Err(e) = Command::new("bash").arg("-c").arg(cmd).spawn() {
        eprintln!("{e}");
    }
}
