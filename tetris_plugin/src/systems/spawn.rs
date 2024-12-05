use crate::{
    Board, Coordinates, CurrentTetromino, GameOverEvent, Map, Shape, ShapeEntity, ShapeType,
    SpawnEvent, Tetromino, Tile,
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
    for event in spawn_event_rdr.read() {
        info!("EVENT {:?}", event);
        let selected_shape_type: ShapeType = rand::random(); //TODO Get shape type from queue
        let Shape {
            anker,
            layout,
            shape_type,
            ..
        } = Shape::blueprint(selected_shape_type);
        let blocks = Tetromino::blocks_from_type(selected_shape_type);
        assert_eq!(selected_shape_type, shape_type);

        commands.entity(board.entity).with_children(|parent| {
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
                let _entity = parent
                    .spawn((
                        Sprite {
                            color: block.color.clone(),
                            custom_size: Some(Vec2::splat(board.tile_size)),
                            ..Default::default()
                        },
                        Transform::from_translation(Vec3::new(
                            (coordinates.x as f32 * board.tile_size) + (board.tile_size / 2.0),
                            board.bounds.size.y
                                - (coordinates.y as f32 * board.tile_size)
                                - 0.5 * board.tile_size,
                            2.0,
                        )),
                        CurrentTetromino {},
                        Name::new(format!("Block ({:?})", block.tetromino_type)),
                        coordinates.clone(),
                        block.index.clone(),
                        block.clone(),
                    ))
                    //TODO ShapePosition is already in the Tetromino. what is this then?
                    // .insert(ShapePosition {
                    //     x: block.1.index.x + 3,
                    //     y: board.bounds.size.y as u16 - tetromino_matrix_size + block.1.index.y,
                    // })
                    .id();

                let _tile = map.insert(
                    coordinates,
                    Tile::CurrentTetromino(shape_type.as_char()),
                    // Tile::CurrentTetromino(crate::resources::Color::from(block.color), entity.clone()),
                );
            }
        });
        let shape = ShapeEntity {
            anker,
            shape_type,
            layout,
            position_on_board: (4, 0).into(),
        };
        commands.insert_resource(shape);
    }
}
