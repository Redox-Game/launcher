use iced::{Align, Column, Element, Text, button, Button, Row, Length};
use std::time::Instant;
use crate::fonts::Icon;

#[derive(Debug, Clone)]
pub enum PlayMessage {
    Display,
    Pressed
}

#[derive(Debug, Clone)]
pub struct PlayView {
    quick_play_button: button::State,
    random_mode_button: button::State,
}

impl PlayView {
    pub fn new() -> Self {
        Self {
            quick_play_button: button::State::new(),
            random_mode_button: button::State::new()
        }
    }

    pub fn update(&mut self, message: PlayMessage) {
        match message {
            PlayMessage::Display => {}
            PlayMessage::Pressed => {
                println!("Pressed")
            }
        }
    }

    pub fn view(&mut self) -> Element<PlayMessage> {
        let quick_play = Button::new(
            &mut self.quick_play_button,
            Row::new()
                .push(Text::new("Quick Play"))
                .push(Icon::Play.as_text(16)),
        )
            .on_press(PlayMessage::Pressed);

        let random_play = Button::new(
            &mut self.random_mode_button,
            Row::new()
                .push(Text::new("Random Play"))
                .push(Icon::Play.as_text(16)),
        )
            .on_press(PlayMessage::Pressed);

        let content = Row::new()
            .align_items(Align::Center)
            .push(quick_play)
            .push(random_play);

        Column::new()
            .push(Text::new("Play Page"))
            .push(content)
            .align_items(Align::Center)
            .into()
    }

    pub fn tick(&mut self, instant: Instant) {}
}
