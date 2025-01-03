use bevy::color::palettes::css::{BLUE, GRAY, GREEN, RED};
use bevy::window::{PrimaryWindow, Window};
use bevy::{log, math::Vec3Swizzles, prelude::*};

use crate::{
    bounds::Bounds2, Board, BoardAssets, BoardOptions, BoardPosition, Map, SpawnEvent, TileSize,
};
use crate::{Score, TileMapRoot};

pub fn update_board_position(board: ResMut<Board>, window: Query<&Window, With<PrimaryWindow>>) {
    let windows = window.single().size();
    println!("windowsize {:?}", windows);
    // board.bounds.size = Vec2::new(windows.width(), windows.height);
}

pub fn create_board(
    mut commands: Commands,
    options: Res<BoardOptions>,
    window: Query<&Window, With<PrimaryWindow>>,
    board_assets: Res<BoardAssets>,
    mut spawn_ewr: EventWriter<SpawnEvent>,
) {
    println!("windows {:?}", window);

    // We define the size of our tiles in world space
    let tile_size = match options.tile_size {
        TileSize::Fixed(v) => v,
        TileSize::Adaptive { min, max } => {
            crate::window::adaptative_tile_size(window.single(), (min, max), &options)
        }
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
        .spawn((
            Name::new("Board"),
            Transform::default(),
            Sprite {
                // image: board_assets.board_material.texture.clone(),
                color: GREEN.into(),
                custom_size: Some(board_size),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("TileMap"),
                TileMapRoot,
                Transform::default(),
                Sprite {
                    // image: board_assets.board_material.texture.clone(),
                    color: BLUE.into(),
                    custom_size: Some(board_size),
                    ..Default::default()
                },
            ));
            // We spawn the board background sprite at the center of the board, since the sprite pivot is centered
            parent
                .spawn((
                    Sprite {
                        // image: board_assets.board_material.texture.clone(),
                        color: board_assets.board_material.color,
                        custom_size: Some(board_size),
                        ..Default::default()
                    },
                    // This is the anchor of the sprite
                    // Transform::from_xyz(board_size.x / 2., board_size.y / 2., -10.),
                    Transform::from_xyz(0., 0., -10.),
                    Name::new("Background"),
                ))
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
                    // spawn_tiles(
                    //     parent,
                    //     options.map_size.1,
                    //     options.map_size.0,
                    //     tile_size,
                    //     options.tile_padding,
                    //     &board_assets,
                    //     board_position.clone(),
                    // );
                });
        })
        .id();

    commands.insert_resource(Board {
        entity: board_entity,
        tile_size,
        bounds: Bounds2 {
            position: board_position.xy(),
            size: board_size,
        },
    });

    commands.insert_resource(Map::new(
        options.map_size.0 as usize,
        options.map_size.1 as usize,
    ));
    commands.insert_resource(Score(0));

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
            parent.spawn((
                Sprite {
                    color: GRAY.into(),
                    // color: board_assets.tile_material.color,
                    // image: board_assets.tile_material.texture.clone(),
                    custom_size: Some(Vec2::splat(tile_size - padding)),
                    ..Default::default()
                },
                Transform::from_xyz(
                    (x as f32 * tile_size) + (tile_size / 2.) + (board_position.x),
                    (y as f32 * tile_size) + (tile_size / 2.) + (board_position.y),
                    20.,
                ),
                Name::new(format!("Tile ({}, {})", x, y)),
            ));
        }
    }
}

pub(crate) fn cleanup_board(board: Res<Board>, mut commands: Commands) {
    commands.entity(board.entity).despawn_recursive();
    commands.remove_resource::<Board>();
}
