use assets::tileparser::{Properties, Tile, Tileset};
use ggez::graphics::Rect;
use std::collections::HashMap;

// REFACTOR !!!
#[derive(Debug, Clone)]
pub struct Sprite {
    pub sprite_id: i32,
    pub src: Rect,
    pub properties: Properties,
}

pub type ManagedSprite = HashMap<i32, Sprite>;

pub fn create_sprite_with_index(tileset: Tileset, firstgid: i32, sprites: &mut ManagedSprite) {
    if let Some(tiles) = tileset.clone().tiles {
        for tile in tiles.into_iter() {
            let src = calc_rect(&tileset, &tile);
            let e_tile = Sprite {
                sprite_id: tile.id,
                properties: tile.properties,
                src,
            };
            sprites.entry(tile.id + &firstgid).or_insert(e_tile);
        }
    } else {
        debug!("\ntileset: \n{:?}", tileset);
    }
}

fn calc_rect(tileset: &Tileset, tile: &Tile) -> Rect {
    let iw = tileset.imagewidth as f32;
    let ih = tileset.imageheight as f32;

    let c = tileset.columns as f32;
    let w = tileset.tilewidth as f32;
    let h = tileset.tileheight as f32;
    let tile_id = tile.id as f32;

    let rw = w / iw;
    let rh = h / ih;

    let x = (tile_id % c * w) / iw;
    let y = ((tile_id / c) as i32 * 16) as f32 / ih;

    Rect::new(x, y, rw, rh)
}

#[test]
fn name() {
    use assets::tileparser::parse_tileset;
    let path = "resources/ashlands_tileset.json";
    let tileset = parse_tileset(path.to_string()).unwrap_or_else(|_| {
        panic!("Didn't find file {}");
    });
    if let Some(tiles) = tileset.clone().tiles {
        let tile = tiles.get(0).unwrap();
        let a = calc_rect(&tileset, &tile);
        println!("--> {:#?}", a);
    }

    assert_eq!(1, 1);
}
