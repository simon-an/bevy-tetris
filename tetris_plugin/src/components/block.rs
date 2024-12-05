use bevy::prelude::{Component, Reflect};

#[cfg_attr(feature = "debug", derive(Reflect))]
#[derive(Debug, Component, Clone)]
pub struct Block;
