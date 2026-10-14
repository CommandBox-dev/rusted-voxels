use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, sound::static_sound::StaticSoundData,
};

pub struct AudioServer {
    manager: AudioManager<DefaultBackend>,
}

impl AudioServer {
    pub fn new() -> Self {
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .expect("Failed to create Audio Manager!");
        Self { manager }
    }

    pub fn play_sound_flat(&mut self, sound: StaticSoundData) {
        self.manager.play(sound);
    }

    pub fn stream_sound_flat(&mut self, sound: StaticSoundData) {}

    pub fn play_sound_3d(&mut self, path: &str) {}

    pub fn stream_sound_3d(&mut self, path: &str) {}
}
