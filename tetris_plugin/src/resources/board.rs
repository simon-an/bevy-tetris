use crate::Bounds2;
use crate::Coordinates;
use crate::RotateEvent;
use crate::ShapePosition;
use bevy::prelude::*;

#[derive(Debug)]
pub(crate) struct Board {
    pub bounds: Bounds2,
    pub tile_size: f32,
    // pub current_tetromino_shape: Option<ShapeEntity>, // now a world resource
    pub entity: Entity,
}

impl Board {
    // TODO TEST THIS
    // pub fn set_map(&mut self, map: Map) {
    //     self.current_tetromino_shape = Some(map.try_parse_shape().expect("shape must be parsable"));
    //     self.map = map;
    // }

    pub fn calc_translation(&self, coordinates: &Coordinates) -> (f32, f32) {
        let new_x: f32 = (coordinates.x as f32 * self.tile_size) + (self.tile_size / 2.0);
        let new_y: f32 =
            self.bounds.size.y - (coordinates.y as f32 * self.tile_size) - 0.5 * self.tile_size;

        (new_x, new_y)
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
            entity,
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

            entity,
            tile_size: 99.999,
        };

        assert!(uut.is_free(&RotateEvent::ClockWise));
    }
}
