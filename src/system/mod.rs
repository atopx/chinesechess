pub mod Audio {
    use bevy::prelude::{AudioSourceBundle, Handle, AudioSource, PlaybackSettings, default};

    pub fn play_once(sound: Handle<AudioSource>) -> AudioSourceBundle {
        AudioSourceBundle {
            source: sound,
            settings: PlaybackSettings { 
                mode: bevy::audio::PlaybackMode::Once,
                ..default()
             },
        }
    }
}
