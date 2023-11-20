mod manifest;

use std::rc::Rc;

use tracing::info;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    tracing_subscriber::fmt::init();

    info!("Initialising application window");
    let app = Rc::new(AppWindow::new()?);

    // Can't seem to set values for two way bindings
    // So this is a little hack to get around it
    let is_dark_mode = app.get_dark_mode();
    app.set_is_dark_mode(is_dark_mode);

    app.run()
}


