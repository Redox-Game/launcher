use crate::ui::launcher::home::{HomeMessage, HomeView};
use crate::ui::launcher::play::{PlayMessage, PlayView};
use crate::ui::launcher::profile::{ProfileMessage, ProfileView};
use crate::ui::launcher::shop::{ShopMessage, ShopView};
use crate::widgets::top_bar::TopBar;
use iced::{scrollable, Align, Column, Container, Element, Length, Scrollable};

pub mod home;
pub mod play;
pub mod profile;
pub mod shop;

#[derive(Debug, Clone)]
pub enum LauncherMessage {
    SwitchState(LauncherSubState),

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

#[derive(Debug, Clone)]
pub struct LauncherState {
    scrollable: scrollable::State,
    top_bar: TopBar,
    current: LauncherSubState,
}

impl LauncherState {
    pub fn new() -> Self {
        Self {
            scrollable: scrollable::State::new(),
            top_bar: TopBar::new(50.0),
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
        }
    }

    pub fn view(&mut self) -> Element<LauncherMessage> {
        let top_bar = self.top_bar.view();

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

        let content = Column::new().push(top_bar).push(view);

        Scrollable::new(&mut self.scrollable)
            .align_items(Align::Center)
            .push(Container::new(content).width(Length::Fill).center_x())
            .into()
    }
}
