use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum PageState {
    ShowArticle,
    ShowRight,
}

impl fmt::Display for PageState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PageState::ShowArticle => write!(f, "ShowArticle"),
            PageState::ShowRight => write!(f, "ShowRight"),
        }
    }
}
