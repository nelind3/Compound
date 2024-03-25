use iced::Command;
use quartz_nbt::NbtTag;
use rfd::AsyncFileDialog;
use std::{fs::File, sync::Arc};

use crate::nbt::NbtFile;

#[derive(Debug, Clone)]
pub enum Message {
    Error(Arc<anyhow::Error>),
    OpenFile,
    FileOpenCancled,
    SetNewFile(NbtFile),
    SetTag(String, NbtTag),
    SaveFile,
    CloseFile,
}

pub fn update(state: &mut super::State, message: Message) -> Command<Message> {
    match message {
        Message::OpenFile => open_file(),

        Message::FileOpenCancled => {
            log::warn!("File open cancelled");
            Command::none()
        }

        Message::SetTag(path, value) => {
            let Some(file) = &mut state.nbtfile else {
                log::error!("Cant set tag. No file open");
                return Command::none();
            };

            if let Err(err) = file.set_tag(path, value) {
                log::error!("{err}");
            }

            Command::none()
        }

        Message::SetNewFile(file) => {
            state.nbtfile = Some(file);
            Command::none()
        }

        Message::SaveFile => {
            let Some(file) = &mut state.nbtfile else {
                log::error!("Cant save. No file open");
                return Command::none();
            };

            if let Err(err) = file.write_to_disk() {
                log::error!("{err}");
            }

            Command::none()
        }

        Message::CloseFile => {
            state.nbtfile = None;
            Command::none()
        }

        Message::Error(err) => {
            log::error!("{err}");
            Command::none()
        }
    }
}

fn open_file() -> Command<Message> {
    Command::perform(
        async {
            let path_wrapper = AsyncFileDialog::new()
                .add_filter("NBT file", &["dat", "nbt"])
                .pick_file()
                .await?;

            match File::options()
                .read(true)
                .write(true)
                .append(false)
                .open(path_wrapper.path())
            {
                Ok(nbt_file_handle) => Some(NbtFile::new(nbt_file_handle)),
                Err(err) => Some(Err(err.into())),
            }
        },
        |optfile| {
            if let Some(nbtfile_result) = optfile {
                match nbtfile_result {
                    Ok(nbtfile) => Message::SetNewFile(nbtfile),
                    Err(err) => Message::Error(Arc::new(err.into())),
                }
            } else {
                Message::FileOpenCancled
            }
        },
    )
}
