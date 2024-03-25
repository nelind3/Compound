#![deny(clippy::unwrap_used)]

use iced::{
    window::{icon, Settings as WinSettings},
    Application, Settings,
};

mod gui;
mod nbt;

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    <gui::State as Application>::run(settings())?;
    Ok(())
}

fn settings() -> Settings<()> {
    Settings {
        window: WinSettings {
            icon: icon::from_file_data(include_bytes!("../assets/compound.png"), None).ok(),
            ..Default::default()
        },
        id: Some(String::from("dk.nelind.compound")),
        ..Default::default()
    }
}
