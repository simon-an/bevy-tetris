use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

use bevy::prelude::*;

use crate::{Board, GameCommand, Map, Transitions};

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
    // mut shape: Option<Res<ShapeEntity>>,
    transitions: Option<Res<Transitions>>,
    mut game_command: EventReader<crate::GameCommand>,
    // mut state: ResMut<State<T>>,
    // pause_state: ResMut<T>,
) {
    let file = "save.txt";
    for event in game_command.read() {
        // state.set(pause_state.clone()).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .append(false)
            .open(file)
            .expect("open save.txt failed");
        if event == &GameCommand::Save {
            if transitions.is_none() {
                file.write_all(map.as_savegame_string().as_bytes())
                    .expect("writing to file failed");
            } else {
                warn!("cannot save while transitions is going on");
            }
        } else if event == &GameCommand::Load {
            // TODO DESPAWN OLD STUFF
            commands.entity(board.entity).despawn_descendants();

            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).expect("read file must work");
            let res = String::from_utf8(buffer).unwrap();
            println!("map loaded {res}");
            let map = Map::from_str(&res);
            println!("map loaded {}", map.to_string());

            let shape_coordinates = map.get_current_shape_coordinates();
            commands.insert_resource(map);

            // let mut positions = vec![];
            //  ShapePosition {
            //     x: (coordinates.x - pos.x) as i16,
            //     y: (coordinates.y - pos.y) as i16,
            // },
            // commands.insert_resource(ShapeEntity {

            // });
        }
    }
}
