use ggez;
use ggez::graphics;
use ggez_goodies::scene;
use specs::{self, Join};
use warmy;

use components as c;
use input;
use resources;
use scenes::*;
use systems::*;
use world::World;

use assets::{parse_tileset, Tileset};
use loader::{Sprite, TileManager};

pub struct LevelScene {
    done: bool,
    player: Sprite,
    image: warmy::Res<resources::Image>,
    dispatcher: specs::Dispatcher<'static, 'static>,
}

fn get_dungeon() -> Tileset {
    let path = "resources/dungeon.json".to_string();
    parse_tileset(path).unwrap_or_else(|_| {
        panic!("Didn't find file");
    })
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;

        let tile_manager = TileManager::new(get_dungeon());
        let player = tile_manager.by_id(132).unwrap().to_owned();

        let image = world
            .assets
            .get::<_, resources::Image>(&warmy::FSKey::new(&tile_manager.image), ctx)
            .unwrap();
        let dispatcher = Self::register_systems();

        LevelScene {
            done,
            player,
            image,
            dispatcher,
        }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        specs::DispatcherBuilder::new()
            .with(MovementSystem, "sys_movement", &[])
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
        let pos = gameworld.specs_world.read_storage::<c::Position>();

        for p in pos.join() {
            // graphics::draw(ctx, &(self.kiwi.borrow().0), p.0, 0.0)?;
            // println!("{:?}", p  );
            graphics::draw_ex(
                ctx,
                &(self.image.borrow().0),
                graphics::DrawParam {
                    src: self.player.src,
                    scale: graphics::Point2::new(4.0, 4.0),
                    // dest: graphics::Point2::new(100., 100.),
                    dest: p.0,
                    // dest: p.Point2,
                    ..Default::default()
                },
            ).unwrap();
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
