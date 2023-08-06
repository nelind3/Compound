use iced::{executor, Application, Command, Theme};

use crate::nbt::NbtFile;

mod update;
mod view;

pub struct State {
    nbtfile: Option<NbtFile>,
    current_theme: Theme,
}

impl Application for State {
    type Executor = executor::Default;
    type Flags = ();
    type Theme = iced::Theme;
    type Message = update::Message;

    fn title(&self) -> String {
        String::from("Compound")
    }

    fn theme(&self) -> Self::Theme {
        self.current_theme.clone()
    }

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            State {
                nbtfile: None,
                current_theme: Theme::Dark,
            },
            Command::none(),
        )
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        view::view(self)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        update::update(self, message)
    }
}
