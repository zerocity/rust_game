//use ggez::nalgebra as na;
// use ggez::graphics::*;
use ggez::graphics::{Drawable, Point2, Rect, Vector2};
use specs::*;

#[derive(Clone, Debug)]
pub struct Render {
    pub src: Rect,
    // dest: Point2,
}

impl Component for Render {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug)]
pub struct Position(pub Point2);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug)]
pub struct Sprite {
    // id of spite
    sprite_id: i32,
    src: Rect,
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug)]
pub struct Motion {
    pub velocity: Vector2,
    pub acceleration: Vector2,
}

impl Component for Motion {
    type Storage = VecStorage<Self>;
}

// Just a marker that a particular entity is the player.
#[derive(Clone, Debug, Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(Clone, Debug, Default)]
pub struct Shot {
    pub damage: u32,
}

impl Component for Shot {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug)]
pub struct CBackgroundScroller {
    pub scroll_speed: Vector2,
}

impl Component for CBackgroundScroller {
    type Storage = HashMapStorage<Self>;
}
