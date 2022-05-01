use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::*;
use std::collections::BTreeMap;
use std::time::Duration;

use crate::Bounds2;
use crate::Coordinates;
use crate::Map;
use crate::MoveEvent;
use crate::RotateEvent;
use crate::ShapeEntity;
use crate::ShapePosition;
use crate::ShapeType;
use crate::Tile;
use bevy::prelude::*;

#[derive(Debug)]
pub(crate) struct Board {
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub map: Map,
    pub current_tetromino_shape: Option<ShapeEntity>,
    pub entity: Entity,
}

impl Board {
    pub fn animate(
        &mut self,
        map: BTreeMap<&Coordinates, (&Transform, Entity)>,
        mut commands: Commands,
    ) {
        if let Some(ts) = self.map.transitions.take() {
            for (from, to) in ts {
                // let gui_pos = self.calc_translation(&from);
                if let Some((t, e)) = map.get(&from) {
                    let (new_x, new_y): (f32, f32) = self.calc_translation(&(to.x, to.y).into());
                    // println!(
                    //     "current translation: {:?}, new pos {:?}",
                    //     gui_pos,
                    //     (new_x, new_y)
                    // );

                    let tween = Tween::new(
                        // Use a quadratic easing on both endpoints.
                        EaseFunction::QuadraticInOut,
                        // Loop animation back and forth.
                        TweeningType::Once,
                        // Animation time (one way only; for ping-pong it takes 2 seconds
                        // to come back to start).
                        Duration::from_secs(1),
                        // The lens gives access to the Transform component of the Entity,
                        // for the Animator to animate it. It also contains the start and
                        // end values respectively associated with the progress ratios 0. and 1.
                        TransformPositionLens {
                            start: t.translation,
                            end: Vec3::new(new_x, new_y, 5.),
                        },
                    );

                    commands
                        .entity(*e)
                        // Add an Animator component to control and execute the animation.
                        .insert(Animator::new(tween));
                } else {
                    panic!("from coordinates not found {}", from)
                }
            }
        }
    }

    // TODO TEST THIS
    pub fn set_map(&mut self, map: Map) {
        self.current_tetromino_shape = Some(map.try_parse_shape().expect("shape must be parsable"));
        self.map = map;
    }
    pub fn set_positions(&mut self, pos: Vec<(Entity, ShapePosition)>) {
        match self.current_tetromino_shape.as_mut() {
            Some(v) => (*v)
                .positions
                .append(&mut pos.into_iter().collect::<BTreeMap<Entity, ShapePosition>>()),
            None => {}
        }
    }

    pub fn get_color(shape: ShapeType) -> Color {
        // make customizeable
        match shape {
            ShapeType::I => Color::rgb(0.0, 0.7, 0.7),
            ShapeType::O => Color::rgb(0.7, 0.7, 0.0), // square, yellow
            ShapeType::T => Color::rgb(0.7, 0.0, 0.7), // T, purple
            ShapeType::Z => Color::rgb(0.7, 0.0, 0.0), // Z, red
            ShapeType::S => Color::rgb(0.0, 0.7, 0.0), // S, green
            ShapeType::L => Color::rgb(0.0, 0.0, 0.7), // L, blue
            ShapeType::J => Color::rgb(0.9, 0.25, 0.0), // J, orange
        }
    }

    pub fn calc_translation(&self, coordinates: &Coordinates) -> (f32, f32) {
        let new_x: f32 = (coordinates.x as f32 * self.tile_size) + (self.tile_size / 2.0);
        let new_y: f32 =
            self.bounds.size.y - (coordinates.y as f32 * self.tile_size) - 0.5 * self.tile_size;

        (new_x, new_y)
    }
    pub(crate) fn get_current_shape_coordinates(&self) -> Vec<Coordinates> {
        self.map.get_current_shape_coordinates()
    }
    pub(crate) fn get_current_shape_tile_coordinates(&self) -> Vec<Coordinates> {
        self.map.get_current_shape_tile_coordinates()
    }

    pub fn rotate_block(&mut self, e: &Entity, event: &RotateEvent) -> Option<(Coordinates, Tile)> {
        let mut coords = self.shapepos_as_coords(
            self.current_tetromino_shape
                .as_ref()
                .unwrap()
                .positions
                .get(e)
                .unwrap(),
        );

        if let Some(ShapeEntity {
            // anker,
            position_on_board,
            positions,
            shape_type,
            layout,
            ..
        }) = &mut self.current_tetromino_shape
        {
            match shape_type {
                crate::ShapeType::O => return None,
                crate::ShapeType::I => return None, // TODO
                _ => (),
            }

            println!("rotating entity: {:?}", e);
            let pos = positions.get_mut(e).expect("Entity must be known");

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
            let tile = self
                .map
                .insert(position_on_board.clone() + (orig_x, orig_y), Tile::Empty)
                .unwrap();
            Some((coords, tile))
            // }
        } else {
            None
        }
    }
    pub fn move_shape(&mut self, event: &MoveEvent) {
        // if let Some(ShapeEntity {
        //     mut position_on_board,
        //     ..
        // }) = &mut self.current_tetromino_shape
        if let Some(shape) = &mut self.current_tetromino_shape {
            let old = shape.position_on_board.clone();
            match event {
                &MoveEvent::Down => {
                    shape.position_on_board.y += 1;
                }
                &MoveEvent::Left => {
                    if shape.position_on_board.x > 0 {
                        shape.position_on_board.x -= 1;
                    } else {
                        warn!("cannot move left");
                    }
                }
                &MoveEvent::Right => {
                    shape.position_on_board.x += 1;
                }
            };
            trace!(
                "shape has moved from {} to {}",
                old,
                shape.position_on_board
            );
        } else {
            error!("shape does not exist; so no shape has moved");
        }
    }
    pub fn move_block(
        &mut self,
        e: &Entity,
        event: &MoveEvent,
    ) -> Result<Option<(Coordinates, Tile)>, &str> {
        if let Some(ShapeEntity {
            anker,
            position_on_board,
            positions,
            ..
        }) = &mut self.current_tetromino_shape
        {
            debug!("move entity: {:?}", e);
            let pos = positions.get(e);
            if pos.is_none() {
                return Err("enitity not found");
            }
            let pos = pos.unwrap();
            let mut coords = position_on_board.clone() - anker.clone() + pos.clone();
            let tile = self.map.insert(coords, Tile::Empty);
            if tile.is_none() {
                return Err("tile not found");
            }
            let tile = tile.unwrap();

            match event {
                &MoveEvent::Down => {
                    coords.y += 1;
                }
                &MoveEvent::Left => {
                    coords.x -= 1;
                }
                &MoveEvent::Right => {
                    coords.x += 1;
                }
            };
            Ok(Some((coords, tile)))
        } else {
            Ok(None)
        }
    }

    pub fn get_shape_positions(&self) -> Option<Vec<&ShapePosition>> {
        self.current_tetromino_shape
            .as_ref()
            .map(|s| s.positions.values().collect())
    }

    /// only call this if board has a shape
    pub fn is_free(&self, direction: &RotateEvent) -> bool {
        // if let Some(shape) = &self.current_tetromino_shape {
        //     enum Check {
        //         W,
        //         S,
        //         O,
        //         N,
        //         Inner,
        //     }
        //     let target = if shape.layout.width == shape.layout.height {
        //         (Check::Inner, None, 0)
        //     } else if shape.layout.width > shape.layout.height {
        //         if direction == RotateEvent::ClockWise {
        //             // xxxx      xoox
        //             // xooo =>   xoox
        //             // xooo      xoox
        //             (Check::N, None, shape.layout.width - shape.layout.height)
        //         } else {
        //             // xooo =>   ooxx
        //             // xooo      ooxx
        //             // xxxx      ooxx
        //             (
        //                 Check::S,
        //                 Some(Check::W),
        //                 shape.layout.width - shape.layout.height,
        //             )
        //         }
        //     } else {
        //         if direction == RotateEvent::ClockWise {
        //             // xxoo      ooox
        //             // xxoo =>   ooox
        //             // xxoo      xxxx
        //             (Check::W, None, shape.layout.height - shape.layout.width)
        //         } else {
        //             // xoox      xooo
        //             // xoox =>   xooo
        //             // xoox      xxxx
        //             (Check::O, None, shape.layout.height - shape.layout.width)
        //         }
        //     };

        //     match target {
        //         (Check::Inner, _, _) => {
        //             for c in self.get_current_shape_coordinates() {
        //                 if let Some(x) = self.map.get(&(*c + shape.anker + shape.position_on_board))
        //                 {
        //                     if let Tile::Block = x {
        //                         return false; // Collision
        //                     } else {
        //                         continue;
        //                     }
        //                 } else {
        //                     return false; // out of bounds
        //                 }
        //             }
        //         }
        //         (compass, _, diff) => {
        //             let count = diff
        //                 * match compass {
        //                     Check::N => -1 ,
        //                     Check::O,
        //                     _ => 0
        //                 };
        //             for c in 0..count {
        //                 if let Some(t) = self.map.get(&(*c + shape.anker + shape.position_on_board))
        //                 {
        //                     if let Tile::Block = t {
        //                         return false; // Collision
        //                     } else {
        //                         continue;
        //                     }
        //                 } else {
        //                     return false; // out of bounds
        //                 }
        //             }
        //         }
        //     }
        // }
        true
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Board ({}, {}) with tile_size {}\n",
            self.bounds.size.x, self.bounds.size.y, self.tile_size
        );
        let map = self.map.as_tetris();
        buffer.push_str(&map);
        buffer
    }

    pub(crate) fn shapepos_as_coords(&self, pos: &ShapePosition) -> Coordinates {
        let shape = self
            .current_tetromino_shape
            .as_ref()
            .expect("shapepos_as_coords may only be called with a shape present");
        shape.position_on_board.clone() - shape.anker.clone() + pos.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::{Board, Map, Matrix, Shape, ShapePosition, ShapeType, TileBlueprint, ToMap};
    use bevy::prelude::Entity;
    use bevy::{render::settings::WgpuSettings, winit::WinitPlugin, DefaultPlugins};
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

        let mut uut = Board {
            bounds: Bounds2 {
                position: Vec2::splat(15.0),
                size: Vec2::splat(15.0),
            },
            current_tetromino_shape: Some(ShapeEntity {
                anker: anker.clone(),
                positions: positions.clone(),
                shape_type: ShapeType::S,
                layout: Matrix {
                    width: 3,
                    height: 2,
                },
                position_on_board: (1, 1).into(),
            }),
            entity,
            map,
            tile_size: 99.999,
        };

        assert_eq!(uut.current_tetromino_shape.as_ref().unwrap().anker, (0, 0));
        assert_eq!(
            uut.current_tetromino_shape
                .as_ref()
                .unwrap()
                .position_on_board,
            position_on_board
        );

        let res_coords: Map = res.to_map();
        println!("res_coords: {}", res_coords);
        let target = res_coords.get_current_shape_tile_coordinates();
        let current = uut.get_current_shape_tile_coordinates();
        println!("target: {:?}", target);
        println!("current: {:?}", current);
        let mut coords = vec![];
        for i in 0..4 {
            let block = blocks.get(i).unwrap();
            if let Some((c1, t)) = uut.rotate_block(block, &direction) {
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
        println!("map: {}", uut.map);
        assert!(uut.map.is_empty());
        for coordinates in coords {
            uut.map
                .insert(coordinates, Tile::CurrentTetromino(shape_type.as_char()));
        }
        // let current = uut.get_current_shape_coordinates(); // includes empty coordinates
        let current = uut.get_current_shape_tile_coordinates();
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
        let mut map = Map::new(5, 5);
        map.spawn(&shape, &position_on_board);
        let shapee = ShapeEntity::spawn(shape, &position_on_board, &mut world);

        let uut = Board {
            bounds: Bounds2 {
                position: Vec2::splat(0.),
                size: Vec2::splat(1000.),
            },
            current_tetromino_shape: Some(shapee),
            entity,
            map,
            tile_size: 99.999,
        };

        assert!(uut.is_free(&RotateEvent::ClockWise));
    }
}
