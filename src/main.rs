//! A modular i3-bar written in rust

#![feature(const_fn)]
#![feature(move_ref_pattern)]

mod bar;
mod error;
pub mod window;

pub use bar::{Bar, X11Bar};
pub use error::BarError;

fn run_bar() -> Result<(), BarError> {
    Ok(X11Bar::new()?.main_loop()?)
}

/// Runs a new coffee-bar instance
fn main() {
    if let Err(err) = run_bar() {
        eprintln!("error: {}", err);
        std::process::exit(1)
    }
}
