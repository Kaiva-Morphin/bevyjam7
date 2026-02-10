use bevy::math::vec3;
use crate::prelude::*;


pub const HAND_IN_ANIMATION_DURATION : f32 = 0.25;
pub const HAND_OUT_ANIMATION_DURATION : f32 = 0.2;



pub const MIAMI_SHADOW_OFFSET : Vec3 = vec3(2.0, -2.0, -0.5);
pub fn miami_shadow_color() -> Color {
    Color::Srgba(Srgba::rgba_u8(0, 0, 0, 218))    
}
