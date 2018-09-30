use ggez::graphics::{Point2, Rect, Vector2};
use specs::HashMapStorage;
use specs::*;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Render {
    pub src: Rect,
    pub image: String,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Position(pub Point2);

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    sprite_id: i32,
    src: Rect,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Motion {
    pub velocity: Vector2,
    pub acceleration: Vector2,
}

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Controllable;
