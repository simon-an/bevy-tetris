use bevy::{prelude::*, window::PrimaryWindow};

pub fn change_title(time: Res<Time>, mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.title = format!("Seconds since startup: {}", time.elapsed_secs().round());
    }
}

pub fn toggle_override(
    input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        if input.just_pressed(KeyCode::Enter) {
            let scale = window.resolution.scale_factor_override().xor(Some(1.));
            window.resolution.set_scale_factor_override(scale);
        }
    }
}

/// This system changes the scale factor override when up or down is pressed
pub fn change_scale_factor(
    input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        if input.just_pressed(KeyCode::ArrowUp) {
            let scale = window.resolution.scale_factor_override().map(|n| n + 1.);
            window.resolution.set_scale_factor_override(scale);
        } else if input.just_pressed(KeyCode::ArrowDown) {
            let scale = window
                .resolution
                .scale_factor_override()
                .map(|n| (n - 1.).max(1.));
            window.resolution.set_scale_factor_override(scale);
        }
    }
}
