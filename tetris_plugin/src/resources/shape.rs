use core::fmt;
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

use bevy::prelude::{trace, warn, Color, Component, Reflect, Resource};
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::{Coordinates, Matrix, MoveEvent, TileBlueprint};

// Holds a block's position within a tetromino for rotation
#[cfg_attr(feature = "debug", derive(Reflect))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Component)]
pub struct ShapePosition {
    pub x: i16,
    pub y: i16,
}

impl From<ShapePosition> for (i16, i16) {
    fn from(c: ShapePosition) -> Self {
        (c.x, c.y)
    }
}
impl From<(i16, i16)> for ShapePosition {
    fn from(c: (i16, i16)) -> Self {
        Self { x: c.0, y: c.1 }
    }
}

impl PartialEq<ShapePosition> for (i16, i16) {
    fn eq(&self, other: &ShapePosition) -> bool {
        self.0 == other.x && self.1 == other.y
    }
}
impl PartialEq<(i16, i16)> for ShapePosition {
    fn eq(&self, other: &(i16, i16)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

// #[derive(Debug, Clone, Resource)]
// pub struct ShapeEntity {
//     pub shape_type: ShapeType,
//     // pub positions: BTreeMap<Entity, ShapePosition>,
//     // pub positions: HashMap<ShapePosition, Option<Entity>>, // this would allow a shape without a entity
//     // pub positions: HashMap<ShapePosition, Option<Entity>>, // this would allow a shape without a entity
//     pub anker: Coordinates, // Should be the top left corner
//     pub position_on_board: Coordinates,
//     pub layout: Matrix,
// }

#[derive(Debug, Clone)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub anker: Coordinates, // Should be the top left corner
    pub layout: Matrix,
    pub positions: BTreeMap<ShapePosition, TileBlueprint>, //[TileBlueprint; N], // size = layout.x * layout.y
}
impl Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, (h: {},w: {}))",
            self.shape_type, self.anker, self.layout.height, self.layout.width
        )
    }
}

#[cfg(test)]
use bevy::prelude::World;

// impl ShapeEntity {
//     pub fn shapepos_as_coords(&self, pos: &ShapePosition) -> Coordinates {
//         self.position_on_board.clone() - self.anker.clone() + pos.clone()
//     }

//     pub fn move_shape(&mut self, event: &MoveEvent) {
//         let old = self.position_on_board.clone();
//         match event {
//             &MoveEvent::Down => {
//                 self.position_on_board.y += 1;
//             }
//             &MoveEvent::Left => {
//                 if self.position_on_board.x > 0 {
//                     self.position_on_board.x -= 1;
//                 } else {
//                     warn!("cannot move left");
//                 }
//             }
//             &MoveEvent::Right => {
//                 self.position_on_board.x += 1;
//             }
//         };
//         trace!("shape has moved from {} to {}", old, self.position_on_board);
//     }

//     #[cfg(test)]
//     pub fn spawn(shape: Shape, position_on_board: &Coordinates, world: &mut World) -> Self {
//         use bevy::prelude::Entity;

//         use crate::components::Block;

//         let Shape {
//             anker,
//             layout,
//             positions,
//             shape_type,
//         } = shape;
//         let positions: BTreeMap<Entity, ShapePosition> = positions
//             .into_iter()
//             .map(|(pos, _blueprint)| (world.spawn((Block, pos)).id(), pos))
//             .collect();
//         Self {
//             anker,
//             layout,
//             shape_type,
//             position_on_board: position_on_board.clone(),
//         }
//     }
//     // pub fn is_y_i(&self) -> bool {
//     //     assert_eq!(self.shape_type, ShapeType::I);
//     //     assert_eq!(positions.len(), 4);
//     //     let positions: Vec<&ShapePosition> = positions.values().collect();
//     //     positions[0].x == 0 && positions[1].x == 0 && positions[2].x == 0 && positions[3].x == 0
//     // }
//     // pub fn is_x_i(&self) -> bool {
//     //     assert_eq!(self.shape_type, ShapeType::I);
//     //     let positions: Vec<&ShapePosition> = positions.values().collect();
//     //     assert_eq!(positions.len(), 4);
//     //     positions[0].y == 0 && positions[1].x == 0 && positions[2].x == 0 && positions[3].x == 0
//     // }
//     // pub fn is_block(&self, c: Coordinates) -> Option<ShapePosition> {
//     //     positions
//     //         .values()
//     //         .find(|p| (c - (self.anker + **p)) == self.position_on_board)
//     //         .map(|p| p.clone())
//     // }
//     // pub fn reflect(&self, c: Coordinates) -> Option<ShapePosition> {
//     //     positions
//     //         .values()
//     //         .find(|p| (c - (self.anker + **p)) == self.position_on_board)
//     //         .map(|s| s.clone())
//     // }
// }

#[cfg_attr(feature = "debug", derive(Reflect))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShapeType {
    I = 0,
    O = 1,
    T = 2,
    S = 3,
    Z = 4,
    L = 5,
    J = 6,
}
#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct NextShape(pub ShapeType);

impl Display for ShapeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self)
    }
}

impl ShapeType {
    pub fn get_color(&self) -> Color {
        match self {
            &Self::I => Color::srgb(0.0, 0.7, 0.7),
            &Self::O => Color::srgb(0.7, 0.7, 0.0), // square, yellow
            &Self::T => Color::srgb(0.7, 0.0, 0.7), // T, purple
            &Self::Z => Color::srgb(0.7, 0.0, 0.0), // Z, red
            &Self::S => Color::srgb(0.0, 0.7, 0.0), // S, green
            &Self::L => Color::srgb(0.0, 0.0, 0.7), // L, blue
            &Self::J => Color::srgb(0.9, 0.25, 0.0), // J, orange
        }
    }

    pub(crate) fn as_char(&self) -> char {
        match self {
            &Self::I => 'i',
            &Self::O => 'o',
            &Self::T => 't',
            &Self::Z => 'z',
            &Self::S => 's',
            &Self::L => 'l',
            &Self::J => 'j',
        }
    }
    pub(crate) fn from_char(c: char) -> Self {
        match c {
            'i' => Self::I,
            'o' => Self::O,
            't' => Self::T,
            'z' => Self::Z,
            's' => Self::S,
            'l' => Self::L,
            'j' => Self::J,
            _ => panic!("unkown char"),
        }
    }

    pub(crate) fn is_shape_type(c: char) -> bool {
        match c {
            'i' => true,
            'o' => true,
            't' => true,
            'z' => true,
            's' => true,
            'l' => true,
            'j' => true,
            _ => false,
        }
    }
}

impl Shape {
    pub fn blueprint(shape_type: ShapeType) -> Shape {
        Shape::blueprints().remove(shape_type as usize)
    }

    pub fn initial_occupied(&self) -> Vec<&ShapePosition> {
        self.positions.keys().collect()
    }

    fn blueprints() -> Vec<Shape> {
        let blueprint = vec![
            // line, cyan
            Shape {
                shape_type: ShapeType::I,
                anker: (0, 0).into(), // Should be the top left corner
                layout: Matrix {
                    width: 1,
                    height: 4,
                },
                positions: vec![
                    ((0, 3).into(), TileBlueprint::CurrentTetromino),
                    ((0, 2).into(), TileBlueprint::CurrentTetromino),
                    ((0, 1).into(), TileBlueprint::CurrentTetromino),
                    ((0, 0).into(), TileBlueprint::CurrentTetromino),
                ]
                .into_iter()
                .collect::<BTreeMap<ShapePosition, TileBlueprint>>(),
            },
            // square, yellow
            Shape {
                shape_type: ShapeType::O,
                anker: (0, 0).into(), // Should be the top left corner
                layout: Matrix {
                    width: 2,
                    height: 2,
                },
                positions: vec![
                    ((0, 0).into(), TileBlueprint::CurrentTetromino),
                    ((0, 1).into(), TileBlueprint::CurrentTetromino),
                    ((1, 0).into(), TileBlueprint::CurrentTetromino),
                    ((1, 1).into(), TileBlueprint::CurrentTetromino),
                ]
                .into_iter()
                .collect::<BTreeMap<ShapePosition, TileBlueprint>>(),
            },
            Shape {
                shape_type: ShapeType::T,
                anker: (0, 0).into(), // Should be the top left corner
                layout: Matrix {
                    width: 3,
                    height: 2,
                },
                positions: vec![
                    ((0, 0).into(), TileBlueprint::CurrentTetromino),
                    ((0, 1).into(), TileBlueprint::CurrentTetromino),
                    ((0, 2).into(), TileBlueprint::CurrentTetromino),
                    ((1, 1).into(), TileBlueprint::CurrentTetromino),
                    ((1, 0).into(), TileBlueprint::Empty),
                    ((1, 2).into(), TileBlueprint::Empty),
                ]
                .into_iter()
                .collect::<BTreeMap<ShapePosition, TileBlueprint>>(),
            },
            Shape {
                shape_type: ShapeType::S,
                anker: (0, 0).into(), // Should be the top left corner
                layout: Matrix {
                    width: 3,
                    height: 2,
                },
                positions: vec![
                    ((0, 1).into(), TileBlueprint::CurrentTetromino),
                    ((1, 1).into(), TileBlueprint::CurrentTetromino),
                    ((1, 0).into(), TileBlueprint::CurrentTetromino),
                    ((2, 0).into(), TileBlueprint::CurrentTetromino),
                    ((0, 0).into(), TileBlueprint::Empty),
                    ((2, 1).into(), TileBlueprint::Empty),
                ]
                .into_iter()
                .collect::<BTreeMap<ShapePosition, TileBlueprint>>(),
            },
            Shape {
                shape_type: ShapeType::Z,
                anker: (0, 0).into(), // Should be the top left corner
                layout: Matrix {
                    width: 3,
                    height: 2,
                },
                positions: vec![
                    ((0, 0).into(), TileBlueprint::CurrentTetromino),
                    ((1, 0).into(), TileBlueprint::CurrentTetromino),
                    ((1, 1).into(), TileBlueprint::CurrentTetromino),
                    ((2, 1).into(), TileBlueprint::CurrentTetromino),
                    ((2, 0).into(), TileBlueprint::Empty),
                    ((0, 1).into(), TileBlueprint::Empty),
                ]
                .into_iter()
                .collect::<BTreeMap<ShapePosition, TileBlueprint>>(),
            },
            Shape {
                shape_type: ShapeType::L,
                anker: (0, 0).into(), // Should be the top left corner
                layout: Matrix {
                    width: 3,
                    height: 2,
                },
                positions: vec![
                    ((0, 0).into(), TileBlueprint::CurrentTetromino),
                    ((0, 1).into(), TileBlueprint::CurrentTetromino),
                    ((1, 0).into(), TileBlueprint::CurrentTetromino),
                    ((2, 0).into(), TileBlueprint::CurrentTetromino),
                    ((1, 1).into(), TileBlueprint::Empty),
                    ((2, 1).into(), TileBlueprint::Empty),
                ]
                .into_iter()
                .collect::<BTreeMap<ShapePosition, TileBlueprint>>(),
            },
            Shape {
                shape_type: ShapeType::J,
                anker: (0, 0).into(), // Should be the top left corner
                layout: Matrix {
                    width: 3,
                    height: 2,
                },
                positions: vec![
                    ((0, 0).into(), TileBlueprint::CurrentTetromino),
                    ((1, 0).into(), TileBlueprint::CurrentTetromino),
                    ((2, 0).into(), TileBlueprint::CurrentTetromino),
                    ((2, 1).into(), TileBlueprint::CurrentTetromino),
                    ((1, 1).into(), TileBlueprint::Empty),
                    ((0, 1).into(), TileBlueprint::Empty),
                ]
                .into_iter()
                .collect::<BTreeMap<ShapePosition, TileBlueprint>>(),
            },
        ];

        // for b in blueprint.iter() {
        //     let size = b.layout.width * b.layout.height;
        //     assert_eq!(size as usize, b.positions.len());
        //     // for i in b.anker.x..b.layout.width - b.anker.x {
        //         // for j in b.anker.y..b.layout.height - b.anker.y {
        //     for i in 0..b.layout.width {
        //         for j in 0..b.layout.height {
        //             let x = b.positions.get(&ShapePosition {
        //                 x: i as i16,
        //                 y: j as i16,
        //             });
        //             assert!(x.is_some(), "{b}: ({i} {j})");
        //         }
        //     }
        // }

        blueprint
    }
}

impl Distribution<ShapeType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ShapeType {
        match rng.gen_range(0..6) {
            0 => ShapeType::I,
            1 => ShapeType::O,
            2 => ShapeType::T,
            3 => ShapeType::S,
            4 => ShapeType::Z,
            5 => ShapeType::L,
            _ => ShapeType::J,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ShapePosition;
    use crate::Shape;

    #[test]
    fn test_from_shapepos_from_i16() {
        let x: ShapePosition = (0i16, 15i16).into();
        assert_eq!(0, x.x);
        assert_eq!(15, x.y);
    }

    #[test]
    fn test_blueprints() {
        Shape::blueprints();
    }
}
