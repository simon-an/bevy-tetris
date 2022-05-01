use std::collections::BTreeMap;

use crate::{
    Board, Coordinates, CurrentTetromino, GameOverEvent, Map, Shape, ShapeEntity, ShapePosition,
    ShapeType, SpawnEvent, Tetromino, Tile,
};
use bevy::prelude::*;

pub(crate) fn spawn_tetromino(
    mut commands: Commands,
    board: Res<Board>,
    mut map: ResMut<Map>,
    // board_assets: Res<BoardAssets>,
    mut spawn_event_rdr: EventReader<SpawnEvent>,
    mut game_over_event_wtr: EventWriter<GameOverEvent>,
    // map: Query<&Children>,
) {
    for event in spawn_event_rdr.iter() {
        info!("EVENT {:?}", event);
        let selected_shape_type: ShapeType = rand::random(); //TODO Get shape type from queue
        let Shape {
            anker,
            layout,
            shape_type,
            ..
        } = Shape::blueprint(selected_shape_type);
        let blocks = Tetromino::blocks_from_type(selected_shape_type);
        let mut entity_positions: BTreeMap<Entity, ShapePosition> = BTreeMap::default();
        assert_eq!(selected_shape_type, shape_type);

        commands.entity(board.entity).with_children(|mut parent| {
            // TODO Iterate over blueprint.positions instead
            for block in blocks.into_iter() {
                let coordinates = Coordinates {
                    x: block.index.x as u16 + 4, // start at 5. x-square
                    y: block.index.y as u16,
                };
                if let Some(&Tile::Block(_)) = map.get(&coordinates) {
                    game_over_event_wtr.send(GameOverEvent);
                    return;
                }

                // info!("spawning block: {:?}", block);
                let entity = parent
                    .spawn()
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(block.color.r(), block.color.g(), block.color.b()),
                            custom_size: Some(Vec2::splat(board.tile_size)),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            (coordinates.x as f32 * board.tile_size) + (board.tile_size / 2.0),
                            board.bounds.size.y
                                - (coordinates.y as f32 * board.tile_size)
                                - 0.5 * board.tile_size,
                            2.0,
                        )),
                        ..Default::default()
                    })
                    .insert(CurrentTetromino {})
                    //TODO ShapePosition is already in the Tetromino. what is this then?
                    // .insert(ShapePosition {
                    //     x: block.1.index.x + 3,
                    //     y: board.bounds.size.y as u16 - tetromino_matrix_size + block.1.index.y,
                    // })
                    .insert(Name::new(format!("Block ({:?})", block.tetromino_type)))
                    .insert(coordinates.clone())
                    .insert(block.clone())
                    .id();

                let _tile = map.insert(
                    coordinates,
                    Tile::CurrentTetromino(shape_type.as_char()),
                    // Tile::CurrentTetromino(crate::resources::Color::from(block.color), entity.clone()),
                );

                entity_positions.insert(entity, block.index); // TODO use positions
            }
        });
        let shape = ShapeEntity {
            anker,
            positions: entity_positions,
            shape_type,
            layout,
            position_on_board: (4, 0).into(),
        };
        commands.insert_resource(shape);
    }
}
