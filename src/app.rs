use ggez::*;
use logger;
use std::path;

use ggez::event::*;
use keyboard;
use scenes;
use world;

struct MainState {
    scenes: scenes::FSceneStack,
}

impl MainState {
    fn new(resource_dir: Option<path::PathBuf>, ctx: &mut Context) -> GameResult<MainState> {
        let world = world::World::new(ctx, resource_dir.clone());
        let mut scenestack = scenes::FSceneStack::new(ctx, world);
        let initial_scene = Box::new(scenes::level::LevelScene::new(ctx, &mut scenestack.world));
        scenestack.push(initial_scene);

        Ok(MainState { scenes: scenestack })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.scenes.update();
        }
        self.scenes.world.assets.sync(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.scenes.draw(ctx);
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        let mut keyboard = self
            .scenes
            .world
            .specs_world
            .write_resource::<keyboard::Keyboard>();

        keyboard.0.insert(keycode, true);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut keyboard = self
            .scenes
            .world
            .specs_world
            .write_resource::<keyboard::Keyboard>();
        keyboard.0.insert(keycode, false);
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
