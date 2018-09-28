// use ggez::graphics::{DrawMode, Point2};
use ggez::graphics::{DrawParam, Rect};
use ggez::*;

use assets::{parse_tilemap, parse_tileset, SmallTileset, Tilemap, Tileset};
use std::collections::HashMap;
use std::env;
// use std::io;
use std::path;

struct MainState {
    pos_x: f32,
    // tilemap: HashMap<String, Tilemap>,
    // images: HashMap<String, graphics::Image>,
    image: graphics::Image,
}

fn get_level() -> Tilemap {
    let path = "resources/lvl1.json";
    parse_tilemap(path).unwrap_or_else(|_| {
        panic!("Didn't find file");
    })
}

fn get_tilesets(sets: &Vec<SmallTileset>) -> Vec<Tileset> {
    sets.into_iter()
        .map(|set: &SmallTileset| {
            let path = format!("resources/{}", set.source);
            let tile = parse_tileset(path).unwrap();
            tile
        }).collect()
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let l = get_level();
        let tiles = get_tilesets(&l.tilesets);
        let mut lvl = HashMap::new();
        lvl.insert("level_one".to_string(), l);

        let v = tiles.get(1).unwrap();

        // let a = &mut ctx.filesystem.get_resources_dir().join(&v.image);
        // let image = graphics::Image::new(ctx, "/0x72_16x16DungeonTileset_walls.v2.png").unwrap();

        let image = graphics::Image::new(ctx, format!("/{}", &v.image)).unwrap();

        // let image = graphics::Image::new(ctx, a).unwrap();

        let s = MainState {
            pos_x: 0.0,
            // tilemap: lvl,
            image,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let dest_point = graphics::Point2::new(100.0, 100.0);
        let img = &self.image;

        let bac = Rect::new(
            0.25, // row
            0.5,  // col
            0.0625, 0.0625,
        );

        // println!("{:#?}", bac);

        graphics::draw_ex(
            ctx,
            img,
            DrawParam {
                src: bac,
                scale: graphics::Point2::new(2.0, 2.0),
                dest: dest_point,
                ..Default::default()
            },
        ).unwrap();

        graphics::draw_ex(
            ctx,
            img,
            DrawParam {
                src: bac,
                scale: graphics::Point2::new(2.0, 2.0),
                dest: graphics::Point2::new(255.0, 100.0),
                ..Default::default()
            },
        ).unwrap();

        // graphics::draw(ctx, img, dest_point, 0.0)?;

        // graphics::circle(
        //     ctx,
        //     DrawMode::Fill,
        //     Point2::new(self.pos_x, 380.0),
        //     100.0,
        //     2.0,
        // )?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn start() {
    // let mut cb = ContextBuilder::new("my_game_name", "zerocity");

    // if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    //     let mut path = path::PathBuf::from(manifest_dir);
    //     path.push("resources");
    //     println!("Adding path {:?}", path);
    //     cb = cb.add_resource_path(path);
    // }
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("imageview", "ggez", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    // let ctx = &mut ctx.build().unwrap();

    match MainState::new(ctx) {
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
        Ok(ref mut game) => {
            let result = event::run(ctx, game);
            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            } else {
                println!("Game exited cleanly.");
            }
        }
    }
}
