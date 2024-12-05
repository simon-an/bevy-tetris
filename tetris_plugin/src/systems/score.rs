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
pub(crate) fn score(mut score: ResMut<Score>, mut event: EventReader<ScoreEvent>) {
    for e in event.read() {
        *score += *e;
    }
}
