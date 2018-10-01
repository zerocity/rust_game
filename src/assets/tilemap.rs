use assets::sprite::{create_sprite_with_index, ManagedSprite, Sprite};
use assets::tileparser::{
    parse_tilemap, parse_tileset, Properties, SmallTileset, Tilemap, Tileset,
};
use ggez::graphics::{Point2, Rect};
use std::collections::HashMap;
// use std::fmt;

#[derive(Debug)]
pub struct Tile {
    pub sprite_id: Option<i32>,
    pub dest: Point2,
    pub src: Rect,
    pub properties: Properties,
}

impl Tile {
    pub fn new(sprite_id: Option<i32>, dest: Point2, src: Rect) -> Self {
        Tile {
            sprite_id,
            dest,
            src,
            properties: None,
        }
    }

    pub fn set_src(&mut self, src: Rect) {
        self.src = src;
    }
    pub fn set_properties(&mut self, properties: &Properties) {
        self.properties = properties.to_owned();
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            sprite_id: None,
            dest: Point2::new(0., 0.),
            src: Rect::new(0., 0., 0., 0.),
            properties: None,
        }
    }
}

fn load_tileset(small_tileset: &SmallTileset) -> Tileset {
    let path = format!("resources/{}", small_tileset.source);
    parse_tileset(path).unwrap_or_else(|_| {
        panic!("Didn't find file {}");
    })
}

fn register_sprites_of_tile_sets(
    tilemap: &Tilemap,
    sprites: &mut ManagedSprite,
    images: &mut Vec<Image>,
) {
    for tileset in tilemap.tilesets.iter() {
        let loaded_set = load_tileset(tileset);
        images.push(Image {
            start: tileset.firstgid.to_owned(),
            src: loaded_set.image.to_string(),
        });
        create_sprite_with_index(loaded_set.clone(), tileset.firstgid, sprites);
    }
}

#[derive(Debug)]
pub struct Image {
    pub start: i32,
    pub src: String,
}

#[derive(Debug)]
pub struct TilemapManager {
    tilemap: Tilemap,
    sprites: ManagedSprite,
    pub images: Vec<Image>,
}

impl TilemapManager {
    pub fn new(path: &str) -> Self {
        // let path = "resources/lvl1.json";
        let tilemap = parse_tilemap(path).unwrap_or_else(|_| {
            panic!("Didn't find file {}", path);
        });

        let mut sprites: ManagedSprite = HashMap::new();

        let mut images: Vec<Image> = Vec::new();
        register_sprites_of_tile_sets(&tilemap, &mut sprites, &mut images);

        TilemapManager {
            tilemap,
            sprites,
            images,
        }
    }

    pub fn get_image_by_id(&self, id: &i32) -> Option<String> {
        for (index, image) in self.images.iter().enumerate() {
            if let Some(end) = self.images.get(index + 1) {
                if id >= &image.start && id < &end.start {
                    return Some(image.src.to_string());
                }
            } else {
                return Some(image.src.to_string());
            }
        }
        None
    }

    pub fn get_grid(&self) -> Vec<Tile> {
        // pub fn build(&self, e: &World) {
        // let height = self.level.height;
        let width = self.tilemap.width;
        let tile_w = self.tilemap.tilewidth;
        let tile_h = self.tilemap.tileheight;

        let mut map: Vec<Tile> = Vec::new();

        // todo first only the first layer
        let layers = self.tilemap.layers.iter();
        for layer in layers {
            for (i, tile) in layer.data.iter().enumerate() {
                let mut t = Tile::default();
                t.sprite_id = Option::Some(tile.to_owned());
                let index = i as i32;
                let row = (index / width) as i32;
                let col = index % width;

                t.dest.y = (row * tile_w) as f32;
                t.dest.x = (col * tile_h) as f32;
                let s = self.get_sprite_by_id(&tile);

                if let Some(s) = s {
                    t.set_src(s.src);
                    t.set_properties(&s.properties);
                }

                map.push(t);
            }
        }

        map
    }
    // get_sprite_by_ids
    pub fn get_sprite_by_id(&self, id: &i32) -> Option<&Sprite> {
        self.sprites.get(id)
    }
}
