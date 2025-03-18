use iced::widget::{Column, button, column};

use crate::Message;

#[derive(Debug, Clone, Copy)]
pub enum CameraMessage {
    Select(usize),
}

impl From<CameraMessage> for Message {
    fn from(msg: CameraMessage) -> Self {
        Message::CameraMessage(msg)
    }
}

#[derive(Debug, Default)]
pub struct CameraApp {
    selected: usize,
}

impl CameraApp {
    pub fn new() -> Self {
        Self { selected: 0 }
    }

    pub fn view(&self) -> Column<Message> {
        column![
            button("Camera 1").on_press(CameraMessage::Select(0).into()),
            button("Camera 2").on_press(CameraMessage::Select(1).into()),
            button("Camera 3").on_press(CameraMessage::Select(2).into()),
        ]
    }

    pub fn update(&mut self, msg: CameraMessage) {
        match msg {
            CameraMessage::Select(index) => self.selected = index,
        }
    }
}
