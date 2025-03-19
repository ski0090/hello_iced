mod camera;
mod wall_preset;

use camera::{CameraApp, CameraMessage};
use iced::widget::{Column, button, column, row, text};
use wall_preset::{WallPresetApp, WallPresetMessage};

#[derive(Debug, Clone, Copy)]
pub enum SwitchMenu {
    Camera,
    WallPreset,
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    SwitchToApp(SwitchMenu),
    CameraMessage(CameraMessage),
    WallPresetMessage(WallPresetMessage),
}

#[derive(Debug)]
struct MyApp {
    count: i32,
    current_app: SwitchMenu,
    camera_app: CameraApp,
    wall_preset_app: WallPresetApp,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            count: 0,
            current_app: SwitchMenu::Camera,
            camera_app: CameraApp::new(),
            wall_preset_app: WallPresetApp::new(),
        }
    }
}

impl MyApp {
    pub fn title(&self) -> String {
        "hello iced".to_string()
    }

    pub fn view(&self) -> Column<Message> {
        let app_buttons = row![
            button("Camera").on_press(Message::SwitchToApp(SwitchMenu::Camera)),
            button("Wall Preset").on_press(Message::SwitchToApp(SwitchMenu::WallPreset)),
        ];

        let current = match self.current_app {
            SwitchMenu::Camera => self.camera_app.view(),
            SwitchMenu::WallPreset => self.wall_preset_app.view(),
        };

        column![
            app_buttons,
            current,
            button("+").on_press(Message::Increment),
            text(self.count),
            button("-").on_press(Message::Decrement),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
            Message::SwitchToApp(index) => self.current_app = index,
            Message::CameraMessage(msg) => self.camera_app.update(msg),
            Message::WallPresetMessage(msg) => self.wall_preset_app.update(msg),
        }
    }
}

fn main() -> iced::Result {
    iced::application(MyApp::title, MyApp::update, MyApp::view)
        .centered()
        .run()
}
