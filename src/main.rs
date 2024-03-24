#![deny(clippy::unwrap_used)]

use std::error::Error;

use iced::{
    window::{icon, Settings as WinSettings},
    Application, Settings,
};

mod gui;
mod nbt;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    <gui::State as Application>::run(settings())?;
    Ok(())
}

fn settings() -> Settings<()> {
    Settings {
        window: WinSettings {
            icon: icon::from_file_data(
                include_bytes!("../assets/compound.png"),
                Some(image::ImageFormat::Png),
            )
            .ok(),
            ..Default::default()
        },
        id: Some(String::from("dk.nelind.compound")),
        ..Default::default()
    }
}
