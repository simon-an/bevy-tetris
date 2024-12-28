use crate::{
    queries, Board, Coordinates, CurrentTetromino, GameOverEvent, Map, NextShape, PreviewRef,
    Shape, ShapeType, SidebarRef, Tetromino, Tile, TileMapRoot,
};
use bevy::{
    color::palettes::css::{DARK_GRAY, DARK_GREEN},
    prelude::*,
};
use states::GameLogicState;

pub(crate) fn spawn_preview(
    mut commands: Commands,
    mut next_shape: ResMut<NextShape>,
    query: Query<queries::PreviewQuery>,
    sidebar: Query<Entity, With<SidebarRef>>,
) {
    let selected_shape_type: ShapeType = rand::random(); //TODO Get shape type from queue
    next_shape.0 = selected_shape_type;

    if let Ok(item) = query.get_single() {
        commands.entity(item.entity).despawn_recursive();
    }

    commands.entity(sidebar.single()).with_children(|parent| {
        let mut parent = parent.spawn((
            (PreviewRef),
            Name::new("Preview"),
            Node {
                display: Display::Grid,
                position_type: PositionType::Relative,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                grid_template_columns: [
                    GridTrack::flex(30.0),
                    GridTrack::flex(30.0),
                    GridTrack::flex(30.0),
                    GridTrack::flex(30.0),
                ]
                .into(),
                grid_template_rows: [
                    GridTrack::flex(30.0),
                    GridTrack::flex(30.0),
                    GridTrack::flex(30.0),
                    GridTrack::flex(30.0),
                ]
                .into(),
                ..Default::default()
            },
            ZIndex(500),
            BackgroundColor(DARK_GREEN.into()),
        ));
        let blocks = Tetromino::blocks_from_type(selected_shape_type);
        for col in 1..5 {
            for row in 1..5 {
                let block = blocks
                    .iter()
                    .find(|block| block.index.x + 1 == col && block.index.y + 1 == row);
                let color = if let Some(block) = block {
                    block.color
                } else {
                    DARK_GRAY.into()
                };
                let _entity = parent
                    .with_child((
                        Node {
                            grid_row: GridPlacement::start(row),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            padding: UiRect::all(Val::Px(1.0)),
                            margin: UiRect::all(Val::Px(1.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            grid_column: GridPlacement::start(col),
                            ..Default::default()
                        },
                        Name::new(format!("Tile ({col}{row}) ({:?})", selected_shape_type)),
                        BackgroundColor(color),
                        ZIndex(501),
                    ))
                    .id();
            }
        }
        // let Shape {
        //     anker,
        //     layout,
        //     shape_type,
        //     ..
        // } = Shape::blueprint(selected_shape_type);
        // let blocks = Tetromino::blocks_from_type(selected_shape_type);
        // assert_eq!(selected_shape_type, shape_type);

        // for block in blocks.into_iter() {
        //     let coordinates = Coordinates {
        //         x: block.index.x as u16 + 4, // start at 5. x-square
        //         y: block.index.y as u16,
        //     };

        //     let _entity = parent
        //         .spawn((
        //             Sprite {
        //                 color: block.color.clone(),
        //                 custom_size: Some(Vec2::splat(30.0)),
        //                 ..Default::default()
        //             },
        //             Transform::from_translation(Vec3::new(
        //                 (coordinates.x as f32 * 30.0) + (30.0 / 2.0),
        //                 30.0 - (coordinates.y as f32 * 30.0) - 0.5 * 30.0,
        //                 100.0,
        //             )),
        //             Name::new(format!("Block ({:?})", block.tetromino_type)),
        //             coordinates.clone(),
        //             block.index.clone(),
        //             block.clone(),
        //         ))
        //         .id();
        // }
    });
}

pub(crate) fn spawn_tetromino(
    mut commands: Commands,
    board: Res<Board>,
    mut map: ResMut<Map>,
    // board_assets: Res<BoardAssets>,
    mut game_over_event_wtr: EventWriter<GameOverEvent>,
    mut logic_state: ResMut<NextState<GameLogicState>>,
    next_shape: Res<NextShape>,
    root: Query<Entity, With<TileMapRoot>>,
) {
    // for event in spawn_event_rdr.read() {
    info!("Spawning");
    let selected_shape_type: ShapeType = next_shape.0; //TODO Get shape type from queue
    let Shape {
        anker,
        layout,
        shape_type,
        ..
    } = Shape::blueprint(selected_shape_type);
    let blocks = Tetromino::blocks_from_type(selected_shape_type);
    assert_eq!(selected_shape_type, shape_type);

    commands
        .entity(root.get_single().expect("TileMapRoot must exist"))
        .with_children(|parent| {
            // TODO Iterate over blueprint.positions instead
            for block in blocks.into_iter() {
                let coordinates = Coordinates {
                    x: block.index.x as u16 + 6, // start at n. x-square
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
                        board.calc_transform(&coordinates),
                        CurrentTetromino {},
                        Name::new(format!("Current ({:?})", block.tetromino_type)),
                        coordinates.clone(),
                        block.index.clone(),
                        block.clone(),
                    ))
                    .id();

                let _tile = map.insert(coordinates, Tile::CurrentTetromino(shape_type.as_char()));
            }
        });
    logic_state.set(GameLogicState::Ticking);
}
