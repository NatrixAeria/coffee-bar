//! A modular i3-bar written in rust

pub mod window;

use window::Display;

/// Runs a new coffee-bar instance
fn main() -> Result<(), String> {

    let mut dis = window::xwindow::Display::new()
                    .map_err(|e| format!("{}", e))?;

    let win: window::xwindow::Window<_> =
            dis.new_window_builder()
                 .title(String::from("coffee bar"))
                 .pos(0, 0)
                 .size(1920, 20)
                 .build()
               .map_err(|e| format!("{}", e))?;

    Ok(())
}
