use iced::{Align, Column, Element, Text};
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum ShopMessage {
    Display,
}

#[derive(Debug, Clone)]
pub struct ShopView {}

impl ShopView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: ShopMessage) {}

    pub fn view(&mut self) -> Element<ShopMessage> {
        Column::new()
            .align_items(Align::Center)
            .push(Text::new("Shop Page"))
            .into()
    }

    pub fn tick(&mut self, instant: Instant) {}
}
