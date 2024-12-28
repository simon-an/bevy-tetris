use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
};

use bevy::prelude::Resource;
use colored::Colorize;

use crate::{
    components::Matrix, events::MoveEvent, Coordinates, RotateEvent, Shape, ShapeType, Tile,
    TileBlueprint, Transitions,
};

pub type MapTile = Tile;

#[derive(PartialEq, Eq, PartialOrd, Ord, Resource)]
pub(crate) struct Map {
    pub(crate) inner: BTreeMap<Coordinates, MapTile>,
    pub width: usize,
    pub height: usize,
    // pub transitions: Option<Vec<(Coordinates, Coordinates)>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum CollisionDetection {
    Block,
    OutOfBounds,
    Bottom,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let inner: BTreeMap<Coordinates, MapTile> = Map::create_tiles(width, height);

        Self {
            inner,
            width,
            height,
        }
    }

    pub(crate) fn create_tiles(width: usize, height: usize) -> BTreeMap<Coordinates, Tile> {
        let tiles = (0..(width * height))
            .into_iter()
            .map(|i| {
                let x = i % width;
                let y = i / width;
                if i == 0 {
                    assert_eq!(x, 0);
                    assert_eq!(y, 0);
                }
                if i == width - 1 {
                    assert_eq!(x, width - 1);
                    assert_eq!(y, 0);
                }
                if i == width {
                    assert_eq!(x, 0);
                    assert_eq!(y, 1);
                }
                if i == width * height {
                    assert_eq!(x, height);
                    assert_eq!(y, width);
                }
                let coordinate: Coordinates = (x as u16, y as u16).into();
                (coordinate, Tile::Empty)
            })
            .collect::<BTreeMap<Coordinates, Tile>>();

        tiles
    }

    pub fn get(&self, coordinates: &Coordinates) -> Option<&Tile> {
        self.inner.get(coordinates)
    }

    // fn get_current_shape(&self) -> Vec<(Coordinates, Tile)> {
    //     self.inner
    //         .iter()
    //         .filter(|(_, t)| t.is_moveable()) // returns empty as well???
    //         .map(|(c, t)| (c.clone(), t.clone()))
    //         .collect()
    // }

    // pub fn get_current_shape_coordinates(&self) -> Vec<Coordinates> {
    //     self.inner
    //         .iter()
    //         .filter(|(_, t)| t.is_moveable()) // returns empty as well???
    //         .map(|(c, _)| c.clone())
    //         .collect()
    // }
    // pub fn get_current_shape_tile_coordinates(&self) -> Vec<Coordinates> {
    //     self.inner
    //         .iter()
    //         .filter(|(_, t)| t.is_moveable())
    //         .map(|(c, _)| c.clone())
    //         .collect()
    // }
    // pub fn get_block_coordinates(&self) -> Vec<&Coordinates> {
    //     self.inner
    //         .iter()
    //         .filter(|(_, t)| t.is_block())
    //         .map(|(c, _)| c)
    //         .collect()
    // }
    // pub fn is_empty(&self) -> bool {
    //     self.inner
    //         .iter()
    //         .filter(|(_, t)| t != &&Tile::Empty)
    //         .count()
    //         == 0
    // }
    pub fn occupied(&self) -> usize {
        self.inner
            .iter()
            .filter(|(_, t)| t != &&Tile::Empty)
            .count()
    }

    // pub fn insert(&mut self, coordinates: &Coordinates, tile: Tile) {
    //     self.inner.insert(coordinates, tile);
    // }
    pub fn insert(&mut self, coordinates: Coordinates, tile: MapTile) -> Option<MapTile> {
        self.inner.insert(coordinates, tile)
    }

    pub fn detect_move_collision(&self, event: &MoveEvent) -> Option<CollisionDetection> {
        self.inner
            .iter()
            .filter(|(_, t)| t.is_moveable())
            .find_map(|(c, _tile)| {
                let new_coordinates = match event {
                    &MoveEvent::Down => *c + (0, 1),
                    &MoveEvent::Left => *c + (-1, 0),
                    &MoveEvent::Right => *c + (1, 0),
                };

                if new_coordinates.y == self.height as u16 {
                    Some(CollisionDetection::Bottom)
                } else {
                    match self.inner.get(&new_coordinates) {
                        Some(Tile::CurrentTetromino(_)) => None, // TODO optimize movement to move only required tiles
                        Some(Tile::Block(_)) => Some(CollisionDetection::Block),
                        Some(Tile::Empty) => None,
                        None => Some(CollisionDetection::OutOfBounds),
                    }
                }
            })
    }

    pub fn detect_collision(&self) -> Option<CollisionDetection> {
        self.inner
            .iter()
            .filter(|(_, t)| t.is_moveable())
            .find_map(|(c, _)| {
                let new_coordinates = *c + (0, 1);
                if new_coordinates.y == self.height as u16 {
                    Some(CollisionDetection::Bottom)
                } else {
                    match self.inner.get(&new_coordinates) {
                        Some(Tile::CurrentTetromino(_)) => None, // TODO optimize movement to move only required tiles
                        Some(Tile::Block(_)) => Some(CollisionDetection::Block),
                        Some(Tile::Empty) => None,
                        None => Some(CollisionDetection::OutOfBounds),
                    }
                }
            })
    }
    pub fn as_coordinates(&self) -> Vec<&Coordinates> {
        self.inner.keys().collect()
    }
    pub fn empty_square(width: u16) -> Self {
        Self {
            inner: Coordinates::from_u16_range(0..width)
                .map(|c| (c, MapTile::Empty))
                .collect::<BTreeMap<Coordinates, MapTile>>(),
            width: width as usize,
            height: width as usize,
        }
    }
    pub fn empty_rect(width: usize, height: usize) -> Self {
        Self {
            inner: Coordinates::from_size(width, height)
                .map(|c| (c, MapTile::Empty))
                .collect::<BTreeMap<Coordinates, MapTile>>(),
            width,
            height,
        }
    }

    pub fn as_savegame_string(&self) -> String {
        let mut buffer = String::new();
        for (i, (_coord, tile)) in self.inner.iter().enumerate() {
            if i % self.width == 0 && i > 1 {
                buffer.push('\n');
            }
            let c = match tile {
                &MapTile::Block(c) => c.to_string(),
                &MapTile::CurrentTetromino(c) => c.to_ascii_uppercase().to_string(),
                &MapTile::Empty => 'x'.to_string(),
            };
            buffer.push_str(&c);
        }
        buffer
    }
    pub fn as_tetris(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str("|0||1||2||3||4||5||6||7||8||9|\n");
        use std::cmp::*;
        let mut vec: Vec<(&Coordinates, &Tile)> = self.inner.iter().collect();
        vec.sort_unstable_by(|a, b| a.0.y.cmp(&b.0.y).then(a.0.x.cmp(&b.0.x)));
        for (i, (_c, tile)) in vec.iter().enumerate() {
            if i > 0 && i % (self.width) == 0 {
                buffer.push('\n');
            }
            buffer.push('|');
            // buffer.push_str(&tile.to_string());
            let c = match tile {
                &MapTile::Block(c) => c.to_string().red().to_string(),
                &MapTile::CurrentTetromino(c) => {
                    format!("{}", c.to_ascii_uppercase()).green().to_string()
                }
                &MapTile::Empty => ' '.to_string(),
            };
            buffer.push_str(&format!("{}", c).green().to_string());
            buffer.push('|');
        }
        // buffer.push('\n');
        buffer
    }

    pub fn spawn(&mut self, shape: &Shape, spawn_pos: &Coordinates) {
        shape
            .positions
            .iter()
            .map(|(pos, bp)| {
                let tile = if bp == &TileBlueprint::CurrentTetromino {
                    MapTile::CurrentTetromino(shape.shape_type.as_char())
                } else {
                    MapTile::Empty
                };
                (
                    (*spawn_pos + *pos),
                    // Tile::CurrentTetromino(shape.shape_type.get_color().into(), e),
                    tile,
                )
            })
            .for_each(|(c, tile)| {
                println!("placing tile {tile} at {c}");
                self.insert(c, tile);
            });
    }

    pub(crate) fn set_lines_to_empty(&mut self, lines: Vec<u16>) -> Vec<Coordinates> {
        println!("set_lines_to_empty {:?}", lines);
        let mut delted_blocks: Vec<Coordinates> = vec![];
        for i in lines.clone() {
            let mut r = self.set_line_to_empty(i);
            delted_blocks.append(&mut r);
        }
        delted_blocks
    }
    pub(crate) fn set_line_to_empty(&mut self, line: u16) -> Vec<Coordinates> {
        let mut res = vec![];
        self.inner
            .iter_mut()
            .filter(|(c, _)| c.y == line)
            .for_each(|(c, t)| {
                assert!(t.is_block());
                *t = MapTile::Empty;
                res.push(c.clone());
            });

        res
    }

    pub(crate) fn is_line_full(&self, y: u16) -> bool {
        let mut count = 0;
        self.inner
            .iter()
            .filter(|(c, _)| c.y == y)
            .for_each(|(_, t)| {
                if t == &MapTile::Empty {
                    // TODO how to fail fast?
                } else if t.is_block() {
                    count += 1;
                }
            });

        return count == self.width;
    }

    /**
     * Returns a Vec of the lines that have to be deleted and the transitions of the blocks which have to be moved.
     * The transitions are sorted in a way that the blocks are moved from the bottom to the top.
     */
    pub fn move_blocks_above_empty_lines(&mut self) -> Option<(Vec<u16>, Transitions)> {
        let mut lines_with_blocks = vec![];
        let mut lines_without_blocks = vec![];
        for (c, t) in self.inner.iter() {
            if t.is_block() {
                lines_with_blocks.push(c.y);
            }
        }
        lines_with_blocks.sort();
        lines_with_blocks.dedup();
        for i in 0..self.height {
            if !lines_with_blocks.contains(&(i as u16)) {
                lines_without_blocks.push(i as u16);
            }
        }

        let mut lines_tuppels_old = vec![];
        let mut lines_to_delete = vec![];
        for y1 in &lines_with_blocks {
            for y2 in &lines_without_blocks.clone() {
                if y1 < y2 {
                    lines_tuppels_old.push((y1.clone(), y2.clone()));
                    lines_to_delete.push(y2.clone());
                }
            }
        }

        lines_to_delete.sort();
        lines_to_delete.dedup();
        if lines_tuppels_old.is_empty() {
            return None;
        }

        println!("lines_to_move {:?}", lines_tuppels_old);
        lines_tuppels_old.reverse();
        println!("lines_to_move {:?}", lines_tuppels_old);
        println!("lines_to_delete {:?}", lines_to_delete);

        let mut transitions = vec![];

        for (y_source, _deleted_line) in lines_tuppels_old.clone() {
            for x in 0..self.width as u16 {
                let from = (x, y_source).into();
                let current = self.inner.get(&from);
                if let Some(t) = current {
                    if t.is_block() {
                        let to = (x, y_source + lines_to_delete.len() as u16).into();
                        let current = self.inner.insert(from.clone(), MapTile::Empty);
                        let must_be_empty = self
                            .inner
                            .insert(to, current.expect("cooridinates must exist"));
                        assert!(matches!(must_be_empty, Some(MapTile::Empty)));
                        transitions.push((from, to))
                    }
                }
            }
        }

        let transitions = Transitions(transitions);
        lines_to_delete.sort();
        Some((lines_to_delete, transitions))
    }
    // fn move_blocks_above_line(&mut self, lines: Vec<u16>) -> Vec<(Coordinates, i16)> {
    //     let mut old_blocks = vec![];
    //     for (c, t) in self.inner.iter_mut() {
    //         if let Tile::Block(color) = t {
    //             let delta_y = get_lines_south(&c.y, &lines) as i16;
    //             if delta_y > 0 {
    //                 old_blocks.push((c.clone(), delta_y, color.clone()));
    //                 *t = MapTile::Empty;
    //             }
    //         }
    //     }
    //     for (coord, _, color) in old_blocks.iter() {
    //         self.insert(coord.clone(), MapTile::Block(*color));
    //     }
    //     old_blocks.into_iter().map(|(c, t, _)| (c, t)).collect()
    // }

    pub(crate) fn from_str(text: &str) -> Self {
        let lines: Vec<&str> = text.split('\n').collect();
        lines.to_map()
    }

    // pub(crate) fn try_parse_shape(&self) -> Result<ShapeEntity, ()> {
    //     struct Bounds {
    //         min_x: u16,
    //         min_y: u16,
    //         max_x: u16,
    //         max_y: u16,
    //     }
    //     let shape_parts = self.get_current_shape();
    //     let bounds: Bounds = shape_parts.iter().map(|(c, _)| c).fold(
    //         Bounds {
    //             min_x: 9999,
    //             min_y: 9999,
    //             max_x: 0,
    //             max_y: 0,
    //         },
    //         |mut state, &c| {
    //             state.min_x = c.x.min(state.min_x);
    //             state.min_y = c.y.min(state.min_y);
    //             state.max_x = c.x.max(state.max_x);
    //             state.max_y = c.y.max(state.max_y);
    //             state
    //         },
    //     );
    //     let m = Matrix {
    //         width: 1 + bounds.max_x - bounds.min_x,
    //         height: 1 + bounds.max_y - bounds.min_y,
    //     };
    //     match shape_parts.get(0).expect("must exist").1 {
    //         Tile::CurrentTetromino(c) => Ok(ShapeEntity {
    //             anker: (0, 0).into(),
    //             layout: m,
    //             position_on_board: (bounds.min_x, bounds.min_y).into(),
    //             shape_type: ShapeType::from_char(c),
    //         }),
    //         _ => Err(()),
    //     }
    // }
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
}

fn get_lines_south(y: &u16, lines: &Vec<u16>) -> u16 {
    lines.iter().filter(|line| line > &y).count() as u16
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.as_tetris();
        writeln!(f, "{}", s)?;
        Ok(())
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "------------------------------------")?;
        for (i, (c, tile)) in self.inner.iter().enumerate() {
            if i % self.width == 0 && i > 1 {
                writeln!(f, "\n------------------------------------")?;
            }
            let t: char = match tile {
                &MapTile::Block(c) => c,
                &MapTile::CurrentTetromino(c) => c.to_ascii_uppercase(),
                &MapTile::Empty => 'x',
            };
            write!(f, "{}({},{})|", format!("{}", t).green(), c.x, c.y)?;
        }
        writeln!(f, "\n------------------------------------")?;
        Ok(())
    }
}

impl Into<Map> for Vec<&str> {
    fn into(self) -> Map {
        self.to_map()
    }
}

pub(crate) trait ToMap {
    type Item: Sized;
    fn to_map(self) -> Self::Item;
}

impl ToMap for Vec<&str> {
    type Item = Map;
    fn to_map(self: Self) -> Self::Item {
        let len: usize = self.len();
        let vec_vec = self
            .into_iter()
            .enumerate()
            .map(|(index_y, line)| {
                let tiles: Vec<(usize, MapTile)> = line
                    .chars()
                    .collect::<Vec<char>>()
                    .into_iter()
                    .enumerate()
                    .map(|(index_x, c)| match c {
                        c if c.is_ascii_uppercase() => {
                            (index_x, MapTile::CurrentTetromino(c.to_ascii_lowercase()))
                        }
                        'x' => (index_x, MapTile::Empty),
                        c => (index_x, MapTile::Block(c)),
                        // forbidden_char => panic!("found a forbidden character: {forbidden_char}"),
                    })
                    .collect();

                let inner_map = tiles
                    .into_iter()
                    .map(|(x, t)| {
                        (
                            Coordinates {
                                x: x as u16,
                                y: index_y as u16,
                            },
                            t,
                        )
                    })
                    .collect::<Vec<(Coordinates, MapTile)>>();
                inner_map
            })
            .collect::<Vec<Vec<(Coordinates, MapTile)>>>();

        let height = vec_vec.len();
        let width = vec_vec[0].len();
        let invalid_len = vec_vec
            .iter()
            .find(|line_parse_result| line_parse_result.len() > width);
        if invalid_len.is_some() {
            panic!("a string is not as long as the first string in this vector")
        }
        assert_eq!(height, len);
        let inner = vec_vec
            .into_iter()
            .flatten()
            .collect::<BTreeMap<Coordinates, MapTile>>();

        Map {
            inner,
            height,
            width,
        }
    }
}
impl ToMap for String {
    type Item = Map;
    fn to_map(self: Self) -> Self::Item {
        let mut lines: Vec<&str> = self.split('\n').collect();
        let first_line: &str = lines.remove(0);
        let width = first_line.replace('|', "").len();
        let lines = lines
            .into_iter()
            .enumerate()
            .map(|(index_y, line)| {
                let tiles: Vec<(usize, MapTile)> = line
                    .chars()
                    .filter(|c| c != &'|')
                    .collect::<Vec<char>>()
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index_x, c)| match c {
                        ' ' => Some((index_x, MapTile::Empty)),
                        c if c.is_ascii_uppercase() => {
                            Some((index_x, MapTile::CurrentTetromino(c.to_ascii_lowercase())))
                        }
                        c if ShapeType::is_shape_type(c) => Some((index_x, MapTile::Block(c))),
                        '\r' => panic!("IDIOT! SAVE FILES WITH LF INSTEAD OF CRLF"), // panic because enumeration index does not match from here on
                        forbidden_char => panic!("found a forbidden character: {forbidden_char}"), // panic because enumeration index does not match from here on
                    })
                    .collect();

                let inner_map = tiles
                    .into_iter()
                    .map(|(x, t)| {
                        (
                            Coordinates {
                                x: x as u16,
                                y: index_y as u16,
                            },
                            t,
                        )
                    })
                    .collect::<Vec<(Coordinates, MapTile)>>();
                inner_map
            })
            .collect::<Vec<Vec<(Coordinates, MapTile)>>>();

        let height = lines.len();
        let invalid_len = lines
            .iter()
            .find(|line_parse_result| line_parse_result.len() > width);
        if invalid_len.is_some() {
            panic!("a string is not as long as the first string in this vector")
        }
        assert_eq!(height, 22);
        assert_eq!(width, 10);
        let inner = lines
            .into_iter()
            .flatten()
            .collect::<BTreeMap<Coordinates, MapTile>>();

        Map {
            inner,
            height,
            width,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Map;
    use crate::{CollisionDetection, Coordinates, MapTile, Shape, ShapeType, Tile, ToMap};
    use pretty_assertions::assert_eq;
    use std::{
        collections::BTreeMap,
        fs::File,
        io::{BufWriter, Write},
    };

    #[test]
    fn test_btreemap_sorted_correctly() {
        let mut map: BTreeMap<Coordinates, Option<bool>> = BTreeMap::default();
        map.insert((1, 1).into(), None);
        map.insert((5, 0).into(), None);
        map.insert((2, 2).into(), None);
        map.insert((11, 0).into(), None);
        map.insert((0, 0).into(), None);

        let mut keys = map.keys();
        assert_eq!(keys.next(), Some(&(0, 0).into()));
        assert_eq!(keys.next(), Some(&(5, 0).into()));
        assert_eq!(keys.next(), Some(&(11, 0).into()));
        assert_eq!(keys.next(), Some(&(1, 1).into()));
        assert_eq!(keys.next(), Some(&(2, 2).into()));
    }
    #[test]
    fn test_empty_square() {
        let expected = Map::empty_square(5);
        assert_eq!(expected.height, 5);
        assert_eq!(expected.width, 5);
        assert_eq!(expected.as_coordinates().len(), 25);
    }
    #[test]
    fn test_empty_rect() {
        let expected = Map::empty_rect(5, 9);
        assert_eq!(expected.as_coordinates().len(), 45);
        assert_eq!(expected.height, 9);
        assert_eq!(expected.width, 5);
    }

    #[test]
    fn test_as_savegame_string() {
        let input1 = vec!["xxxxx", "xxTTx", "xTTxx", "xxxxx", "xxxxx"];
        let mut uut = Map::empty_square(5);
        uut.insert(Coordinates { x: 2, y: 1 }, MapTile::CurrentTetromino('t'));
        uut.insert(Coordinates { x: 3, y: 1 }, MapTile::CurrentTetromino('t'));
        uut.insert(Coordinates { x: 1, y: 2 }, MapTile::CurrentTetromino('t'));
        uut.insert(Coordinates { x: 2, y: 2 }, MapTile::CurrentTetromino('t'));

        assert_eq!(uut.width, 5);
        assert_eq!(uut.height, 5);
        assert_eq!(uut.as_coordinates().len(), 25);
        assert_eq!(uut.as_savegame_string(), input1.join("\n"));
    }
    #[test]
    fn test_to_map() {
        let input1 = vec!["xxxxx", "xxOOx", "xOOxx", "xxxxx", "xxxxx"];
        let uut: Map = input1.to_map();
        let mut expected = Map::empty_square(5);
        expected.insert(Coordinates { x: 2, y: 1 }, MapTile::CurrentTetromino('o'));
        expected.insert(Coordinates { x: 3, y: 1 }, MapTile::CurrentTetromino('o'));
        expected.insert(Coordinates { x: 1, y: 2 }, MapTile::CurrentTetromino('o'));
        expected.insert(Coordinates { x: 2, y: 2 }, MapTile::CurrentTetromino('o'));

        assert_eq!(expected.width, 5);
        assert_eq!(expected.height, 5);
        assert_eq!(expected.as_coordinates().len(), 25);
        assert_eq!(uut.as_coordinates().len(), 25);

        assert_eq!(uut, expected);

        let input2 = vec!["xxxxx", "xOxxx", "xOOxx", "xOxxx", "xxxxx"];
        let uut: Map = input2.to_map();
        let mut expected = Map::empty_square(5);
        expected.insert(Coordinates { x: 1, y: 1 }, MapTile::CurrentTetromino('o'));
        expected.insert(Coordinates { x: 1, y: 2 }, MapTile::CurrentTetromino('o'));
        expected.insert(Coordinates { x: 2, y: 2 }, MapTile::CurrentTetromino('o'));
        expected.insert(Coordinates { x: 1, y: 3 }, MapTile::CurrentTetromino('o'));
        assert_eq!(expected.as_coordinates().len(), 25);
        assert_eq!(uut.as_coordinates().len(), 25);
        assert_eq!(expected.width, 5);
        assert_eq!(expected.height, 5);
        assert_eq!(uut, expected);
    }
    #[test]
    fn test_detect_collision() {
        let input1 = vec!["xxxxx", "xxxxx", "xxxxx", "xOOOx", "xxOxx"];
        let uut: Map = input1.to_map();
        assert_eq!(uut.as_coordinates().len(), 25);
        let collision = uut.detect_collision();
        assert_eq!(Some(CollisionDetection::Bottom), collision);
    }
    #[test]
    fn test_detect_move_collision_bottom() {
        let input1 = vec!["xxxxx", "xxxxx", "xxxxx", "xOOOx", "xxOxx"];
        let uut: Map = input1.to_map();
        assert_eq!(uut.as_coordinates().len(), 25);
        let collision = uut.detect_move_collision(&crate::events::MoveEvent::Down);
        assert_eq!(Some(CollisionDetection::Bottom), collision);
    }
    #[test]
    fn test_detect_move_boarder_left() {
        let input1 = vec!["xxxxx", "xxxxx", "xxxxx", "OOOxx", "xxOxx"];
        let uut: Map = input1.to_map();
        assert_eq!(uut.as_coordinates().len(), 25);
        let collision = uut.detect_move_collision(&crate::events::MoveEvent::Left);
        assert_eq!(Some(CollisionDetection::OutOfBounds), collision);

        let input1 = vec!["xxxxx", "xxxxx", "xxOOx", "xxOxx", "xxOxx"];
        let uut: Map = input1.to_map();
        assert_eq!(uut.as_coordinates().len(), 25);
        let collision = uut.detect_move_collision(&crate::events::MoveEvent::Left);
        assert_eq!(None, collision);
    }
    #[test]
    fn test_detect_move_boarder_right() {
        let input1 = vec!["xxxxx", "xxxxx", "xxxxx", "xxOOO", "xxOxx"];
        let uut: Map = input1.to_map();
        assert_eq!(uut.as_coordinates().len(), 25);
        let collision = uut.detect_move_collision(&crate::events::MoveEvent::Right);
        assert_eq!(Some(CollisionDetection::OutOfBounds), collision);
    }
    #[test]
    fn test_detect_move_block_right() {
        let input1 = vec!["xxxxx", "xxxxx", "xxxxx", "OOOoo", "xOxii"];
        let uut: Map = input1.to_map();
        assert_eq!(uut.as_coordinates().len(), 25);
        let collision = uut.detect_move_collision(&crate::events::MoveEvent::Right);
        assert_eq!(Some(CollisionDetection::Block), collision);
    }
    #[test]
    fn test_detect_collision_block() {
        let input1 = vec!["xxxxx", "xxxxx", "xxOxx", "xOOOx", "xxzxx"];
        let uut: Map = input1.to_map();
        assert_eq!(uut.as_coordinates().len(), 25);
        let collision = uut.detect_collision();
        assert_eq!(Some(CollisionDetection::Block), collision);
    }

    #[test]
    fn test_create_tiles() {
        let c1 = Map::create_tiles(5, 5)
            .into_keys()
            .collect::<Vec<Coordinates>>();
        assert_eq!(c1.len(), 25);
        let c1 = Map::create_tiles(3, 3)
            .into_keys()
            .collect::<Vec<Coordinates>>();
        assert_eq!(c1.len(), 9);
        let c1 = Map::create_tiles(5, 10)
            .into_keys()
            .collect::<Vec<Coordinates>>();
        assert_eq!(c1.len(), 50);
    }

    #[test]
    fn test_from_str() {
        let map = Map::from_str(include_str!("map.txt"));
        assert_eq!(map.as_coordinates().len(), 220);
        assert_eq!(map.get_block_coordinates().len(), 32);
    }
    #[test]
    fn test_to_debug_file() {
        let map = Map::from_str(include_str!("map.txt"));
        assert_eq!(map.as_coordinates().len(), 220);
        assert_eq!(map.get_block_coordinates().len(), 32);
        let mut file = File::create("debug.txt").expect("file creation failed");
        file.write_all(map.to_string().as_bytes())
            .expect("writing to file failed");
    }

    #[test]
    fn test_to_tetris() {
        let map = Map::from_str(include_str!("map.txt"));
        assert_eq!(map.as_coordinates().len(), 220);
        assert_eq!(map.get_block_coordinates().len(), 32);
        let res = map.as_tetris();
        // let mut file = File::create("tetris.txt").expect("file creation failed");
        // file.write_all(res.as_bytes())
        //     .expect("writing to file failed");

        let expected = r#"|0||1||2||3||4||5||6||7||8||9|
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || ||z||z|| || || || ||s|| |
| ||o||o||z||z||l||o||o||s||s|
|j||o||o||l||l||l||o||o||t||s|
|j||j||j||i||i||i||i||t||t||t|"#;

        assert_eq!(res.as_str(), expected);
    }
    #[test]
    fn test_from_tetris() {
        let input = r#"|0||1||2||3||4||5||6||7||8||9|
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || || || || || || || || || |
| || ||z||z|| || || || ||s|| |
| ||o||o||z||z||l||o||o||s||s|
|j||o||o||l||l||l||o||o||t||s|
|j||j||j||i||i||i||i||t||t||t|"#;
        let map = Map::from_str(include_str!("map.txt"));
        let expected = String::to_map(input.to_string());
        // println!("{}", expected.to_string());
        assert_eq!(map, expected);
    }

    #[test]
    fn test_move_blocks_down() {
        let mut map =
            String::to_map(include_str!("../../test_resources/move_blocks_down.txt").to_string());
        let expected = String::to_map(
            include_str!("../../test_resources/move_blocks_down_result.txt").to_string(),
        );
        assert_eq!(map.height, 22);

        let (lines, t) = map.move_blocks_above_empty_lines().unwrap();
        assert_eq!(vec![20u16], lines);
        assert_eq!(
            vec![
                ((2, 19).into(), (2, 20).into()),
                ((3, 19).into(), (3, 20).into()),
                ((5, 19).into(), (5, 20).into()),
                ((6, 19).into(), (6, 20).into()),
                ((9, 19).into(), (9, 20).into()),
                ((3, 18).into(), (3, 19).into()),
                ((6, 18).into(), (6, 19).into()),
                ((9, 18).into(), (9, 19).into()),
                ((6, 17).into(), (6, 18).into()),
            ],
            t.0
        );
        // println!("{}", expected.to_string());
        assert_eq!(map, expected);
    }

    #[test]
    fn test_to_savegame_file() {
        let input = include_str!("map.txt");
        let map = Map::from_str(input);
        assert_eq!(map.as_coordinates().len(), 220);
        assert_eq!(map.get_block_coordinates().len(), 32);
        // let mut file = File::create("foo.txt").expect("file creation failed");
        let s = Vec::new();
        let mut file = BufWriter::new(s);
        file.write_all(map.as_savegame_string().as_bytes())
            .expect("writing to file failed");
        let bytes = file.into_inner().unwrap();
        let res = String::from_utf8(bytes).unwrap();
        assert_eq!(input, res);
    }

    #[test]
    fn test_spawn_shape() {
        let shape = Shape::blueprint(ShapeType::Z);

        let mut map = Map::new(5, 5);
        map.spawn(&shape, &(1, 1).into());
        let mut c1 = Map::create_tiles(5, 5)
            .into_keys()
            .collect::<Vec<Coordinates>>();
        c1.sort_unstable();
        assert_eq!(c1.len(), 25);

        let mut map2 = Map {
            inner: c1.into_iter().map(|c| (c, Tile::Empty)).collect(),
            width: 5,
            height: 5,
        };
        assert_eq!(map.inner[&(1, 1).into()], Tile::CurrentTetromino('z'));
        assert_eq!(
            map.inner[&(2, 1).into()],
            Tile::CurrentTetromino('z') // Tile::CurrentTetromino(Color::rgb(0.7, 0., 0.).into(), e1)
        );
        assert_eq!(map.inner[&(2, 2).into()], Tile::CurrentTetromino('z'));
        assert_eq!(map.inner[&(3, 2).into()], Tile::CurrentTetromino('z'));
        assert_eq!(map.inner[&(3, 1).into()], Tile::Empty);
        assert_eq!(map.inner[&(1, 2).into()], Tile::Empty);

        map2.inner
            .insert((1, 1).into(), Tile::CurrentTetromino('z'));
        map2.inner
            .insert((2, 1).into(), Tile::CurrentTetromino('z'));
        map2.inner
            .insert((2, 2).into(), Tile::CurrentTetromino('z'));
        map2.inner
            .insert((3, 2).into(), Tile::CurrentTetromino('z'));
        assert_eq!(map, map2);
    }

    #[test]
    fn test_fmt() {
        let mut map = Map::empty_square(5);
        map.insert(Coordinates { x: 2, y: 2 }, MapTile::CurrentTetromino('o'));
        map.insert(Coordinates { x: 3, y: 4 }, MapTile::Block('t'));
        let s = format!("{map}");
        assert_eq!(
            s,
            String::from(
                r#"------------------------------------
x(0,0)|x(1,0)|x(2,0)|x(3,0)|x(4,0)|
------------------------------------
x(0,1)|x(1,1)|x(2,1)|x(3,1)|x(4,1)|
------------------------------------
x(0,2)|x(1,2)|O(2,2)|x(3,2)|x(4,2)|
------------------------------------
x(0,3)|x(1,3)|x(2,3)|x(3,3)|x(4,3)|
------------------------------------
x(0,4)|x(1,4)|x(2,4)|t(3,4)|x(4,4)|
------------------------------------
"#
            )
        )
    }
}
