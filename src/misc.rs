use bevy::math::Vec2;

pub fn is_in_circle(p1: Vec2, p2: Vec2, r: f32) -> bool {
    (p2.x - r < p1.x && p1.x < p2.x + r) && (p2.y - r < p1.y && p1.y < p2.y + r)
}
