use iced::{Align, Column, Element, Text};
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum PlayMessage {
    Display,
}

#[derive(Debug, Clone)]
pub struct PlayView {}

impl PlayView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: PlayMessage) {}

    pub fn view(&mut self) -> Element<PlayMessage> {
        Column::new()
            .align_items(Align::Center)
            .push(Text::new("Play Page"))
            .into()
    }

    pub fn tick(&mut self, instant: Instant) {}
}
