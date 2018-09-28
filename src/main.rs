extern crate game;

use game::app;
// use game::assets;
// use game::assets::Tilemap;

// fn get_level() -> Tilemap {
//     let path = "resources/lvl1.json";
//     assets::parse_tilemap(path).unwrap_or_else(|_| {
//         panic!("Didn't find file");
//     })
// }

fn main() {
    // let a = get_level();
    app::start();
}
