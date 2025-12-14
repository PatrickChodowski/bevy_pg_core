use bevy::prelude::{Component, Reflect};
use bevy::math::Vec2;
use std::cmp::Ordering;


#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone, Reflect)]

pub struct Tile {
    pub x: usize,
    pub y: usize
}
impl Tile {
    pub fn new(x: usize, y: usize) -> Self {
        return Tile{x,y}
    }
    pub fn try_add(&self, x: isize, y: isize) -> Option<Tile> {
        let maybe_x: isize = self.x as isize + x;
        let maybe_y: isize = self.y as isize + y;

        if (maybe_x >= 0) & (maybe_y >= 0){
            return Some(Tile::new(maybe_x as usize, maybe_y as usize))
        } else {
            return None;
        }

    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then(other.y.cmp(&self.y))
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Component, Reflect, Debug)]
pub struct TerrainChunk {
    pub loc:  Vec2,
    pub tile: Tile,
    pub dims: Vec2,
    pub map_name: String,
    pub chunk_id: String,
}
