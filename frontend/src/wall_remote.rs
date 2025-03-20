use iced::widget::{Column, column, pick_list};

use crate::Message;

#[derive(Debug, Clone)]
pub enum WallRemoteMessage {
    Select(String),
}

impl From<WallRemoteMessage> for Message {
    fn from(msg: WallRemoteMessage) -> Self {
        Message::WallRemoteMessage(msg)
    }
}

#[derive(Debug, Default)]
pub struct WallRemoteApp {
    selected: Option<String>,
}

impl WallRemoteApp {
    pub fn new() -> Self {
        Self { selected: None }
    }

    pub fn view(&self) -> Column<Message> {
        column![
            pick_list(
                vec![
                    "Wall Remote 1".to_string(),
                    "Wall Remote 2".to_string(),
                    "Wall Remote 3".to_string()
                ],
                self.selected.clone(),
                |msg| Message::WallRemoteMessage(WallRemoteMessage::Select(msg)),
            )
            .placeholder("Select Wall Remote")
        ]
    }

    pub fn update(&mut self, msg: WallRemoteMessage) {
        match msg {
            WallRemoteMessage::Select(selected) => self.selected = Some(selected),
        }
    }
}
