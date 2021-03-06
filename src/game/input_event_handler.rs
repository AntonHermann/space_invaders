use std::thread;
use std::sync::mpsc::{channel, Receiver};
use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;

pub fn input_events() -> Receiver<Key> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.keys() {
            let c = c.expect("error getting keyboard inputs");
            if tx.send(c).is_err() {
                // reveiver closed connection => game isn't running
                debug!("input_event channel closed");
                // break;
                return;
            }
            if c == Key::Char('q') || c == Key::Ctrl('c') {
                debug!("exit key pressed");
                return;
            }
        }
    });
    rx
}