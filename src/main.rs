// extern crates
#[allow(unused_imports)]
#[macro_use] extern crate log;
extern crate clap;
extern crate loggerv;
extern crate failure;
#[allow(unused_imports)]
#[macro_use] extern crate failure_derive;
extern crate termion;
extern crate specs;

// modules
mod menu;
mod game;

// external uses
use clap::{App, Arg};
use failure::Error;

// internal uses
use menu::show_menu;
use menu::MenuOptions;

fn main() {
    if let Err(e) = run() {
        eprintln!("{:?}", e);
        ::std::process::exit(1);
    }
}

fn run() -> Result<(),Error> {
    let args = App::new("Space Invaders")
        .version("0.1.0")
        .author("Anton Hermann")
        .about("Simple Space Invaders clone in Rust")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();
    loggerv::init_with_verbosity(args.occurrences_of("v"))?;

    loop {
        match show_menu()? {
            MenuOptions::Exit => break,
            _ => game::run_game()?,
        }
    }
    println!("{}{}{}{}", termion::cursor::Goto(1,1), termion::clear::All, termion::style::Reset, termion::cursor::Show);
    Ok(())
}

// error!("This is always printed");
// warn!("This too is always printed to stderr");
// info!("This is optionally printed to stdout");  // for ./app -v or higher
// debug!("This is optionally printed to stdout"); // for ./app -vv or higher
// trace!("This is optionally printed to stdout"); // for ./app -vvv