use crate::ui::launcher::home::{HomeMessage, HomeView};
use crate::ui::launcher::play::{PlayMessage, PlayView};
use crate::ui::launcher::profile::{ProfileMessage, ProfileView};
use crate::ui::launcher::shop::{ShopMessage, ShopView};
use crate::widgets::side_bar::SideBar;
use crate::widgets::top_bar::TopBar;
use iced::{scrollable, Align, Column, Container, Element, Length, Row, Scrollable};
use std::time::Instant;

pub mod home;
pub mod play;
pub mod profile;
pub mod shop;

#[derive(Debug, Clone)]
pub enum LauncherMessage {
    SwitchState(LauncherSubState),
    AddFriend,

    PlayMessage(PlayMessage),
    HomeMessage(HomeMessage),
    ProfileMessage(ProfileMessage),
    ShopMessage(ShopMessage),
}

#[derive(Debug, Clone)]
pub enum LauncherSubState {
    Play(PlayView),
    Home(HomeView),
    Profile(ProfileView),
    Shop(ShopView),
}

impl LauncherSubState {
    pub fn tick(&mut self, instant: Instant) {
        match self {
            LauncherSubState::Play(state) => state.tick(instant),
            LauncherSubState::Home(state) => state.tick(instant),
            LauncherSubState::Profile(state) => state.tick(instant),
            LauncherSubState::Shop(state) => state.tick(instant),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LauncherState {
    top_bar: TopBar,
    side_bar: SideBar,
    current: LauncherSubState,
}

impl LauncherState {
    pub fn new() -> Self {
        Self {
            top_bar: TopBar::new(),
            side_bar: SideBar::new(),
            current: LauncherSubState::Home(HomeView::new()),
        }
    }

    pub fn update(&mut self, message: LauncherMessage) {
        match message {
            LauncherMessage::SwitchState(new_state) => {
                self.current = new_state;
            }
            LauncherMessage::PlayMessage(message) => match &mut self.current {
                LauncherSubState::Play(view) => view.update(message),
                _ => {}
            },
            LauncherMessage::HomeMessage(message) => match &mut self.current {
                LauncherSubState::Home(view) => view.update(message),
                _ => {}
            },
            LauncherMessage::ProfileMessage(message) => match &mut self.current {
                LauncherSubState::Profile(view) => view.update(message),
                _ => {}
            },
            LauncherMessage::ShopMessage(message) => match &mut self.current {
                LauncherSubState::Shop(view) => view.update(message),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn view(&mut self) -> Element<LauncherMessage> {
        let top_bar = self.top_bar.view();
        let side_bar = self.side_bar.view();

        let view = match &mut self.current {
            LauncherSubState::Play(sub_state) => sub_state
                .view()
                .map(move |message| LauncherMessage::PlayMessage(message)),
            LauncherSubState::Home(sub_state) => sub_state
                .view()
                .map(move |message| LauncherMessage::HomeMessage(message)),
            LauncherSubState::Profile(sub_state) => sub_state
                .view()
                .map(move |message| LauncherMessage::ProfileMessage(message)),
            LauncherSubState::Shop(sub_state) => sub_state
                .view()
                .map(move |message| LauncherMessage::ShopMessage(message)),
        };

        let content = Column::new()
            .push(Container::new(top_bar).width(Length::Fill).center_x())
            .push(
                Container::new(Row::new().push(view).push(side_bar))
                    .align_x(Align::End)
                    .width(Length::Fill),
            );

        Column::new()
            .align_items(Align::Center)
            .push(Container::new(content).width(Length::Fill).center_x())
            .into()
    }

    pub fn tick(&mut self, instant: Instant) {
        self.current.tick(instant);
    }
}
