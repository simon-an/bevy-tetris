use bevy::prelude::Resource;

use crate::components::Coordinates;

#[derive(Debug, Resource)]
pub struct Transitions(pub Vec<(Coordinates, Coordinates)>);
