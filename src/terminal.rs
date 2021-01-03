use std::{
    io::{self, Write},
    mem,
    os::unix::io::AsRawFd,
};

// Control Sequence Introducer (CSI): escape character + left square bracket
macro_rules! CSI { () => { "\x1B[" } }
pub const UNDERLINE: &str = concat!(CSI!(), "4m");
pub const UNDERLINE_OFF: &str = concat!(CSI!(), "24m");
pub const STRIKE: &str = concat!(CSI!(), "9m");
pub const STRIKE_OFF: &str = concat!(CSI!(), "29m");

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

pub fn flush(mut stdout: &io::Stdout) {
    stdout.flush().unwrap_or_else(|_| {
        crate::quit("Flush failed", 1);
    });
}
