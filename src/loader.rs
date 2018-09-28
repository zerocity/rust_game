#[allow(unused_imports)]
use assets::{parse_tileset, Tile, Tileset};

use ggez::graphics::Rect;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExtendedTile {
    pub tile: Tile,
    pub src: Rect,
    pub x: i32,
    pub y: i32,
}

pub fn get_coords_of_tile_id(w: i32, h: i32, id: i32) -> (i32, i32) {
    let row = (id / w) as i32;
    let col = id % w;
    let y = row * w;
    let x = col * h;
    (x, y)
}

pub fn get_rect_src(x: i32, y: i32, image_height: i32, image_width: i32) -> Rect {
    Rect::new(
        y as f32 / 256 as f32,
        x as f32 / 256 as f32,
        16.0 / image_height as f32,
        16.0 / image_width as f32,
    )
}

pub fn group_by_tile_type(tileset: Tileset) -> HashMap<String, Vec<ExtendedTile>> {
    let mut grouped: HashMap<String, Vec<ExtendedTile>> = HashMap::new();
    let tw = tileset.tilewidth;
    let th = tileset.tileheight;
    let iw = tileset.imagewidth;
    let ih = tileset.imagewidth;
    let tiles = tileset.tiles.unwrap();

    for tile in tiles {
        if let Some(tile_type) = &tile.tile_type {
            let key = tile_type.to_string();
            match grouped.entry(key) {
                Vacant(entry) => {
                    let (x, y) = get_coords_of_tile_id(tw, th, tile.id);
                    let src = get_rect_src(x, y, ih, iw);
                    let e_tile = ExtendedTile {
                        tile: tile.clone(),
                        src,
                        x,
                        y,
                    };
                    let mut a: Vec<ExtendedTile> = vec![e_tile];
                    entry.insert(a);
                }
                Occupied(entry) => {
                    let (x, y) = get_coords_of_tile_id(tw, th, tile.id);
                    let src = get_rect_src(x, y, ih, iw);
                    let e_tile = ExtendedTile {
                        tile: tile.clone(),
                        src,
                        x,
                        y,
                    };
                    entry.into_mut().push(e_tile);
                }
            }
        }
    }
    grouped
}

#[test]
fn it_should_return_a_group_by_tile_type() {
    let path = "resources/dungeon.test.json".to_string();
    let result = group_by_tile_type(parse_tileset(path).unwrap());
    let p = result.get("player").unwrap();
    println!("{:#?}", p);
    let w = result.get("weapon").unwrap();
    let m = result.get("monster").unwrap();
    let l = result.get("layout").unwrap();
    assert_eq!(p.len(), 1);
    assert_eq!(w.len(), 2);
    assert_eq!(m.len(), 8);
    assert_eq!(l.len(), 2);
}

#[test]
fn it_should_return_coords_of_tile_id() {
    assert_eq!(get_coords_of_tile_id(16, 16, 1), (0, 0));
    assert_eq!(get_coords_of_tile_id(16, 16, 17), (0, 0));
}
