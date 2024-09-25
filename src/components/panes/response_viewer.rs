use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

pub struct ResponseViewer {
    focused: bool,
    focused_border_style: Style,
    content_types: Vec<String>,
    content_type_index: usize,
}
