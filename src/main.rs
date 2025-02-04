mod detector;
mod downloader;
mod installer;
mod gui;
mod error;

use gui::App;
use error::Error;

fn main() -> Result<(), Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Universal GPU Driver",
        options,
        Box::new(|_| Box::new(App::new())),
    )
    .map_err(|e| Error::GuiError(e.to_string()))
}