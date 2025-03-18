use iced::widget::{Column, button, column};

use crate::Message;

#[derive(Debug, Clone, Copy)]
pub enum WallPresetMessage {
    Select(usize),
}

impl From<WallPresetMessage> for Message {
    fn from(msg: WallPresetMessage) -> Self {
        Message::WallPresetMessage(msg)
    }
}

#[derive(Debug, Default)]
pub struct WallPresetApp {
    selected: usize,
}

impl WallPresetApp {
    pub fn new() -> Self {
        Self { selected: 0 }
    }

    pub fn view(&self) -> Column<Message> {
        column![
            button("Wall Preset 1").on_press(WallPresetMessage::Select(0).into()),
            button("Wall Preset 2").on_press(WallPresetMessage::Select(1).into()),
            button("Wall Preset 3").on_press(WallPresetMessage::Select(2).into()),
        ]
    }

    pub fn update(&mut self, msg: WallPresetMessage) {
        match msg {
            WallPresetMessage::Select(index) => self.selected = index,
        }
    }
}
