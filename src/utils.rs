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
    pub fn new_min_max() -> Self {
        AABB {
            min_x: f32::MAX,
            min_z: f32::MAX,
            max_x: f32::MIN,
            max_z: f32::MIN
        }
    }
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




pub fn rotate_point_2d(p: &Vec3, origin: &Vec2, angle_y: f32) -> Vec3 {

    // translate point back to origin:
    let p2x = p.x - origin.x;
    let p2z = p.z - origin.y;
  
    let x_new = p2x * angle_y.cos() - p2z * angle_y.sin();
    let z_new = p2x * angle_y.sin() + p2z * angle_y.cos();
    
    // translate point back:
    let p3x = x_new + origin.x;
    let p3z = z_new + origin.y;
    return Vec3::new(p3x, p.y, p3z);
  }
