// coordinates.rs
use bevy::prelude::Component;
use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Range, Sub};
#[cfg(feature = "debug")]
use bevy::reflect::Reflect;
#[cfg(feature = "debug")]
use bevy_inspector_egui::InspectorOptions;
#[cfg(feature = "debug")]
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use crate::ShapePosition;

#[cfg_attr(feature = "debug", derive(InspectorOptions, Reflect))]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Component)]
#[cfg_attr(feature = "debug", reflect(InspectorOptions))]
pub struct Coordinates {
    #[cfg_attr(feature = "debug", inspector(min = 10, max = 70))]
    pub x: u16,
    #[cfg_attr(feature = "debug", inspector(min = 10, max = 70))]
    pub y: u16,
}

impl PartialOrd for Coordinates {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.y.cmp(&other.y).then(self.x.cmp(&other.x)))
    }
}

impl Ord for Coordinates {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl From<Coordinates> for (u16, u16) {
    fn from(c: Coordinates) -> Self {
        (c.x, c.y)
    }
}
impl From<(u16, u16)> for Coordinates {
    fn from(c: (u16, u16)) -> Self {
        Coordinates { x: c.0, y: c.1 }
    }
}
pub trait ToCoordinates {
    fn to_coordinates(self) -> Vec<Coordinates>;
}

impl ToCoordinates for Vec<&str> {
    fn to_coordinates(self: Self) -> Vec<Coordinates> {
        self.into_iter()
            .enumerate()
            .map(|(index_y, line)| {
                let x_coords: Vec<u16> = line
                    .chars()
                    .collect::<Vec<char>>()
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index_x, c)| match c {
                        'o' => Some(index_x as u16),
                        'x' => None,
                        _ => None,
                    })
                    .collect();
                let mut res = vec![];
                x_coords.into_iter().for_each(|x| {
                    res.push(Coordinates {
                        x,
                        y: index_y as u16,
                    });
                });
                res
            })
            .flatten()
            .collect::<Vec<Coordinates>>()
    }
}

impl PartialEq<Coordinates> for (u16, u16) {
    fn eq(&self, other: &Coordinates) -> bool {
        self.0 == other.x && self.1 == other.y
    }
}
impl PartialEq<(u16, u16)> for Coordinates {
    fn eq(&self, other: &(u16, u16)) -> bool {
        other.0 == self.x && other.1 == self.y
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<ShapePosition> for Coordinates {
    type Output = Self;

    fn add(self, rhs: ShapePosition) -> Self::Output {
        let new_x = if rhs.x.is_negative() {
            self.x - (-1 * rhs.x) as u16
        } else {
            self.x + rhs.x as u16
        };
        let new_y = if rhs.y.is_negative() {
            self.y - (-1 * rhs.y) as u16
        } else {
            self.y + rhs.y as u16
        };
        Self { x: new_x, y: new_y }
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl Sub<ShapePosition> for Coordinates {
    type Output = Self;

    fn sub(self, rhs: ShapePosition) -> Self::Output {
        Self {
            x: (self.x as i16).saturating_sub(rhs.x) as u16,
            y: (self.y as i16).saturating_sub(rhs.y) as u16,
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<(i16, i16)> for Coordinates {
    type Output = Self;

    fn add(self, (x, y): (i16, i16)) -> Self::Output {
        let x = ((self.x as i16) + x) as u16;
        let y = ((self.y as i16) + y) as u16;
        Self { x, y }
    }
}
impl Coordinates {
    pub fn from_range(range: Range<Coordinates>) -> impl Iterator<Item = Coordinates> {
        (range.start.x..range.end.x)
            .flat_map(move |y| (range.start.y..range.end.y).map(move |x| Coordinates { x, y }))
    }
    pub fn from_u16_range(range: Range<u16>) -> impl Iterator<Item = Coordinates> {
        (range.start..range.end)
            .flat_map(move |y| (range.start..range.end).map(move |x| Coordinates { x, y }))
    }
    pub fn from_size(width: usize, height: usize) -> impl Iterator<Item = Coordinates> {
        (0..width).flat_map(move |y| {
            (0..height).map(move |x| Coordinates {
                x: x as u16,
                y: y as u16,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coords_from_range() {
        let start = Coordinates { x: 0, y: 0 };
        let end = Coordinates { x: 2, y: 2 };
        let result: Vec<_> = Coordinates::from_range(start..end).collect();

        assert_eq!(
            result,
            vec![
                Coordinates { x: 0, y: 0 },
                Coordinates { x: 1, y: 0 },
                Coordinates { x: 0, y: 1 },
                Coordinates { x: 1, y: 1 },
            ]
        )
    }
    #[test]
    fn coords_from_u16_range() {
        let result: Vec<_> = Coordinates::from_u16_range(0..2).collect();

        assert_eq!(
            result,
            vec![
                Coordinates { x: 0, y: 0 },
                Coordinates { x: 1, y: 0 },
                Coordinates { x: 0, y: 1 },
                Coordinates { x: 1, y: 1 },
            ]
        )
    }

    #[test]
    fn test_add_shape_pos_to_coords() {
        assert_eq!(1u16, (-1i16 * -1) as u16);
        assert_eq!(2u16, (-2i16 * -1) as u16);

        let pos = ShapePosition { x: 1, y: 1 };
        let coords = Coordinates { x: 1, y: 1 };

        let res = coords + pos;
        assert_eq!((2u16, 2u16), res);

        let pos = ShapePosition { x: -1, y: -1 };
        let res = coords + pos;
        assert_eq!((0u16, 0u16), res);

        let coords = Coordinates { x: 15, y: 28 };
        let pos = ShapePosition { x: -5, y: -13 };
        let res = coords + pos;
        assert_eq!((10u16, 15u16), res);
    }
    #[test]
    fn test_sub_shape_pos_from_coords() {
        let pos = ShapePosition { x: 1, y: 1 };
        let coords = Coordinates { x: 1, y: 1 };

        let res = coords - pos;
        assert_eq!((0u16, 0u16), res);

        let pos = ShapePosition { x: -1, y: -1 };
        let res = coords - pos;
        assert_eq!((2u16, 2u16), res);

        let coords = Coordinates { x: 15, y: 28 };
        let pos = ShapePosition { x: -5, y: 13 };
        let res = coords - pos;
        assert_eq!((20u16, 15u16), res);
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_add_shape_pos_to_coords_panics() {
        let coords = Coordinates { x: 0, y: 1 };
        let pos = ShapePosition { x: -1, y: 1 };
        let _ = coords + pos;
    }

    #[test]
    fn test_to_coordinates() {
        let input1 = vec!["xxxxx", "xxoox", "xooxx", "xxxxx", "xxxxx"];
        let coords: Vec<Coordinates> = input1.to_coordinates();
        let res = vec![
            Coordinates { x: 2, y: 1 },
            Coordinates { x: 3, y: 1 },
            Coordinates { x: 1, y: 2 },
            Coordinates { x: 2, y: 2 },
        ];
        assert_eq!(coords, res);

        let input2 = vec!["xxxxx", "xoxxx", "xooxx", "xoxxx", "xxxxx"];
        let coords: Vec<Coordinates> = input2.to_coordinates();
        let res = vec![
            Coordinates { x: 1, y: 1 },
            Coordinates { x: 1, y: 2 },
            Coordinates { x: 2, y: 2 },
            Coordinates { x: 1, y: 3 },
        ];
        assert_eq!(coords, res);
    }
}
