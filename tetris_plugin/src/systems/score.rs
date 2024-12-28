use crate::*;
use bevy::prelude::*;

// pub(crate) fn score(
//     mut score: ResMut<Score>,
//     removed: RemovedComponents<Block>,
// ) {
//     removed.iter().for_each(|removed_entity| println!("{:?}", removed_entity));
//     for _ in removed.iter() {
//         *score += 100;
//     }
// }
pub(crate) fn score(
    mut score: ResMut<Score>,
    mut event: EventReader<ScoreEvent>,
    mut next_state: ResMut<NextState<GameLogicState>>,
) {
    for e in event.read() {
        *score += *e;
    }
    next_state.set(GameLogicState::Spawning);
}
