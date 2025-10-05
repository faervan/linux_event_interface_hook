use std::{error::Error, ffi::OsStr, fmt::Display, io::Read as _, process::Command, time::Instant};

use crate::{EventType, cli_parse::FileListener};

pub fn read(
    listener: &mut FileListener,
    buffer: &mut [u8; size_of::<libc::input_event>()],
) -> Result<(), Box<dyn Error>> {
    let n = listener.file.read(buffer)?;
    if n == 0 {
        return Ok(());
    } else if n != buffer.len() {
        return Err(format!("Read {n} bytes, it should have been {}", buffer.len()).into());
    }

    // SAFETY: buffer is exactly the size of InputEvent
    let event: libc::input_event = unsafe { std::ptr::read(buffer.as_ptr() as *const _) };
    let type_ = EventType::from(event.type_);

    if type_ == EventType::Key {
        // some key was pressed
        for (index, action) in listener.actions.iter_mut().enumerate() {
            if event.code == action.code {
                if event.value == 1 {
                    if action.delayed_cmd.is_some() {
                        listener.schedules.insert(index, Instant::now());
                    } else {
                        command(&action.cmd);
                    }
                } else if event.value == 0 && listener.schedules.remove(&index).is_some() {
                    command(&action.cmd);
                }
                break;
            }
        }
    }

    let mut code_fmt = type_.code(event.code).to_string();
    println!("\n\nfrom {}:", listener.path);
    if !code_fmt.is_empty() {
        code_fmt = format!("\ncode: {code_fmt}");
    }
    println!("{:?}\ntype: {:?}{code_fmt}", event, type_,);

    Ok(())
}

pub fn command<S>(cmd: S)
where
    S: AsRef<OsStr> + Display,
{
    println!("executing:\nbash -c \"{cmd}\"");
    if let Err(e) = Command::new("bash").arg("-c").arg(cmd).spawn() {
        eprintln!("{e}");
    }
}
