use bevy::prelude::*;

use crate::{Board, Coordinates, CurrentTetromino, Map, RotateEvent, ShapeEntity, Tetromino, Tile};

pub(crate) fn update_block_sprites(
    transform: &mut Mut<Transform>,
    coordinates: &Coordinates,
    board: &ResMut<Board>,
) {
    let (new_x, new_y): (f32, f32) = board.calc_translation(coordinates);

    let translation = &mut transform.translation;
    // println!("translation: {translation}");
    translation.x = new_x;
    translation.y = new_y;
    // println!("new translation: {translation}");
}

pub(crate) fn rotate(
    _commands: Commands,
    board: ResMut<Board>,
    mut map: ResMut<Map>,
    shape_entity: Option<ResMut<ShapeEntity>>,
    mut rotate_event_rdr: EventReader<RotateEvent>,
    mut current_query: Query<(Entity, &mut Tetromino, &CurrentTetromino, &mut Transform)>,
) {
    if let Some(mut shape_entity) = shape_entity {
        for event in rotate_event_rdr.iter() {
            let all_clear = map.is_free(&event); // TODO does not prevent panic in line 42
            if all_clear {
                let mut changes = vec![];
                for (entity, _, _current, mut transform) in current_query.iter_mut() {
                    debug!("entity {:?}", entity);
                    let target: Option<(Coordinates, Tile)> =
                        rotate_block(&entity, &event, &mut shape_entity, &mut map);
                    info!("rotation result {:?}", target);
                    if let Some((coordinate, tile)) = target {
                        update_block_sprites(&mut transform, &coordinate, &board);
                        changes.push((coordinate, tile));
                    }
                }

                // insert tile after ALL previous one have been set to empty
                changes.into_iter().for_each(|(coordinates, tile)| {
                    let mustbe_empty_tile = map.insert(coordinates, tile).unwrap();
                    assert_eq!(mustbe_empty_tile, Tile::Empty); // TODO this can be a collision with an existing block
                });
            } else {
                info!("board is not free for rotation");
                return;
            }
            #[cfg(feature = "debug")]
            bevy::log::info!("{}", (*board).console_output());
        }
    }
}

pub(crate) fn rotate_block(
    e: &Entity,
    event: &RotateEvent,
    shape: &mut ShapeEntity,
    map: &mut Map,
) -> Option<(Coordinates, Tile)> {
    let mut coords = shape.shapepos_as_coords(shape.positions.get(e).unwrap());

    match shape.shape_type {
        crate::ShapeType::O => return None,
        crate::ShapeType::I => return None, // TODO
        _ => (),
    }

    println!("rotating entity: {:?}", e);
    let pos = shape.positions.get_mut(e).expect("Entity must be known");

    let orig_x = pos.x;
    let orig_y = pos.y;
    println!("index {:?}", pos);
    if orig_x == 1 || orig_y == 1 {
        if let &RotateEvent::ClockWise = event {
            pos.x = match orig_x {
                0 => 1,
                1 if orig_y == 0 => 2,
                1 if orig_y == 2 => 0,
                2 => 1,
                _ => 1,
            };
            pos.y = match orig_y {
                0 => 1,
                1 if orig_x == 0 => 0,
                1 if orig_x == 2 => 2,
                2 => 1,
                _ => 1,
            };
        } else {
            pos.x = match orig_x {
                0 => 1,
                1 if orig_y == 0 => 0,
                1 if orig_y == 2 => 2,
                2 => 1,
                _ => 1,
            };
            pos.y = match orig_y {
                0 => 1,
                1 if orig_x == 0 => 2,
                1 if orig_x == 2 => 0,
                2 => 1,
                _ => 1,
            };
        }
    } else {
        if let &RotateEvent::ClockWise = event {
            let (x, y) = match (orig_x, orig_y) {
                (0, 0) => (2, 0),
                (2, 0) => (2, 2),
                (2, 2) => (0, 2),
                (0, 2) => (0, 0),
                _ => (1, 1),
            };
            pos.x = x;
            pos.y = y;
        } else {
            let (x, y) = match (orig_x, orig_y) {
                (0, 0) => (0, 2),
                (0, 2) => (2, 2),
                (2, 2) => (2, 0),
                (2, 0) => (0, 0),
                _ => (1, 1),
            };
            pos.x = x;
            pos.y = y;
        }
    }

    println!("new index {:?}", pos);

    let delta_x: i16 = pos.x as i16 - orig_x as i16;
    let delta_y: i16 = pos.y as i16 - orig_y as i16;

    println!("new index {:?}. delta_x {delta_x}, delta_y {delta_y}", pos);

    if delta_x.is_negative() {
        coords.x -= (-delta_x) as u16;
    } else {
        coords.x += delta_x as u16;
    }
    if delta_y.is_negative() {
        coords.y -= (-delta_y) as u16;
    } else {
        coords.y += delta_y as u16;
    }

    // if delta_x == 0 && delta_y == 0 {
    //     None
    // } else {
    let tile = map
        .insert(
            shape.position_on_board.clone() + (orig_x, orig_y),
            Tile::Empty,
        )
        .unwrap();
    Some((coords, tile))
    // }
}

#[cfg(test)]
mod rotate_tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::{Map, Matrix, Shape, ShapePosition, ShapeType, TileBlueprint, ToMap};
    use bevy::prelude::Entity;
    use bevy::winit::WinitPlugin;
    use bevy::{render::settings::WgpuSettings, DefaultPlugins};
    use bevy_egui::EguiPlugin;

    #[test]
    fn headless_mode() {
        App::new()
            .insert_resource(WgpuSettings {
                backends: None,
                ..Default::default()
            })
            .add_plugins_with(DefaultPlugins, |group| group.disable::<WinitPlugin>())
            .add_plugin(EguiPlugin)
            .update();
    }

    fn test_rotate(
        input: Vec<&str>,
        res: Vec<&str>,
        position_on_board: Coordinates,
        anker: Coordinates,
        shape_type: ShapeType,
        direction: RotateEvent,
    ) {
        let mut world = World::new();
        let entity: Entity = world.spawn().id();
        let e1: Entity = world.spawn().id();
        let e2: Entity = world.spawn().id();
        let e3: Entity = world.spawn().id();
        let e4: Entity = world.spawn().id();

        let shape = Shape::blueprint(shape_type);

        let blocks = vec![e1, e2, e3, e4];
        let positions = shape
            .clone()
            .positions
            .into_iter()
            .filter(|(_, b)| b == &TileBlueprint::CurrentTetromino)
            .enumerate()
            .map(|(i, (k, _))| (blocks[i], k))
            .collect::<BTreeMap<Entity, ShapePosition>>();
        let _entities = shape
            .clone()
            .positions
            .into_iter()
            .filter(|(_, b)| b == &TileBlueprint::CurrentTetromino)
            .enumerate()
            .map(|(i, (k, _))| (k, blocks[i]))
            .collect::<BTreeMap<ShapePosition, Entity>>();

        let mut map = Map::new(5, 5);
        map.spawn(&shape, &(1, 1).into());
        let input_map = input.to_map();
        assert_eq!(input_map, map);

        let res_coords: Map = res.to_map();
        println!("res_coords: {}", res_coords);
        let target = res_coords.get_current_shape_tile_coordinates();
        let current = map.get_current_shape_tile_coordinates();
        println!("target: {:?}", target);
        println!("current: {:?}", current);
        let mut coords = vec![];
        let mut shape_entity = ShapeEntity::spawn(shape, &position_on_board, &mut world);
        for i in 0..4 {
            let block = blocks.get(i).unwrap();
            if let Some((c1, t)) = rotate_block(block, &direction, &mut shape_entity, &mut map) {
                println!("new coords: {} {}", c1, t);
                // assert_eq!(
                //     t,
                //     // Tile::CurrentTetromino(ShapeType::get_color(&shape_type).into(), e1)
                //     Tile::CurrentTetromino
                // );
                coords.push(c1);
            } else {
                let empty = positions.get(block).unwrap();
                eprintln!("empty {:?}", empty)
                //     let one_one_expected: ShapePosition = (1, 1).into();
                //     assert_eq!(one_one, &one_one_expected);
                //     coords.push((2, 2).into());
            }
            // assert!(target.contains(&c1));
        }
        coords.sort();
        assert_eq!(coords, target);
        println!("rotate coords: {:?}", coords);
        println!("map: {}", map);
        assert!(map.is_empty());
        for coordinates in coords {
            map.insert(coordinates, Tile::CurrentTetromino(shape_type.as_char()));
        }
        // let current = uut.get_current_shape_coordinates(); // includes empty coordinates
        let current = map.get_current_shape_tile_coordinates();
        assert_eq!(target, current);
    }

    #[test]
    fn test_rotate_s_shape_cw() {
        let input = vec!["xxxxx", "xxSSx", "xSSxx", "xxxxx", "xxxxx"];
        let res = vec!["xxxxx", "xxSxx", "xxSSx", "xxxSx", "xxxxx"];
        let position_on_board = Coordinates { x: 1, y: 1 };
        let anker = Coordinates { x: 0, y: 0 };
        test_rotate(
            input,
            res,
            position_on_board,
            anker,
            ShapeType::S,
            RotateEvent::ClockWise,
        );
    }
    #[test]
    fn test_rotate_s_shape_ccw() {
        let input = vec!["xxxxx", "xxSSx", "xSSxx", "xxxxx", "xxxxx"];
        let res = vec!["xxxxx", "xSxxx", "xSSxx", "xxSxx", "xxxxx"];
        let position_on_board = Coordinates { x: 1, y: 1 };
        let anker = Coordinates { x: 0, y: 0 };
        test_rotate(
            input,
            res,
            position_on_board,
            anker,
            ShapeType::S,
            RotateEvent::CounterClockWise,
        );
    }

    #[test]
    fn test_rotate_z_shape_cw() {
        let input = vec!["xxxxx", "xZZxx", "xxZZx", "xxxxx", "xxxxx"];
        let res = vec!["xxxxx", "xxxZx", "xxZZx", "xxZxx", "xxxxx"];

        let position_on_board = Coordinates { x: 1, y: 1 };
        let anker = Coordinates { x: 0, y: 0 };
        test_rotate(
            input,
            res,
            position_on_board,
            anker,
            ShapeType::Z,
            RotateEvent::ClockWise,
        );
    }
    #[test]
    fn test_rotate_z_shape_ccw() {
        let input = vec!["xxxxx", "xZZxx", "xxZZx", "xxxxx", "xxxxx"];
        let res = vec!["xxxxx", "xxZxx", "xZZxx", "xZxxx", "xxxxx"];

        let position_on_board = Coordinates { x: 1, y: 1 };
        let anker = Coordinates { x: 0, y: 0 };
        test_rotate(
            input,
            res,
            position_on_board,
            anker,
            ShapeType::Z,
            RotateEvent::CounterClockWise,
        );
    }
    #[test]
    fn test_rotate_t_shape_cw() {
        let input = vec!["xxxxx", "xTxxx", "xTTxx", "xTxxx", "xxxxx"];
        let res = vec!["xxxxx", "xTTTx", "xxTxx", "xxxxx", "xxxxx"];

        let position_on_board = Coordinates { x: 1, y: 1 };
        let anker = Coordinates { x: 0, y: 0 };
        test_rotate(
            input,
            res,
            position_on_board,
            anker,
            ShapeType::T,
            RotateEvent::ClockWise,
        );
    }
    #[test]
    fn test_rotate_t_shape_ccw() {
        let input = vec!["xxxxx", "xTxxx", "xTTxx", "xTxxx", "xxxxx"];
        let res = vec!["xxxxx", "xxxxx", "xxTxx", "xTTTx", "xxxxx"];
        let position_on_board = Coordinates { x: 1, y: 1 };
        let anker = Coordinates { x: 0, y: 0 };
        test_rotate(
            input,
            res,
            position_on_board,
            anker,
            ShapeType::T,
            RotateEvent::CounterClockWise,
        );
    }
    #[test]
    fn test_is_free() {
        let mut world = World::new();
        let entity: Entity = world.spawn().id();
        let shape = Shape::blueprint(ShapeType::L);

        let position_on_board = Coordinates { x: 1, y: 1 };
        let mut uut = Map::new(5, 5);
        uut.spawn(&shape, &position_on_board);
        let _shapee = ShapeEntity::spawn(shape, &position_on_board, &mut world);
        assert!(uut.is_free(&RotateEvent::ClockWise));
    }
}
