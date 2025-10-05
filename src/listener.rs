use std::{error::Error, ffi::OsStr, fmt::Display, io::Read as _, process::Command, time::Instant};

use crate::{EventType, cli_parse::FileListener};

pub fn read(
    listener: &mut FileListener,
    buffer: &mut [u8; size_of::<libc::input_event>()],
) -> Result<(), Box<dyn Error>> {
    println!("{} schedules", listener.schedules.len());
    let finished_schedules =
        listener
            .schedules
            .iter()
            .fold(vec![], |mut acc, (index, schedule_time)| {
                let (duration, cmd) = listener.actions[*index].delayed_cmd.as_ref().unwrap();
                println!("elapsed: {}", schedule_time.elapsed().as_secs_f32());
                if schedule_time.elapsed() > *duration {
                    command(cmd);
                    acc.push(*index);
                }
                acc
            });

    for index in finished_schedules {
        listener.schedules.remove(&index);
    }

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

fn command<S>(cmd: S)
where
    S: AsRef<OsStr> + Display,
{
    println!("executing:\nbash -c \"{cmd}\"");
    if let Err(e) = Command::new("bash").arg("-c").arg(cmd).spawn() {
        eprintln!("{e}");
    }
}
