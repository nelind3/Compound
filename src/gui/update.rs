use iced::Command;
use quartz_nbt::NbtTag;
use rfd::AsyncFileDialog;

use crate::nbt::NbtFile;

#[derive(Debug, Clone)]
pub enum Message {
    OpenFile,
    FileOpenFailed,
    SaveFile,
    UpdateNBTView(NbtFile),
    SetTag(String, NbtTag),
    CloseFile,
}

pub fn update(state: &mut super::State, message: Message) -> Command<Message> {
    match message {
        Message::OpenFile => open_file(),

        Message::FileOpenFailed => {
            log::warn!("Couldnt open file: File open cancelled or io errored");
            Command::none()
        }

        Message::SaveFile => {
            let Some(file) = &state.nbtfile else {
                log::error!("Cant save. No file open");
                return Command::none();
            };

            if let Err(err) = file.write_to_disk() {
                log::error!("{err}");
            }

            Command::none()
        }

        Message::UpdateNBTView(file) => {
            state.nbtfile = Some(file);
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

        Message::CloseFile => {
            state.nbtfile = None;

            Command::none()
        }
    }
}

fn open_file() -> Command<Message> {
    Command::perform(
        async {
            let Some(handle) = AsyncFileDialog::new()
                .add_filter("NBT file", &["dat", "nbt"])
                .pick_file()
                .await
            else {
                return None;
            };

            match NbtFile::new(handle.path().to_path_buf()).await {
                Ok(nbtfile) => Some(nbtfile),
                Err(err) => {
                    log::error!("{}", err);
                    None
                }
            }
        },
        |optfile| {
            if let Some(nbtfile) = optfile {
                Message::UpdateNBTView(nbtfile)
            } else {
                Message::FileOpenFailed
            }
        },
    )
}
