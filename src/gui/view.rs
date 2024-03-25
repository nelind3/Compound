use iced::{
    widget::{self as w},
    Length, Theme,
};

use super::update::Message;

pub fn view(state: &super::State) -> iced::Element<'_, Message> {
    w::column![
        topbar(&state.nbtfile.is_some()),
        w::button("Set tag Data.Difficulty to Int(99)").on_press(Message::SetTag(
            "Data.Difficulty".to_string(),
            quartz_nbt::NbtTag::Int(99)
        )),
        w::button("Set tag Data.Difficulty to Byte(2)").on_press(Message::SetTag(
            "Data.Difficulty".to_string(),
            quartz_nbt::NbtTag::Byte(2)
        )),
        nbt_file_view(state)
    ]
    .into()
}

fn topbar<'a>(has_file_opened: &bool) -> w::Row<'a, Message, Theme> {
    let mut bar = w::Row::new().width(Length::Fill).padding(2).spacing(2);
    bar = bar.push(w::button("Open file").on_press(Message::OpenFile));

    if *has_file_opened {
        bar = bar.push(w::button("Close File").on_press(Message::CloseFile));
        bar = bar.push(w::button("Save File").on_press(Message::SaveFile));
    }

    bar
}

fn nbt_file_view(state: &super::State) -> w::Scrollable<'_, Message, Theme> {
    w::scrollable(
        w::container(if let Some(nbtfile) = &state.nbtfile {
            w::text(&nbtfile.roottag.0.to_pretty_snbt())
        } else {
            w::text("Open a file first!")
        })
        .padding(2),
    )
    .width(Length::Fill)
}
