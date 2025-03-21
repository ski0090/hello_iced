use std::fmt::Debug;

use iced::widget::{Column, column};

use iced_video_player::{Video, VideoPlayer};

use crate::Message;

pub struct CameraApp {
    video: Video,
}

impl Debug for CameraApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VideoApp")
    }
}

impl CameraApp {
    pub fn new() -> Self {
        Self {
            video: create_video(),
        }
    }

    pub fn view(&self) -> Column<Message> {
        let video_player = VideoPlayer::new(&self.video);
        column![video_player]
    }
}

fn create_video() -> Video {
    let url = "souphttpsrc location=http://172.25.200.99/live/video33.m3u8?profile=low ! queue ! hlsdemux ! queue ! tsdemux ! queue ! h264parse ! queue ! d3d11h264dec ! queue ! videoconvert ! videoscale ! appsink name=iced_video caps=video/x-raw,format=NV12,pixel-aspect-ratio=1/1";

    Video::from_pipeline(&url, Some(true)).unwrap()
}
