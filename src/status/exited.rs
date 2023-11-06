use bevy::{app::AppExit, prelude::EventWriter};

pub fn enter_exit(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}
