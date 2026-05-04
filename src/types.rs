use bevy::ecs::component::Component;
use bevy::reflect::Reflect;
use bevy::math::{Vec3, Vec2};
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};



#[derive(Hash, Debug, Eq, PartialEq, Copy, Clone, Reflect, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
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

impl From<Tile> for String {
    fn from(tile: Tile) -> String {
        format!("{},{}", tile.x, tile.y)
    }
}

impl TryFrom<String> for Tile {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        // Split the string at the comma
        let (x_str, y_str) = s
            .split_once(',')
            .ok_or_else(|| format!("Expected format 'x,y', found '{}'", s))?;

        // Parse both sides into usize
        let x = x_str.parse::<usize>().map_err(|_| format!("Invalid x coordinate in '{}'", s))?;
        let y = y_str.parse::<usize>().map_err(|_| format!("Invalid y coordinate in '{}'", s))?;

        Ok(Tile { x, y })
    }
}

#[derive(Component, Reflect, Debug)]
pub struct TerrainChunk {
    pub dims: Vec2
    // pub loc:  Vec3,
    // pub tile: Tile,
    // pub dims: Vec2,
    // pub name: String
}


#[derive(Component, Debug, Reflect)]

pub struct WaterChunk{
    pub dims: Vec2,
    pub just_spawned: bool // Jesus christ, to not update dims on the very first spawn ehh
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub enum EditorAsset {
    Asset(String),
    Spawner (String),
    Marker(String),
    Water
}