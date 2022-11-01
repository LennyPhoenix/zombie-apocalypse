use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;

pub fn pause() {
    thread::sleep(Duration::from_millis(750));
}

pub fn wait() {
    let mut string = String::new();
    stdin().read_line(&mut string).unwrap();
}

pub fn ellipsis() {
    let mut out = stdout();
    out.flush().unwrap();
    for _ in 0..3 {
        print!(".");
        pause();
        out.flush().unwrap();
    }
}

pub fn clear() {
    print!("{esc}c", esc = 27 as char);
}

pub fn flush() {
    stdout().flush().unwrap();
}
