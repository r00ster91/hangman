use std::{io, mem, os::unix::io::AsRawFd};

// Each starts with a Control Sequence Introducer (CSI): escape character + left square bracket
pub const UNDERLINE: &str = "\x1B[4m";
pub const UNDERLINE_OFF: &str = "\x1B[24m";
pub const STRIKE: &str = "\x1B[9m";
pub const STRIKE_OFF: &str = "\x1B[29m";

pub fn echo(stdin: &io::Stdin, echo: bool) {
    unsafe {
        let mut termios: libc::termios = mem::zeroed();
        libc::tcgetattr(stdin.as_raw_fd(), &mut termios);
        if echo {
            termios.c_lflag |= libc::ECHO;
        } else {
            termios.c_lflag &= !libc::ECHO;
        }
        libc::tcsetattr(stdin.as_raw_fd(), 0, &termios);
    }
}
