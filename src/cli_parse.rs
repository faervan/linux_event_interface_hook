use std::{error::Error, time::Duration};

pub fn cli_parse() -> Result<Vec<FileListener>, Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    let mut files = vec![];

    for group in args.split(|i| i.as_str() == "-") {
        if group.is_empty() {
            println!(
                "Example usage:\n\
                $thisbinary /dev/input/event0 12 \"niri msg action toggle-overview\" \
                - /dev/input/event1 13 \"echo this is a bash cmd\":5:\"poweroff\"\n\
                \n\
                Provide a file name, then add pairs of value (~key code) and a bash \
                command to execute when it occurs. Optionally add a ':', then a \
                duration in seconds, again a ':' and finally a second command that \
                will be executed when the key is held for the entire duration.\n\
                If the key is released before, the first command will be executed.\n\
                \n\
                Different files with their own keys can be specified, seperate them \
                by '-' as seen in the example.\
                "
            );
            return Err("Each group is required to have a minimum length of 1".into());
        }

        let mut actions = vec![];
        for action in group[1..].chunks_exact(2) {
            let [ref value, ref cmds] = action[..] else {
                unreachable!()
            };
            let (cmd, delayed_cmd) = match cmds.splitn(3, ':').collect::<Vec<_>>().as_slice() {
                [cmd, duration, delayed_cmd] => {
                    let num = duration.parse()?;
                    (
                        cmd.trim_matches('"').to_string(),
                        Some((
                            Duration::from_secs_f32(num),
                            delayed_cmd.trim_matches('"').to_string(),
                        )),
                    )
                }
                _ => (cmds.to_string(), None),
            };
            let value = value.parse()?;
            actions.push(KeyListener {
                value,
                cmd,
                delayed_cmd,
            });
        }
        files.push(FileListener {
            file: group[0].clone(),
            actions,
        });
    }

    println!("Files: {files:#?}");

    Ok(files)
}

#[derive(Debug)]
pub struct FileListener {
    pub file: String,
    pub actions: Vec<KeyListener>,
}

#[derive(Debug)]
pub struct KeyListener {
    pub value: u16,
    pub cmd: String,
    pub delayed_cmd: Option<(Duration, String)>,
}
