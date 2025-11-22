use bevy::math::{vec2, vec3, vec4, Vec2, Vec3, Vec4, Vec2Swizzles, Vec4Swizzles};
use serde::{Serialize, Deserialize};


#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct AABB {
    pub min_x: f32,
    pub max_x: f32,
    pub min_z: f32,
    pub max_z: f32,
}

impl Default for AABB {
    fn default() -> Self {
        return AABB{
            min_x: 0.0, 
            max_x: 0.0,
            min_z: 0.0, 
            max_z: 0.0
        };
    }
}

impl AABB {
    pub fn from_loc_dims(loc: Vec2, dim: Vec2) -> AABB {
        AABB {
            min_x: loc.x - dim.x / 2.0,
            max_x: loc.x + dim.x / 2.0,
            min_z: loc.y - dim.y / 2.0,
            max_z: loc.y + dim.y / 2.0,
        }
    }

    pub fn has_point(&self, loc: Vec2) -> bool {
        loc.x >= self.min_x && loc.x <= self.max_x && loc.y >= self.min_z && loc.y <= self.max_z
    }

}