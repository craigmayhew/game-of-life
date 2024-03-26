use bevy::prelude::*;
use bevy::audio::{PlaybackMode, Volume};

use crate::SOUND_BG_LOOP;

#[derive(Component)]
pub struct BackgroundMusic;


pub fn setup(mut audio_assets: ResMut<Assets<AudioSource>>, mut commands: Commands) {
    let audio_source_handle = audio_assets.add(AudioSource {
        bytes: SOUND_BG_LOOP.into(),
    });

    commands.spawn((
        AudioBundle {
            source: audio_source_handle,
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                speed: 1.0,
                paused: false,
                spatial: false,
                spatial_scale: None,
                volume: Volume::new(1.0),
            }
        },
        BackgroundMusic
    ));
}
