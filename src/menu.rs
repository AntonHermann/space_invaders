use std::io::{stdin, stdout, Write};
use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use failure::Error;

#[derive(Debug, PartialEq)]
pub enum MenuOptions {
    Singleplayer = 0,
    Multiplayer = 1,
    Exit = 2,
}
const MENUITEMS: [MenuOptions; 3] = [MenuOptions::Singleplayer, MenuOptions::Multiplayer, MenuOptions::Exit];

impl MenuOptions {
    pub fn next(self) -> MenuOptions {
        use self::MenuOptions::*;
        match self {
            Singleplayer => Multiplayer,
            Multiplayer => Exit,
            Exit => Singleplayer,
        }
    }
    pub fn prev(self) -> MenuOptions {
        use self::MenuOptions::*;
        match self {
            Singleplayer => Exit,
            Multiplayer => Singleplayer,
            Exit => Multiplayer,
        }
    }
    pub fn to_string(&self) -> &'static str {
        use self::MenuOptions::*;
        match self {
            Singleplayer => "SINGLEPLAYER",
            Multiplayer  => "MULTIPLAYER",
            Exit         => "EXIT",
        }
    }
}

pub fn show_menu() -> Result<MenuOptions, Error> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;
    let mut selected = MenuOptions::Singleplayer;

    // clear screen
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide)?;
    draw_menu(&mut stdout, &selected)?;

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Up => selected = selected.prev(),
            Key::Down => selected = selected.next(),
            Key::Char('\n') => {
                write!(stdout, "{}", termion::cursor::Show)?;
                return Ok(selected)
            },
            n => write!(stdout, "{}{:?}", termion::cursor::Goto(1,1) ,n)?,
            // _ => {},
        }
        draw_menu(&mut stdout, &selected)?;
    }
    write!(stdout, "{}", termion::cursor::Show)?;
    Ok(MenuOptions::Exit)
}

fn draw_menu<W: Write>(stdout: &mut W, selected: &MenuOptions) -> Result<(), Error> {
    use termion::{color, cursor, clear};

    let (term_width, term_height) = termion::terminal_size()?;
    let y_offset = (term_height / 2) - (MENUITEMS.len() as u16 / 2);

    for (i, mi) in MENUITEMS.iter().enumerate() {
        let mi_str = mi.to_string();
        let x_offset = (term_width / 2) - (mi_str.len() as u16 / 2);

        write!(stdout, "{}{}", cursor::Goto(x_offset, y_offset + i as u16), clear::CurrentLine)?;
        if mi == selected {
            write!(stdout, "{}", color::Fg(color::Red))?;
        }
        write!(stdout, "{}{}", mi_str, color::Fg(color::Reset))?;
    }
    stdout.flush()?;
    Ok(())
}