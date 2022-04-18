use bevy::{log, math::Vec3Swizzles, prelude::*};

use crate::{
    bounds::Bounds2, Board, BoardAssets, BoardOptions, BoardPosition, Map, SpawnEvent, TileSize,
};

pub fn create_board(
    mut commands: Commands,
    board_options: Option<Res<BoardOptions>>,
    window: Res<WindowDescriptor>,
    board_assets: Res<BoardAssets>,
    mut spawn_ewr: EventWriter<SpawnEvent>,
) {
    let options = match board_options {
        None => BoardOptions::default(), // If no options is set we use the default one
        Some(o) => o.clone(),
    };

    // We define the size of our tiles in world space
    let tile_size = match options.tile_size {
        TileSize::Fixed(v) => v,
        TileSize::Adaptive { min, max } => crate::window::adaptative_tile_size(
            window,
            (min, max),
            (options.map_size.0, options.map_size.1),
        ),
    };
    log::info!("tile size is {}", tile_size);

    let board_size = Vec2::new(
        options.map_size.0 as f32 * tile_size,
        options.map_size.1 as f32 * tile_size,
    );
    log::info!("board size: {}", board_size);
    // We define the board anchor position (bottom left)
    let board_position = match options.position {
        BoardPosition::Centered { offset } => {
            Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
        }
        BoardPosition::Custom(p) => p,
    };

    let board_entity = commands
        .spawn()
        .insert(Name::new("Board"))
        .insert(Transform::from_translation(board_position))
        .insert(GlobalTransform::default())
        .with_children(|parent| {
            // We spawn the board background sprite at the center of the board, since the sprite pivot is centered
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.board_material.color,
                        custom_size: Some(board_size),
                        ..Default::default()
                    },
                    // This is the anchor of the sprite
                    texture: board_assets.board_material.texture.clone(),
                    transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                    ..Default::default()
                })
                .insert(Name::new("Background"))
                .with_children(|parent| {
                    spawn_tiles_at_background(
                        parent,
                        options.map_size.1,
                        options.map_size.0,
                        tile_size,
                        options.tile_padding,
                        &board_assets,
                        board_position.clone(),
                    );
                });
        })
        .id();

    let board_position = match options.position {
        BoardPosition::Centered { offset } => {
            Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
        }
        BoardPosition::Custom(p) => p,
    };

    commands.insert_resource(Board {
        map: Map::new(options.map_size.0 as usize, options.map_size.1 as usize),
        entity: board_entity,
        tile_size,
        bounds: Bounds2 {
            position: board_position.xy(),
            size: board_size,
        },
        current_tetromino_shape: None,
    });

    spawn_ewr.send(SpawnEvent);
}

fn spawn_tiles_at_background(
    parent: &mut ChildBuilder,
    height: u16,
    width: u16,
    tile_size: f32,
    padding: f32,
    board_assets: &BoardAssets,
    board_position: Vec3,
) {
    // Tiles
    for y in 0..height {
        for x in 0..width {
            // let coordinates = Coordinates {
            //     x: x as u16,
            //     y: y as u16,
            // };
            let mut cmd = parent.spawn();
            cmd.insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: board_assets.tile_material.color,
                    custom_size: Some(Vec2::splat(tile_size - padding)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    (x as f32 * tile_size) + (tile_size / 2.) + (board_position.x),
                    (y as f32 * tile_size) + (tile_size / 2.) + (board_position.y),
                    1.,
                ),
                texture: board_assets.tile_material.texture.clone(),
                ..Default::default()
            })
            .insert(Name::new(format!("Tile ({}, {})", x, y)));
            // .insert(coordinates);
        }
    }
}

fn spawn_tiles(
    parent: &mut ChildBuilder,
    height: u16,
    width: u16,
    tile_size: f32,
    padding: f32,
    board_assets: &BoardAssets,
) {
    // Tiles
    for y in 0..height {
        for x in 0..width {
            // let coordinates = Coordinates {
            //     x: x as u16,
            //     y: y as u16,
            // };
            let mut cmd = parent.spawn();
            cmd.insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: board_assets.tile_material.color,
                    custom_size: Some(Vec2::splat(tile_size - padding)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    (x as f32 * tile_size) + (tile_size / 2.),
                    (y as f32 * tile_size) + (tile_size / 2.),
                    2.,
                ),
                texture: board_assets.tile_material.texture.clone(),
                ..Default::default()
            })
            .insert(Name::new(format!("Tile ({}, {})", x, y)));
            // .insert(coordinates);
        }
    }
}

pub(crate) fn cleanup_board(board: Res<Board>, mut commands: Commands) {
    commands.entity(board.entity).despawn_recursive();
    commands.remove_resource::<Board>();
}
