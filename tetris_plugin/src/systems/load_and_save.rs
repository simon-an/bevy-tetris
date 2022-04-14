use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

use bevy::{ecs::schedule::StateData, prelude::*};

use crate::{components::CurrentTetromino, Board, GameCommand, Map, ShapePosition};

pub(crate) fn load_and_save<T>(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut game_command: EventReader<crate::GameCommand>,
    // mut state: ResMut<State<T>>,
    // pause_state: ResMut<T>,
) where
    T: StateData + Send + Sync,
{
    let file = "save.txt";
    for event in game_command.iter() {
        // state.set(pause_state.clone()).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .append(false)
            .open(file)
            .expect("open save.txt failed");
        if event == &GameCommand::Save {
            if board.map.transitions.is_none() {
                file.write_all(board.map.as_savegame_string().as_bytes())
                    .expect("writing to file failed");
            }else {
                warn!("cannot save while transitions is going on");
            }
        } else if event == &GameCommand::Load {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).expect("read file must work");
            let res = String::from_utf8(buffer).unwrap();
            println!("map loaded {res}");
            let map = Map::from_str(&res);
            println!("map loaded {}", map.to_string());

            // TODO DESPAWN OLD STUFF
            // commands.entity(board.entity).commands().remove_resource()

            let shape = map.get_current_shape_coordinates();

            board.set_map(map);

            let mut positions = vec![];

            let color = board
                .current_tetromino_shape
                .as_ref()
                .unwrap()
                .shape_type
                .get_color();

            // SPAWN NEW STUFF
            for coordinates in shape {
                let entity = commands
                    .spawn()
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: color,
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
                    // .insert(Name::new(format!("Block ({:?})", block.tetromino_type)))
                    .insert(coordinates.clone())
                    .id();

                let pos = board
                    .current_tetromino_shape
                    .as_ref()
                    .unwrap()
                    .position_on_board;
                positions.push((
                    entity,
                    ShapePosition {
                        x: (coordinates.x - pos.x) as i16,
                        y: (coordinates.y - pos.y) as i16,
                    },
                ));
            }

            board.set_positions(positions);

            // TODO SPAWN BLOCKS
        }
    }
}
