mod screen_about;
mod screen_edit_message;
mod screen_post;
mod screen_start;

pub use screen_about::*;
pub use screen_edit_message::*;
pub use screen_post::*;
pub use screen_start::*;

#[derive(Clone)]
pub enum Screen {
    Start,
    Post,
    EditMessage,
    About,
}
impl Screen {
    pub fn from_path(path: &str) -> Self {
        let p = path;

        match p {
            p if p == ScreenStart::path() => Self::Start,
            p if p == ScreenPost::path() => Self::Post,
            p if p == ScreenEditMessage::path() => Self::EditMessage,
            p if p == ScreenAbout::path() => Self::About,
            _ => Self::Start,
        }
    }

    /// Title for the desktop layout top-bar.
    pub fn desktop_title(&self) -> String {
        let base_title = ScreenStart::title();
        let delimiter = "-";

        match self {
            Self::Start => base_title.into(),
            Self::Post => format!("{} {delimiter} {base_title}", ScreenPost::title()),
            Self::EditMessage => format!("{} {delimiter} {base_title}", ScreenEditMessage::title()),
            Self::About => format!("{} {delimiter} {base_title}", ScreenAbout::title()),
        }
    }

    /// Path for navigation.
    pub fn path(&self) -> &'static str {
        match self {
            Self::Start => ScreenStart::path(),
            Self::Post => ScreenPost::path(),
            Self::EditMessage => ScreenEditMessage::path(),
            Self::About => ScreenAbout::path(),
        }
    }
}
