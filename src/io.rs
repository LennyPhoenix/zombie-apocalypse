use crossterm::{
    cursor::MoveToPreviousLine,
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;

pub fn pause() {
    thread::sleep(Duration::from_millis(850));
}

pub fn wait() {
    print!("→");
    flush();
    read_line();
    stdout()
        .queue(MoveToPreviousLine(1))
        .expect("cursor to be moveable");
    stdout()
        .queue(Clear(ClearType::CurrentLine))
        .expect("line to be clearable");
    println!();
}

pub fn ellipsis() {
    for _ in 0..3 {
        print!(".");
        flush();
        thread::sleep(Duration::from_millis(700));
    }
}

pub fn clear() {
    print!("{esc}c", esc = 27 as char);
}

pub fn flush() {
    stdout().flush().unwrap();
}

pub fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("valid input");
    input
}
