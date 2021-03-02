use iced::{Align, Column, Element, Text};
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum ProfileMessage {
    Display,
}

#[derive(Debug, Clone)]
pub struct ProfileView {}

impl ProfileView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: ProfileMessage) {}

    pub fn view(&mut self) -> Element<ProfileMessage> {
        Column::new()
            .align_items(Align::Center)
            .push(Text::new("Profile Page"))
            .into()
    }

    pub fn tick(&mut self, instant: Instant) {}
}
