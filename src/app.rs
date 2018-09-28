use ggez::graphics::DrawParam;
use ggez::*;

use assets::{parse_tileset, Tileset};
use loader::{ExtendedTile, TileManager};
use std::env;
use std::path;

struct MainState {
    pos_x: f32,
    // tilemap: HashMap<String, Tilemap>,
    // images: HashMap<String, graphics::Image>,
    image: graphics::Image,
    player: ExtendedTile,
}

fn get_dungeon() -> Tileset {
    let path = "resources/dungeon.json".to_string();
    parse_tileset(path).unwrap_or_else(|_| {
        panic!("Didn't find file");
    })
}

//

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let tile_manager = TileManager::new(get_dungeon());
        // should be managed via TileManager
        let image = graphics::Image::new(ctx, format!("/{}", &tile_manager.image)).unwrap();

        let player = tile_manager
            .get_tiles_by_type("player")
            .unwrap() // this is for mutible tile sets 
            .get(0)
            .unwrap()
            .to_owned(); // wtf ?? what is that
        Ok(MainState {
            pos_x: 0.0,
            image,
            player,
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let dest_point = graphics::Point2::new(self.pos_x, 100.0);
        let img = &self.image;

        graphics::draw_ex(
            ctx,
            img,
            DrawParam {
                src: self.player.src,
                scale: graphics::Point2::new(2.0, 2.0),
                dest: dest_point,
                ..Default::default()
            },
        ).unwrap();

        graphics::draw_ex(
            ctx,
            img,
            DrawParam {
                src: self.player.src,
                scale: graphics::Point2::new(2.0, 2.0),
                dest: graphics::Point2::new(255.0, self.pos_x),
                ..Default::default()
            },
        ).unwrap();
        graphics::present(ctx);
        Ok(())
    }
}

pub fn start() {
    let mut cb = ContextBuilder::new("my_game_name", "zerocity");

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }

    let ctx = &mut cb.build().unwrap();

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
