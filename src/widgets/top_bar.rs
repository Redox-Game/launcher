use crate::fonts::Icon;
use crate::ui::launcher::home::HomeView;
use crate::ui::launcher::play::PlayView;
use crate::ui::launcher::profile::ProfileView;
use crate::ui::launcher::shop::ShopView;
use crate::ui::launcher::{LauncherMessage, LauncherSubState};
use iced::{button, Align, Button, Element, Row, Text};

#[derive(Debug, Clone)]
pub struct TopBar {
    height: f32,
    play_button: button::State,
    home_button: button::State,
    profile_button: button::State,
    shop_button: button::State,
    // settings_button: button::State,
}

impl TopBar {
    pub fn new(height: f32) -> Self {
        Self {
            height,
            play_button: button::State::new(),
            home_button: button::State::new(),
            profile_button: button::State::new(),
            shop_button: button::State::new(),
            // settings_button: button::State::new()
        }
    }
    pub fn view(&mut self) -> Element<LauncherMessage> {
        let play_button = Button::new(
            &mut self.play_button,
            Row::new()
                .push(Text::new("Play"))
                .push(Icon::Play.as_text(16)),
        )
        .on_press(LauncherMessage::SwitchState(LauncherSubState::Play(
            PlayView::new(),
        )));

        let home_button = Button::new(
            &mut self.home_button,
            Row::new()
                .push(Text::new("Home"))
                .push(Icon::Home.as_text(16)),
        )
        .on_press(LauncherMessage::SwitchState(LauncherSubState::Home(
            HomeView::new(),
        )));

        let profile_button = Button::new(
            &mut self.profile_button,
            Row::new()
                .push(Text::new("Profile"))
                .push(Icon::User.as_text(16)),
        )
        .on_press(LauncherMessage::SwitchState(LauncherSubState::Profile(
            ProfileView::new(),
        )));

        let shop_button = Button::new(
            &mut self.shop_button,
            Row::new()
                .push(Text::new("Shop"))
                .push(Icon::Shop.as_text(16)),
        )
        .on_press(LauncherMessage::SwitchState(LauncherSubState::Shop(
            ShopView::new(),
        )));

        Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(play_button)
            .push(home_button)
            .push(profile_button)
            .push(shop_button)
            .into()
    }
}
