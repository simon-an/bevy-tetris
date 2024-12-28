use bevy::prelude::*;
use states::GameLogicState;

use crate::{
    queries::{self},
    CollisionDetection, Map, MoveEvent,
};

pub(crate) fn detect_collision(
    query: Query<queries::CurrentTetrominoQuery>,
    map: Res<Map>,
    mut logic_state: ResMut<NextState<GameLogicState>>,
) {
    info!("detect_collision");
    if !query.is_empty() {
        let collision = map.detect_collision();
        info!("Collision {:?}", collision);
        if let Some(collision) = collision {
            if collision == CollisionDetection::Bottom || collision == CollisionDetection::Block {
                // info!("Collision {:?}", collision);
                logic_state.set(GameLogicState::Cleaning);
            }
        }
    }
}

pub(crate) fn play_error_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    mut move_event_rdr: EventReader<MoveEvent>,
) {
    for event in move_event_rdr.read() {
        let collision = map.detect_move_collision(event);
        debug!("Collision {:?}", collision);

        // Convert Tetromino to Blocks
        if let Some(CollisionDetection::Bottom | CollisionDetection::Block) = collision {
            // Nothing to do here. convert_to_block is now a separate system
        } else if let Some(CollisionDetection::OutOfBounds) = collision {
            commands.spawn((AudioPlayer::<AudioSource>(asset_server.load("error.mp3")),));
        }
    }
}
