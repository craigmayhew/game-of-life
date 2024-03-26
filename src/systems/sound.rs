use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

use crate::SOUND_BG_LOOP;
use crate::SOUND_BUTTON_CLICK;
use crate::SOUND_BUTTON_HOVER;

#[derive(Component)]
pub struct BackgroundMusic;

#[derive(Resource)]
pub struct SoundResource {
    music: bevy::prelude::Handle<AudioSource>,
    pub button_click: bevy::prelude::Handle<AudioSource>,
    pub button_hover: bevy::prelude::Handle<AudioSource>,
}

pub fn setup(mut audio_assets: ResMut<Assets<AudioSource>>, mut commands: Commands) {
    let sound_resources = SoundResource {
        music: audio_assets.add(AudioSource {
            bytes: SOUND_BG_LOOP.into(),
        }),
        button_click: audio_assets.add(AudioSource {
            bytes: SOUND_BUTTON_CLICK.into(),
        }),
        button_hover: audio_assets.add(AudioSource {
            bytes: SOUND_BUTTON_HOVER.into(),
        }),
    };

    commands.insert_resource(sound_resources);
}

pub fn update(
    mut commands: Commands,
    sound_resources: ResMut<SoundResource>,
    music_entities: Query<Entity, With<BackgroundMusic>>,
) {
    // if we don't yet have a background music entity, then spawn one
    if music_entities.iter().count() == 0 {
        // spawn background music entity
        commands.spawn((
            AudioBundle {
                source: sound_resources.music.clone(),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    speed: 1.0,
                    paused: false,
                    spatial: false,
                    spatial_scale: None,
                    volume: Volume::new(1.0),
                },
            },
            BackgroundMusic,
        ));
    }
}
