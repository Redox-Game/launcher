#![windows_subsystem = "windows"]
#![allow(unused)]
#![deny(unused_imports)]

use crate::error::LoadError;
use crate::ui::launcher::{LauncherMessage, LauncherState};
use crate::ui::login::{LoginMessage, LoginState};
use iced::{executor, Application, Command, Element, Settings, Text};

mod demo_data;
mod error;
mod fonts;
mod ui;
mod widgets;

fn main() -> iced::Result {
    Launcher::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    Loading,
    Loaded(Result<State, LoadError>),
    StateMessage(StateMessage),
}

#[derive(Debug, Clone)]
pub enum StateMessage {
    LauncherMessage(LauncherMessage),
    LoginMessage(LoginMessage),
}

#[derive(Debug, Clone)]
pub enum Launcher {
    Loading,
    Loaded(State),
}

#[derive(Debug, Clone)]
pub enum State {
    Login(LoginState),
    Launcher(LauncherState),
}

impl State {
    pub fn new() -> Self {
        Self::Launcher(LauncherState::new())
    }

    pub fn update(&mut self, message: StateMessage) {
        match message {
            StateMessage::LauncherMessage(launcher_message) => match self {
                State::Launcher(launcher) => {
                    launcher.update(launcher_message);
                }
                _ => {}
            },
            StateMessage::LoginMessage(login_message) => match self {
                State::Login(login) => {
                    unimplemented!()
                }
                _ => {}
            },
        }
    }

    pub fn view(&mut self) -> Element<StateMessage> {
        match self {
            State::Login(state) => {
                unimplemented!()
            }
            State::Launcher(state) => state
                .view()
                .map(move |message| StateMessage::LauncherMessage(message)),
        }
    }

    pub async fn load() -> Result<Self, LoadError> {
        Ok(Self::new())
    }
}

impl Application for Launcher {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self::Loading,
            Command::perform(State::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("Redox")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            Launcher::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Launcher::Loaded(state);
                    }
                    Message::Loaded(Err(err)) => {
                        // TODO error handling
                        eprintln!("{:?}", err);
                        *self = Launcher::Loaded(State::new());
                    }
                    _ => {}
                }
            }
            Launcher::Loaded(state) => match message {
                Message::StateMessage(state_message) => {
                    state.update(state_message);
                }
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Launcher::Loading => Text::new("Loading...").into(),
            Launcher::Loaded(state) => state
                .view()
                .map(move |message| Message::StateMessage(message)),
        }
    }
}
