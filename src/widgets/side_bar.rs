use crate::demo_data::FRIENDS;
use crate::fonts::Icon;
use crate::ui::launcher::LauncherMessage;
use iced::{button, text_input, Button, Column, Element, Row, Text};

#[derive(Debug, Clone)]
pub struct SideBar {
    add_friend_button: button::State,
    friends: Vec<String>,
}

impl SideBar {
    pub fn new() -> Self {
        Self {
            add_friend_button: Default::default(),
            friends: FRIENDS.to_vec().iter().map(|str| str.to_string()).collect(),
        }
    }

    pub fn view(&mut self) -> Element<LauncherMessage> {
        let add_friend_button = Button::new(
            &mut self.add_friend_button,
            Row::new()
                .push(Text::new("Add Friend"))
                .push(Icon::Add.as_text(16)),
        )
        .on_press(LauncherMessage::AddFriend);

        let friends = self.friends.iter().fold(Column::new(), |col, friend| {
            col.push(Row::new().push(Text::new(friend)))
        });

        Column::new().push(add_friend_button).push(friends).into()
    }
}
