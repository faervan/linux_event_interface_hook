# Bind actions to linux key events

## Why?
You got some keys on your device that are not included in your keyboard layout by default.

## How?
In your `/dev/input` directory, you will find some `eventN` files (`N` is a number). Those are so called "character devices".
To detect which of those feeds the events of your special key, do `cat /dev/input/eventN` for every `N` you've got, and press your key each time.
If there is some output, it's the right one.

This program then allows you to bind actions (shell commands) to key presses and optionally add delayed commands, which activate if you hold the key long enough.

## Building
```sh
# Get the source code
git clone https://github.com/faervan/linux_event_interface_hook.git
cd linux_event_interface_hook

# Build the project using cargo (requires rustup installed)
cargo build --release

# Install to /usr/local/bin
sudo cp target/release/dev_input_tracker /usr/local/bin

# Optionally remove the build cache
rm -r target
```

## Usage
### Basic syntax
```
dev_input_tracker POLL_INTERVAL [EVENT_FILE...]
```
where:
- `POLL_INTERVAL`: The number of milliseconds a poll is allowed to take.
    This only matters when using delayed commands, as an increased `POLL_INTERVAL` means a potentially longer delay.
- `EVENT_FILE`: The path to a character device and key actions. Multiple `EVENT_FILE`'s need to be seperated by a `-`
    ```
    PATH [KEY ACTION ...]
    ```
    where:
    - `KEY`: a integer representing your key (see below)
    - `ACTION`:
        - can be just a shell command, passed to `sh -c $ACTION`
        - or a shell command, followed by a delay in seconds and a second shell command to execute when the key is held for the entire delay duration
            `COMMAND:DELAY:DELAYED_COMMAND`

**How to get the key code?**<br>
Run `dev_input_tracker 100 /dev/input/eventN`, then press your key.
You should get an output like this:
```
from /dev/input/event3:
input_event { time: timeval { tv_sec: 1759824645, tv_usec: 306486 }, type_: 1, code: 28, value: 1 }
type: Key
```
`code` is what you are looking for, here it is `28`.

### Examples
`dev_input_tracker 100 /dev/input/event3 28 "echo -e '\n\n\n\n\n\n\nThe Enter key has been pressed\n\n\n\n\n'"`

**Meaning:**<br>
Print "The Enter key has been pressed" when the key with code 28 (my enter key) was pressed

`dev_input_tracker 100 /dev/input/event1 114 "niri msg action spawn -- kill -s SIGRTMIN $(pidof wvkbd-mobintl)" - /dev/input/event3 172 "niri msg action toggle-overview":2:"niri msg action spawn -- foot" 115 "niri msg action spawn -- chatty":2:"niri msg action spawn -- gnome-calls" - /dev/input/event0 116 "":3:"niri msg action spawn -- poweroff"`

**Meaning:**<br>
Listen to various different character devices:
- `event1`: Toggle the virtual keyboard when the volume down key (114) was pressed
- `event3`:
    - Toggle the overview if the home key (172) was released
    - Spawn `foot` if the home key was held pressed for 2 seconds
    - Spawn `chatty` if the volume up key (115) was released
    - Spawn `gnome-calls` if the volume up key was held pressed for 2 seconds
- `event0`: Poweroff the system if the poweroff key (116) was held pressed for 3 seconds
