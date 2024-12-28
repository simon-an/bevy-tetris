use bevy::{prelude::*, utils::info};
use bevy_pkv::PkvStore;

use crate::{
    Board, CurrentTetromino, GameCommand, Map, ShapePosition, ShapeType, Tetromino, Tile,
    TileMapRoot, Transitions,
};

pub(crate) fn load_and_save_warning(mut game_command: EventReader<crate::GameCommand>) {
    for event in game_command.read() {
        if event == &GameCommand::Save || event == &GameCommand::Load {
            warn!("load and save is only active when game is paused");
        }
    }
}
pub(crate) fn load_and_save(
    mut commands: Commands,
    board: Res<Board>,
    map: Res<Map>,
    transitions: Option<Res<Transitions>>,
    mut game_command: EventReader<crate::GameCommand>,
    root: Query<Entity, With<TileMapRoot>>,
    mut pkv: ResMut<PkvStore>,
) {
    for event in game_command.read() {
        info!("load_and_save {:?}", event);
        // state.set(pause_state.clone()).unwrap();
        if event == &GameCommand::Save {
            info("save");
            if transitions.is_none() {
                pkv.set_string("savegame", map.as_savegame_string().as_str())
                    .expect("failed to store username");
            } else {
                warn!("cannot save while transitions is going on");
            }
        } else if event == &GameCommand::Load {
            info("load");
            // TODO DESPAWN OLD STUFF
            commands.entity(root.single()).despawn_descendants();

            if let Ok(res) = pkv.get::<String>("savegame") {
                info!("map loaded {res}");
                let map = Map::from_str(&res);

                #[cfg(feature = "debug")]
                bevy::log::info!("{}", map);

                commands.entity(root.single()).with_children(|parent| {
                    for (coordinates, tile) in map
                        .inner
                        .iter()
                        .filter(|(_, tile)| tile.is_block() || tile.is_moveable())
                    {
                        let shape = if let Tile::CurrentTetromino(c) = tile {
                            ShapeType::from_char(*c)
                        } else if let Tile::Block(c) = tile {
                            ShapeType::from_char(*c)
                        } else {
                            panic!("unexpected tile type");
                        };
                        let mut entity = parent.spawn((
                            Sprite {
                                color: shape.get_color(),
                                custom_size: Some(Vec2::splat(board.tile_size)),
                                ..Default::default()
                            },
                            board.calc_transform(&coordinates),
                            Name::new(format!("Block ({:?})", shape)),
                            coordinates.clone(),
                            ShapePosition {
                                x: coordinates.x as i16,
                                y: coordinates.y as i16,
                            },
                            Tetromino {
                                color: shape.get_color(),
                                tetromino_type: shape,
                                index: ShapePosition {
                                    x: coordinates.x as i16,
                                    y: coordinates.y as i16,
                                },
                            },
                        ));
                        if let Tile::CurrentTetromino(_) = tile {
                            entity.insert(CurrentTetromino {});
                        }
                    }
                });

                commands.insert_resource(map);
            } else {
                error!("savegame not found");
            }
        }
    }
}
