use bevy::prelude::{default, AudioSource, AudioSourceBundle, Handle, PlaybackSettings};


pub fn play_once(sound: Handle<AudioSource>) -> AudioSourceBundle {
    AudioSourceBundle {
        source: sound,
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            ..default()
        },
    }
}
