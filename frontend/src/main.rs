#[cfg(feature = "desktop")]
mod camera;
mod wall_preset;
mod wall_remote;

use camera::CameraApp;
use iced::widget::{Column, button, column, row, text};
use wall_preset::{WallPresetApp, WallPresetMessage};
use wall_remote::{WallRemoteApp, WallRemoteMessage};

#[derive(Debug, Clone, Copy)]
pub enum SwitchMenu {
    Camera,
    WallPreset,
    WallRemote,
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    SwitchToApp(SwitchMenu),
    WallPresetMessage(WallPresetMessage),
    WallRemoteMessage(WallRemoteMessage),
}

#[derive(Debug)]
struct MyApp {
    count: i32,
    current_app: SwitchMenu,
    #[cfg(feature = "desktop")]
    camera_app: CameraApp,
    wall_preset_app: WallPresetApp,
    wall_remote_app: WallRemoteApp,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            count: 0,
            current_app: SwitchMenu::Camera,
            camera_app: CameraApp::new(),
            wall_preset_app: WallPresetApp::new(),
            wall_remote_app: WallRemoteApp::new(),
        }
    }
}

impl MyApp {
    pub fn title(&self) -> String {
        "hello iced".to_string()
    }

    pub fn view(&self) -> Column<Message> {
        let app_buttons = row![
            #[cfg(feature = "desktop")]
            button("Camera").on_press(Message::SwitchToApp(SwitchMenu::Camera)),
            button("Wall Preset").on_press(Message::SwitchToApp(SwitchMenu::WallPreset)),
            button("Wall Remote").on_press(Message::SwitchToApp(SwitchMenu::WallRemote)),
        ];

        let current = match self.current_app {
            #[cfg(feature = "desktop")]
            SwitchMenu::Camera => self.camera_app.view(),
            SwitchMenu::WallPreset => self.wall_preset_app.view(),
            SwitchMenu::WallRemote => self.wall_remote_app.view(),
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
            Message::WallPresetMessage(msg) => self.wall_preset_app.update(msg),
            Message::WallRemoteMessage(msg) => self.wall_remote_app.update(msg),
        }
    }
}

fn main() -> iced::Result {
    iced::application(MyApp::title, MyApp::update, MyApp::view)
        .centered()
        .run()
}
