use bevy::prelude::*;

use crate::SOUND_BG_LOOP;

pub fn setup(mut audio_assets: ResMut<Assets<AudioSource>>, mut commands: Commands) {
    let audio_source_handle = audio_assets.add(AudioSource {
        bytes: SOUND_BG_LOOP.into(),
    });

    commands.spawn(AudioBundle {
        source: audio_source_handle,
        ..default()
    });
}
