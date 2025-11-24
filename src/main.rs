// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ctl;

use std::error::Error;

use ctl::controller_binding::ControllerBinding;

slint::include_modules!();



fn main() -> Result<(), Box<dyn Error>> {
    let mut controller_binding = ControllerBinding::init();
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
            let result = controller_binding.standby.toggle();
            if let Ok(result) = result {
                println!("Standby: {}", result);
            } else {
                println!("Error: {}", result.unwrap_err());
            }
        }
    });

    ui.run()?;

    Ok(())
}
