//! A modular i3-bar written in rust

pub mod window;

use window::{Display, WindowType};

/// Runs a new coffee-bar instance
fn main() -> Result<(), String> {

    let dis = window::xwindow::Display::new()
                    .map_err(|e| format!("{}", e))?;

    let size = dis.get_screen_dimension(dis.get_main_screen())
                    .ok_or_else(|| String::from("No screen available"))?;

    let win: window::xwindow::Window =
            dis.new_window_builder()
                 .title(String::from("coffee bar"))
                 .pos(0, 0)
                 .size(size.0, 40)
                 .transparency(true)
                 .window_type(WindowType::Docking)
                 .build()
               .map_err(|e| format!("{}", e))?;

    for event in win {
        println!("event: {:?}", event);
    }

    Ok(())
}
