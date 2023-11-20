use std::rc::Rc;

use slint::PlatformError;
use tracing::{error, info};

use crate::manifest::get_manifest;

mod manifest;

const APP_IDENTIFIER: [&'static str; 3] = ["au", "water261", "endless_launcher"];

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    tracing_subscriber::fmt::init();

    let manifest = match get_manifest() {
        Ok(manifest) => Rc::new(Box::new(manifest)),
        Err(_) => {
            error!("An error occurred when reading and parsing the manifest file");
            return Err(PlatformError::Other(String::from("endless_launcher: Manifest Error")));
        }
    };

    info!("Initialising application window");
    let app = Rc::new(AppWindow::new()?);

    // Can't seem to set values for two way bindings
    // So this is a little hack to get around it
    let is_dark_mode = app.get_dark_mode();
    app.set_is_dark_mode(is_dark_mode);

    app.run()
}
