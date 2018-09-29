use ggez::*;
use logger;
use std::path;

use ggez::event::*;
use input;
use scenes;
use world;

struct MainState {
    scenes: scenes::FSceneStack,
    input_binding: input::InputBinding,
}

impl MainState {
    fn new(resource_dir: Option<path::PathBuf>, ctx: &mut Context) -> GameResult<MainState> {
        let world = world::World::new(ctx, resource_dir.clone());
        let mut scenestack = scenes::FSceneStack::new(ctx, world);
        let initial_scene = Box::new(scenes::level::LevelScene::new(ctx, &mut scenestack.world));
        scenestack.push(initial_scene);

        Ok(MainState {
            scenes: scenestack,
            input_binding: input::create_input_binding(),
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.scenes.update();
        }
        self.scenes.world.assets.sync(ctx);
        // self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.scenes.draw(ctx);
        graphics::present(ctx);
        Ok(())

        // graphics::clear(ctx);
        // let dest_point = graphics::Point2::new(self.pos_x, 100.0);
        // let img = &self.image;

        // graphics::draw_ex(
        //     ctx,
        //     img,
        //     DrawParam {
        //         src: self.player.src,
        //         scale: graphics::Point2::new(2.0, 2.0),
        //         dest: dest_point,
        //         ..Default::default()
        //     },
        // ).unwrap();

        // graphics::draw_ex(
        //     ctx,
        //     img,
        //     DrawParam {
        //         src: self.player.src,
        //         scale: graphics::Point2::new(2.0, 2.0),
        //         dest: graphics::Point2::new(255.0, self.pos_x),
        //         ..Default::default()
        //     },
        // ).unwrap();
        // graphics::present(ctx);
        // Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.scenes.input(ev, true);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.scenes.input(ev, false);
        }
    }
}

pub fn start() {
    logger::setup_logger().expect("Could not set up logging!");
    let mut cb = ContextBuilder::new("my_game_name", "zerocity");

    let cargo_path: Option<path::PathBuf> = option_env!("CARGO_MANIFEST_DIR").map(|env_path| {
        let mut res_path = path::PathBuf::from(env_path);
        res_path.push("resources");
        res_path
    });
    // If we have such a path then add it to the context builder too
    // (modifying the cb from inside a closure gets sticky)
    if let Some(ref s) = cargo_path {
        cb = cb.add_resource_path(s);
    }

    let ctx = &mut cb.build().unwrap();

    match MainState::new(cargo_path, ctx) {
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
