use assets::tileparser::Tileset;
use ggez::graphics::Rect;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Sprite {
    pub sprite_id: i32,
    pub src: Rect,
}

pub type ManagedSprite = HashMap<i32, Sprite>;

fn get_coords_of_tile_id(w: i32, h: i32, id: i32) -> (i32, i32) {
    let row = (id / w) as i32;
    let col = id % w;
    let x = row * w;
    let y = col * h;
    (x, y)
}

fn get_rect_src(x: i32, y: i32, image_height: i32, image_width: i32) -> Rect {
    // let a = Rect::new(y as f32, x as f32, 16 as f32, 16 as f32);
    // println!("{} {}", x, y);
    let a = Rect::new(
        y as f32 / (image_height) as f32,
        x as f32 / (image_width) as f32,
        16.0 / (image_height) as f32,
        16.0 / (image_width) as f32,
    );
    // println!("{:?}", a);
    a
}

pub fn create_sprite_with_index(tileset: Tileset, firstgid: i32, sprites: &mut ManagedSprite) {
    let tw = tileset.tilewidth;
    let th = tileset.tileheight;
    let iw = tileset.imagewidth;
    let ih = tileset.imagewidth;

    if let Some(tiles) = tileset.tiles {
        for tile in tiles.into_iter() {
            let (x, y) = get_coords_of_tile_id(tw, th, tile.id);
            let src = get_rect_src(x, y, ih, iw);

            let e_tile = Sprite {
                sprite_id: tile.id,
                src,
            };
            sprites.entry(tile.id + &firstgid).or_insert(e_tile);
        }
    } else {
        debug!("\ntileset: \n{:?}", tileset);
    }
}
