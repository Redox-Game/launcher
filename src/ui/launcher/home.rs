use iced::{Align, Column, Element, Text};

#[derive(Debug, Clone)]
pub enum HomeMessage {
    Display,
}

#[derive(Debug, Clone)]
pub struct HomeView {}

impl HomeView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: HomeMessage) {}

    pub fn view(&mut self) -> Element<HomeMessage> {
        Column::new()
            .align_items(Align::Center)
            .push(Text::new("Home Page"))
            .into()
    }
}
