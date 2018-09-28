// extern crate ggez;
// extern crate serialize;
// extern crate tiled;

// use ggez::graphics::{rectangle, DrawMode, Point2, Rect};
// use ggez::graphics::{DrawMode, Point2};
// use ggez::*;

extern crate ggez_one;

// use tiled::parse;

// fn get_my_tiles() -> tiled::Map {
//     let file = File::open(&Path::new("resources/level1.tmx")).unwrap();
//     println!("Opened file");
//     let reader = BufReader::new(file);
//     let map = parse(reader).unwrap();
//     map
// }

// impl tiled::Image for ggez::graphics::Drawable {

// }

// fn draw_tile_map(ctx: &mut ggez::Context, map: &tiled::Map) {
//     // println!("{:?}", map);
//     let base_image =
//         graphics::Image::new(ctx, "resource/0x72_16x16DungeonTileset_walls.v2.png").unwrap();

//     for layer in map.layers.iter() {
//         if !layer.visible {
//             continue;
//         }

//         for (j, row) in layer.tiles.iter().enumerate() {
//             for (i, &gid) in row.iter().enumerate() {
//                 if gid == 0 {
//                     continue;
//                 }

//                 let tileset = map.get_tileset_by_gid(gid).unwrap();

//                 let gid = gid - tileset.first_gid;
//                 let ti = gid % (tileset.images[0].width as u32 / tileset.tile_width);
//                 let tj = gid / (tileset.images[0].width as u32 / tileset.tile_width);
//                 println!("{} {}", ti, tj);
//                 println!(
//                     "{:?}",
//                     Rect::new(
//                         ti as f32 * tileset.tile_width as f32 / tileset.images[0].width as f32,
//                         tj as f32 * tileset.tile_height as f32 / tileset.images[0].height as f32,
//                         tileset.tile_width as f32 / tileset.images[0].width as f32,
//                         tileset.tile_height as f32 / tileset.images[0].height as f32
//                     )
//                 );

//                 graphics::draw_ex(
//                     ctx,
//                     &base_image,
//                     graphics::DrawParam {
//                         src: Rect::new(
//                             ti as f32 * tileset.tile_width as f32 / tileset.images[0].width as f32,
//                             tj as f32 * tileset.tile_height as f32
//                                 / tileset.images[0].height as f32,
//                             tileset.tile_width as f32 / tileset.images[0].width as f32,
//                             tileset.tile_height as f32 / tileset.images[0].height as f32,
//                         ),
//                         dest: Point2::new(
//                             i as f32 * map.tile_width as f32,
//                             j as f32 * map.tile_height as f32,
//                         ),
//                         ..Default::default()
//                     },
//                 ).unwrap();
//             }
//         }
//     }
// }

// struct Level {
//     name: String,
//     x: f32,
//     y: f32,
// }

// impl Level {
//     fn new() -> Level {
//         Level {
//             name: "level1".to_string(),
//             x: 10.,
//             y: 10.,
//         }
//     }
//     // pub fn from(name: &str, x: f32, y: f32) -> Level {
//     //     Level {
//     //         name: name.to_string(),
//     //         x,
//     //         y,
//     //     }
//     // }
// }

// struct MainState {
//     // level: Level,
//     frames: usize,
//     map: tiled::Map,
// }

// impl MainState {
//     fn new(_ctx: &mut Context) -> GameResult<MainState> {
//         let s = MainState {
//             // level: Level::new(),
//             frames: 0,
//             map: get_my_tiles(),
//         };
//         Ok(s)
//     }
// }

// const GRID_X: f32 = 32.;
// const GRID_Y: f32 = 32.;
// const BORDER: f32 = 1.;

// fn draw_level(ctx: &mut ggez::Context, level: &Level) {
// let rows = level.x as i32;
// let cols = level.y as i32;

// for row in 0..rows {
//     for col in 0..cols {
//         let x = row as f32;
//         let y = col as f32;
//         let x = x + (GRID_X * x) + BORDER;
//         let y = y + (GRID_Y * y) + BORDER;

//         let r = Rect::new(x, y, 32., 32.);
//         graphics::rectangle(ctx, DrawMode::Fill, r).unwrap_or_else(|e| println!("{:?}", e))
//     }
// }
// }

// impl event::EventHandler for MainState {
//     fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
//         // self.pos_x = self.pos_x % 800.0 + 1.0;
//         Ok(())
//     }

//     fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
//         graphics::clear(ctx);
//         // draw_level(ctx, &self.level);
//         draw_tile_map(ctx, &self.map);
//         graphics::present(ctx);

//         self.frames += 1;
//         if (self.frames % 100) == 0 {
//             println!("FPS: {}", ggez::timer::get_fps(ctx));
//         }

//         Ok(())
//     }
// }

pub fn main() {
    // let c = conf::Conf::new();

    ggez_one::tileset::read_tmx("path: &str");

    // let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    // let state = &mut MainState::new(ctx).unwrap();
    // event::run(ctx, state).unwrap();
}
