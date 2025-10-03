use std::{error::Error, ffi::OsStr, io::Read as _, process::Command};

use crate::{EventType, InputEvent, cli_parse::FileListener};

pub fn poll(listener: &mut FileListener, buffer: &mut [u8]) -> Result<(), Box<dyn Error>> {
    let n = listener.file.read(buffer)?;
    if n == 0 {
        return Ok(());
    } else if n != buffer.len() {
        return Err(format!("Read {n} bytes, it should have been {}", buffer.len()).into());
    }

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
    println!("\n\nfrom {}:", listener.path);
    if !code_fmt.is_empty() {
        code_fmt = format!("code: {code_fmt}");
    }
    println!("{:?}\ntype: {:?}{code_fmt}", event, type_,);

    Ok(())
}

fn command<S>(cmd: S)
where
    S: AsRef<OsStr>,
{
    if let Err(e) = Command::new("bash").arg("-c").arg(cmd).spawn() {
        eprintln!("{e}");
    }
}
