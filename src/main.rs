mod error;
use crate::error::LoadError;
use iced::{executor, Align, Application, Button, Column, Command, Element, Settings, Text};

fn main() -> iced::Result {
    Launcher::run(Settings::default())
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

    pub async fn load() -> Result<Self, LoadError> {
        Ok(Self::new())
    }
}

// TODO: implement this
#[derive(Debug, Clone)]
pub struct LoginState {}

#[derive(Debug, Clone)]
pub struct LauncherState {
    current: MenuState,
}

impl LauncherState {
    pub fn new() -> Self {
        Self {
            current: MenuState::Home,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MenuState {
    Home,
    Profile,
    Shop,
    Inventory,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<State, LoadError>),
}

impl Application for Launcher {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self::Loading,
            Command::perform(State::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        String::from("Redox")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Loaded(Ok(state)) => {
                *self = Launcher::Loaded(State::Launcher(LauncherState::new()))
            }
            Message::Loaded(Err(err)) => {
                // TODO error handling
                *self = Launcher::Loaded(State::new())
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Column::new().push(Text::new("Hello World")).into()
    }
}
