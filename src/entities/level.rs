use assets::sprite::{ManagedSprite, Sprite, TileManager};
use assets::tileparser::Tilemap;
use ggez::graphics::{Point2, Rect};
// use specs::world::EntityBuilder;
use components as c;
use specs::World;
use std::fmt;

pub struct LevelMap {
    pub level: Tilemap,
    pub manager: TileManager,
}

pub struct Tile {
    pub sprite_id: Option<i32>,
    pub dest: Point2,
    pub src: Rect,
}

impl Tile {
    pub fn new(sprite_id: Option<i32>, dest: Point2, src: Rect) -> Self {
        Tile {
            sprite_id,
            dest,
            src,
        }
    }

    pub fn set_src(&mut self, src: Rect) {
        self.src = src;
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            sprite_id: None,
            dest: Point2::new(0., 0.),
            src: Rect::new(0., 0., 0., 0.),
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tile({:?}) \n  dest: ({}, {}) \n  src : (x {}, y {}, w {}, h {}) ",
            self.sprite_id,
            self.dest.x,
            self.dest.y,
            self.src.x,
            self.src.y,
            self.src.w,
            self.src.h
        )
    }
}

// impl std::format::Display for Tile {}

impl LevelMap {
    pub fn new(level: Tilemap, manager: TileManager) -> Self {
        LevelMap { level, manager }
    }

    pub fn get_grid(&self) -> Vec<Tile> {
        // pub fn build(&self, e: &World) {
        // let height = self.level.height;
        let width = self.level.width;
        let tile_w = self.level.tilewidth;
        let tile_h = self.level.tileheight;

        let mut map: Vec<Tile> = Vec::new();

        // todo first only the first layer

        let layer = self.level.layers.first().unwrap();

        for (i, tile) in layer.data.iter().enumerate() {
            let mut t = Tile::default();
            t.sprite_id = Option::Some(tile.to_owned());
            let index = i as i32;
            let row = (index / width) as i32;
            let col = index % width;

            t.dest.y = (row * tile_w) as f32;
            t.dest.x = (col * tile_h) as f32;
            let s = self.manager.by_id(tile.to_owned() - 257);

            if let Some(s) = s {
                t.set_src(s.src);
            }
            map.push(t);
        }
        map
    }
}
