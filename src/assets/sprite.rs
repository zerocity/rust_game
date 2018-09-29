use assets::tileparser::{Tile, Tileset};
use ggez::graphics::Rect;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExtendedTile {
    pub tile: Tile,
    pub src: Rect,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub sprite_id: i32,
    pub src: Rect,
}

pub type ManagedSprite = HashMap<i32, Sprite>;

#[derive(Debug, Clone)]
pub struct TileManager {
    // set of tiles
    pub tileset_name: String,
    pub image: String,
    // save parent
    pub sprite: ManagedSprite,
}

impl TileManager {
    pub fn new(tileset: Tileset) -> TileManager {
        let sprite = create_sprite_with_index(tileset.clone());
        TileManager {
            sprite,
            image: tileset.image,
            // images: tileset.image,
            tileset_name: tileset.name,
        }
    }

    pub fn by_id(&self, id: i32) -> Option<&Sprite> {
        self.sprite.get(&id)
        // .unwrap().to_owned()
    }
}

fn get_coords_of_tile_id(w: i32, h: i32, id: i32) -> (i32, i32) {
    let row = (id / w) as i32;
    let col = id % w;
    let x = row * w;
    let y = col * h;
    (x, y)
}

fn get_rect_src(x: i32, y: i32, image_height: i32, image_width: i32) -> Rect {
    Rect::new(
        y as f32 / 256 as f32,
        x as f32 / 256 as f32,
        16.0 / image_height as f32,
        16.0 / image_width as f32,
    )
}

pub fn create_sprite_with_index(tileset: Tileset) -> ManagedSprite {
    let mut sprites: ManagedSprite = HashMap::new();
    let tw = tileset.tilewidth;
    let th = tileset.tileheight;
    let iw = tileset.imagewidth;
    let ih = tileset.imagewidth;
    let tiles = tileset.tiles.unwrap();

    for tile in tiles.into_iter() {
        let (x, y) = get_coords_of_tile_id(tw, th, tile.id);
        let src = get_rect_src(x, y, ih, iw);

        let e_tile = Sprite {
            sprite_id: tile.id,
            src,
        };
        sprites.entry(tile.id).or_insert(e_tile);
        // v.push(e_tile);
    }
    sprites
}

#[test]
fn it_should_be_an_hashmap_with_sprite_key_is_sprite_id() {
    let a: u32 = 307 << 16;
    let b = a as i32;
    println!("{}", b);
    use assets::tileparser::parse_tileset;
    let path = "resources/dungeon.test.json".to_string();
    let tileset = parse_tileset(path).unwrap();
    let sprites = TileManager::new(tileset);
    let sprite = sprites.by_id(10).unwrap().to_owned();
    assert_eq!(sprite.sprite_id, 10);
}
