use iced::{Font, HorizontalAlignment, Length, Text};

pub const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

pub fn icon(unicode: char, size: u16) -> Text {
    Text::new(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(size))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(size)
}

pub enum Icon {
    Exit,
    Home,
    Settings,
    User,
    Friends,
    Speed,
    Play,
    Shop,
    Add,
}

impl Icon {
    pub fn as_text(&self, size: u16) -> Text {
        icon(self.as_unicode(), size)
    }

    pub fn as_unicode(&self) -> char {
        match self {
            Icon::Exit => '\u{ecf9}',
            Icon::Home => '\u{ef47}',
            Icon::Settings => '\u{efe2}',
            Icon::User => '\u{ed05}',
            Icon::Friends => '\u{ef1d}',
            Icon::Speed => '\u{eff3}',
            Icon::Play => '\u{ec74}',
            Icon::Shop => '\u{efe7}',
            Icon::Add => '\u{ec3e}',
        }
    }
}
