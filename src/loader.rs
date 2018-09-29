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

#[derive(Debug, Clone)]
pub struct Sprite {
    pub sprite_id: i32,
    pub src: Rect,
    // pub x: i32,
    // pub y: i32,
}

type ManagedTileSet = HashMap<String, Vec<ExtendedTile>>;

pub struct TileManager {
    // set of tiles
    pub tileset_name: String,
    pub image: String,
    pub sprite: Vec<Sprite>, // should be an hashmap key : id v: sprite
    map: HashMap<String, ManagedTileSet>,
}

impl TileManager {
    pub fn new(tileset: Tileset) -> TileManager {
        let result = group_by_tile_type(tileset.clone());
        let sprite = add_rect_src_on_tiles(tileset.clone());
        let mut map: HashMap<String, ManagedTileSet> = HashMap::new();
        map.insert(tileset.name.to_string(), result);
        TileManager {
            map,
            sprite,
            // broken ....
            image: tileset.image,
            tileset_name: tileset.name,
        }
    }

    // pub fn get_tile_by_id(&self, id: i32) -> Option<ExtendedTile> {
    //     self.tiles.get(id)
    // }

    pub fn get_tiles_by_type(&self, tile_type: &str) -> Option<&Vec<ExtendedTile>> {
        self.map.get(&self.tileset_name).unwrap().get(tile_type)
    }

    pub fn get(&self) -> Option<&ManagedTileSet> {
        self.map.get(&self.tileset_name)
    }

    pub fn get_tileset(&self, tile_type: &str) -> Option<&ManagedTileSet> {
        self.map.get(&tile_type.to_string())
    }

    pub fn set_tileset(&mut self, tile_type: &str) {
        // TODO check for key !!
        self.tileset_name = tile_type.to_string();
    }

    pub fn add_tileset(&mut self, tileset: Tileset) {
        let result = group_by_tile_type(tileset.clone());
        self.map.insert(tileset.name, result);
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

pub fn add_rect_src_on_tiles(tileset: Tileset) -> Vec<Sprite> {
    let tw = tileset.tilewidth;
    let th = tileset.tileheight;
    let iw = tileset.imagewidth;
    let ih = tileset.imagewidth;
    let tiles = tileset.tiles.unwrap();
    let mut v: Vec<Sprite> = Vec::new();

    for tile in tiles.into_iter() {
        let (x, y) = get_coords_of_tile_id(tw, th, tile.id);
        let src = get_rect_src(x, y, ih, iw);

        let e_tile = Sprite {
            sprite_id: tile.id,
            src,
        };
        v.push(e_tile);
    }
    v
}

#[test]
fn it_should_add_rect_src_on_tiles() {
    let path = "resources/dungeon.test.json".to_string();
    let result = add_rect_src_on_tiles(parse_tileset(path).unwrap());
    assert_eq!(1, 1);
}

fn group_by_tile_type(tileset: Tileset) -> HashMap<String, Vec<ExtendedTile>> {
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
                    let mut v: Vec<ExtendedTile> = Vec::new();
                    v.push(e_tile);
                    entry.insert(v);
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
    // .clone()
    grouped
}

// fn get_level() -> Tilemap {
//     let path = "resources/lvl1.json";
//     parse_tilemap(path).unwrap_or_else(|_| {
//         panic!("Didn't find file");
//     })
// }

// fn get_tilesets(sets: &Vec<SmallTileset>) -> Vec<Tileset> {
//     sets.into_iter()
//         .map(|set: &SmallTileset| {
//             let path = format!("resources/{}", set.source);
//             let tile = parse_tileset(path).unwrap();
//             tile
//         }).collect()
// }

// use ggez::graphics::{DrawMode, Point2};
// use assets::{parse_tilemap, parse_tileset, SmallTileset, Tilemap, Tileset};
// use std::collections::HashMap;

#[test]
fn it_should_return_a_group_by_tile_type() {
    let path = "resources/dungeon.test.json".to_string();
    let result = group_by_tile_type(parse_tileset(path).unwrap());
    let p = result.get("player").unwrap();

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

#[test]
fn it_should_create_an_tile_manager() {
    let path = "resources/dungeon.test.json".to_string();
    let tileset = parse_tileset(path).unwrap();
    let t = TileManager::new(tileset);
    let p = t.get_tiles_by_type("player").unwrap();
    println!("{:#?}", p);
    assert_eq!(p.len(), 1);
}
