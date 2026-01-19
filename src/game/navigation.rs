#[cfg(feature = "ui")]
use crate::{View, CurrentView};


#[cfg(feature = "ui")]
impl View {
    fn default() -> Self {
        Self { current_view: CurrentView::Lobby }
    }
    pub fn new() -> Self {
        Self::default() 
    }

    pub fn from(view: &CurrentView) -> Self {
        Self { current_view: *view }
    }
}

