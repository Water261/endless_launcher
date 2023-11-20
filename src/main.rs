use std::rc::Rc;
use std::sync::RwLock;

use slint::PlatformError;
use tracing::{error, info};

use crate::manifest::get_manifest;
use crate::plugins::{get_installed_plugins, PluginManager};

mod manifest;
mod plugins;

const APP_IDENTIFIER: [&str; 3] = ["au", "water261", "endless_launcher"];

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    tracing_subscriber::fmt::init();

    let manifest = match get_manifest() {
        Ok(manifest) => Box::new(manifest),
        Err(_) => {
            error!("An error occurred when reading and parsing the manifest file");
            return Err(PlatformError::Other(String::from("endless_launcher: Manifest Error")));
        }
    };

    let plugins = match get_installed_plugins() {
        Ok(plugins) => RwLock::new(plugins),
        Err(_) => {
            error!("An error occurred when reading installed plugins");
            return Err(PlatformError::Other(String::from("endless_launcher: Installed Plugin Parse Error")));
        }
    };

    let _plugins_manager = PluginManager::new(manifest, plugins);

    info!("Initialising application window");
    let app = Rc::new(AppWindow::new()?);

    // Can't seem to set values for two way bindings
    // So this is a little hack to get around it
    let is_dark_mode = app.get_dark_mode();
    app.set_is_dark_mode(is_dark_mode);

    app.run()
}
