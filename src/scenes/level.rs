use ggez;
// use ggez::graphics;
use components as c;
use ggez::graphics::{draw_ex, DrawParam, Point2};
use ggez_goodies::scene;
use specs::{self, Join};
use std::collections::HashMap;
use warmy;
// use ggez_goodies::input::InputEffect;

use assets::tilemap::TilemapManager;
use scenes::*;
use setup::{input, resources};
use systems;
use world::World;

use assets::entity_factory;

pub struct LevelScene {
    done: bool,
    reg_images: HashMap<String, warmy::Res<resources::Image>>,
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;
        let tile_manager = TilemapManager::new("resources/lvl2.json");

        entity_factory::create_map(&tile_manager, world);
        entity_factory::create_player(&tile_manager, world);

        // Register images
        let mut reg_images: HashMap<String, warmy::Res<resources::Image>> = HashMap::new();
        for img in tile_manager.images.into_iter() {
            let image = world
                .assets
                .get::<_, resources::Image>(&warmy::FSKey::new(&img.src), ctx)
                .unwrap();
            reg_images.entry(img.src).or_insert(image);
        }

        let dispatcher = Self::register_systems();

        LevelScene {
            done,
            reg_images,
            dispatcher,
        }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        specs::DispatcherBuilder::new()
            .with(systems::player::PlayerMovementSystem, "sys_movement", &[])
            .build()
    }
}

impl scene::Scene<World, input::InputEvent> for LevelScene {
    fn update(&mut self, gameworld: &mut World) -> FSceneSwitch {
        self.dispatcher.dispatch(&mut gameworld.specs_world.res);
        if self.done {
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let scale = 3 as f32;
        assert_ne!(scale, 0 as f32);

        // Get Components for rendering
        let pos = gameworld.specs_world.read_storage::<c::Position>();
        let render = gameworld.specs_world.read_storage::<c::Render>();

        for (p, r) in (&pos, &render).join() {
            let img = self.reg_images.get(&r.image);
            if let Some(img) = img {
                draw_ex(
                    ctx,
                    &(img.borrow().0),
                    DrawParam {
                        src: r.src,
                        scale: Point2::new(scale, scale),
                        dest: p.0 * scale,
                        ..Default::default()
                    },
                ).unwrap();
            }
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "LevelScene"
    }

    fn input(&mut self, gameworld: &mut World, ev: input::InputEvent, _started: bool) {
        debug!("Input: {:?}", ev);
        if gameworld.input.get_button_pressed(input::Button::Menu) {
            self.done = true;
        }
    }
}
