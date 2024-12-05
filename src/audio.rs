use bevy::{audio::AudioSink, prelude::*};

#[derive(Component)]
pub struct MusicController;
#[derive(Resource)]
pub struct Volume(pub f32);
impl Default for Volume {
    fn default() -> Self {
        Self(1.0)
    }
}

pub fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::<AudioSource>(asset_server.load("guitar-improv.mp3")),
        MusicController,
    ));
}

pub fn volume(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    music_controller: Query<&AudioSink, With<MusicController>>,
    mut volume: ResMut<Volume>,
) {
    if keyboard_input.just_pressed(KeyCode::NumpadAdd) {
        volume.0 += 0.1;
    } else if keyboard_input.just_pressed(KeyCode::NumpadSubtract) {
        volume.0 -= 0.1;
    } else if let Ok(sink) = music_controller.get_single() {
        if volume.is_changed() {
            sink.set_volume(volume.0);
        }
    }
}
