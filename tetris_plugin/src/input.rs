use bevy::log;
use bevy::prelude::*;

use crate::MoveEvent;
use crate::RotateEvent;

pub fn input(
    keys: Res<Input<KeyCode>>,
    mut move_ewr: EventWriter<MoveEvent>,
    mut rotate_ewr: EventWriter<RotateEvent>,
) {
    if keys.just_pressed(KeyCode::Left) {
        log::debug!("move left");
        move_ewr.send(MoveEvent::Left);
    }
    if keys.just_pressed(KeyCode::Right) {
        log::debug!("move right");

        move_ewr.send(MoveEvent::Right);
    }
    if keys.just_pressed(KeyCode::Down) {
        log::debug!("rotate ClockWise");
        rotate_ewr.send(RotateEvent::ClockWise);
    }
    if keys.just_pressed(KeyCode::Up) {
        log::debug!("rotate CounterClockWise");
        rotate_ewr.send(RotateEvent::CounterClockWise);
    }
    if keys.just_pressed(KeyCode::Space) {
        log::debug!("move down");
        move_ewr.send(MoveEvent::Down);
    }
}
