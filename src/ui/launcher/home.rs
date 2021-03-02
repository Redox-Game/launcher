use crate::widgets::cosmetic::solar_system::SolarSystem;
use iced::{Align, Canvas, Column, Element, Length};
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum HomeMessage {
    Display,
}

#[derive(Debug, Clone)]
pub struct HomeView {
    solar_system: SolarSystem,
}

impl HomeView {
    pub fn new() -> Self {
        Self {
            solar_system: SolarSystem::new(),
        }
    }

    pub fn update(&mut self, message: HomeMessage) {}

    pub fn view(&mut self) -> Element<HomeMessage> {
        let solar = Canvas::new(&mut self.solar_system)
            .width(Length::Fill)
            .height(Length::Fill);

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(solar)
            .into()
    }

    pub fn tick(&mut self, instant: Instant) {
        self.solar_system.tick(instant)
    }
}
