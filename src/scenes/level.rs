use ggez;
// use ggez::graphics;
use components as c;
use ggez::graphics::{draw_ex, DrawParam, Point2, Vector2};
use ggez_goodies::scene;
use specs::{self, Join};
use std::collections::HashMap;
use warmy;
// use ggez_goodies::input::InputEffect;
use assets::tilemap::TilemapManager;
use scenes::*;
use setup::{input, resources};
use specs::Builder;
use systems;
use world::World;

pub struct LevelScene {
    done: bool,
    reg_images: HashMap<String, warmy::Res<resources::Image>>,
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;
        let tile_manager = TilemapManager::new("resources/lvl1.json");

        // Create World Entities
        for tile in tile_manager.get_grid().iter() {
            if let Some(id) = tile.sprite_id {
                if id > 0 {
                    if let Some(id) = &tile.sprite_id {
                        let image = tile_manager.get_image_by_id(id);
                        if let Some(image) = image {
                            world
                                .specs_world
                                .create_entity()
                                .with(c::Render {
                                    src: tile.src,
                                    image,
                                }).with(c::Position(tile.dest))
                                .build();
                        }
                    }
                }
            }
        }

        // CREATE Player Entity
        let player = tile_manager.by_id(&389).unwrap().to_owned();
        let p_images = tile_manager.get_image_by_id(&389);
        if let Some(p_images) = p_images {
            world
                .specs_world
                .create_entity()
                .with(c::Render {
                    src: player.src,
                    image: p_images,
                }).with(c::Position(Point2::new(0.0, 0.0)))
                .with(c::Controllable)
                .with(c::Motion {
                    velocity: Vector2::new(0.0, 0.0),
                    acceleration: Vector2::new(0.0, 0.0),
                }).build();
        }

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
        let scale = 3.5 as f32;
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
