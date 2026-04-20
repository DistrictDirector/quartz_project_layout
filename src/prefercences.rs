pub struct Preferences {
    pub master_volume: f32,
    pub music_volume:  f32,
    pub sfx_volume:    f32,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume:  0.8,
            sfx_volume:    1.0,
        }
    }
}