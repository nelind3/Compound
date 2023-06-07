use iced::{widget as w, Length, Renderer};

use super::update::Message;

pub fn view(state: &super::State) -> iced::Element<'_, Message> {
    w::column![topbar(), nbt_file_view(state)].into()
}

fn topbar<'a>() -> w::Container<'a, Message, Renderer> {
    let top_bar_style: fn(&iced::Theme) -> w::container::Appearance =
        |theme| w::container::Appearance {
            background: Some(theme.extended_palette().primary.base.color.into()),
            ..Default::default()
        };

    w::container(w::button("Open file").on_press(Message::OpenFile))
        .width(Length::Fill)
        .style(top_bar_style)
}

fn nbt_file_view(state: &super::State) -> w::Scrollable<'_, Message, Renderer> {
    w::scrollable(
        w::container(if let Some(nbtfile) = &state.nbtfile {
            w::column![
                w::button("Set tag Data.Difficulty to Int(99)").on_press(Message::SetTag(
                    "Data.Difficulty".to_string(),
                    quartz_nbt::NbtTag::Int(99)
                )),
                w::button("Set tag Data.Difficulty to Byte(2)").on_press(Message::SetTag(
                    "Data.Difficulty".to_string(),
                    quartz_nbt::NbtTag::Byte(2)
                )),
                w::button("Save file").on_press(Message::SaveFile),
                w::text(&nbtfile.roottag.to_pretty_snbt())
            ]
        } else {
            w::column![w::text("Open a file first!")]
        })
        .width(Length::Fill)
        .padding(5),
    )
    .width(Length::Fill)
}
