use iced::widget::{Column, column, pick_list};

use crate::Message;

#[derive(Debug, Clone)]
pub enum WallPresetMessage {
    Select(String),
}

impl From<WallPresetMessage> for Message {
    fn from(msg: WallPresetMessage) -> Self {
        Message::WallPresetMessage(msg)
    }
}

#[derive(Debug, Default)]
pub struct WallPresetApp {
    selected: Option<String>,
}

impl WallPresetApp {
    pub fn new() -> Self {
        Self { selected: None }
    }

    pub fn view(&self) -> Column<Message> {
        column![
            pick_list(
                vec![
                    "Wall Preset 1".to_string(),
                    "Wall Preset 2".to_string(),
                    "Wall Preset 3".to_string()
                ],
                self.selected.clone(),
                |msg| Message::WallPresetMessage(WallPresetMessage::Select(msg)),
            )
            .placeholder("Select Wall Preset")
        ]
    }

    pub fn update(&mut self, msg: WallPresetMessage) {
        match msg {
            WallPresetMessage::Select(selected) => self.selected = Some(selected),
        }
    }
}
