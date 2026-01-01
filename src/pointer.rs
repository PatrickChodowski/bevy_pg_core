
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hoverables {
    Navmesh(Entity), // Navmesh entity
    Target(Entity),  // Some object entity
    UI,
    // Other, Non walking target entity
    None
}

#[derive(Resource, Debug)]
pub struct PointerData {
    pub cursor_pos:              Option<Vec2>,
    pub center_screen_world_pos: Option<Vec3>, // For Editor
    pub world_pos:               Option<Vec3>,
    pub hoverable:               Hoverables
}

impl PointerData {
    pub fn new() -> PointerData {
        return Self {
            center_screen_world_pos: None,
            cursor_pos: None,
            world_pos: None,
            hoverable: Hoverables::None,
        };
    }
    pub fn reset(&mut self) {
        *self = PointerData::new();
    }
    pub fn get_hover_entity(&self) -> Option<Entity> {
        if let Hoverables::Target(entity) = self.hoverable {
            return Some(entity)
        } else {
            return None;
        }
    }
    pub fn get_hover_navmesh(&self) -> Option<Entity> {
        if let Hoverables::Navmesh(entity) = self.hoverable {
            return Some(entity)
        } else {
            return None;
        }
    }
}