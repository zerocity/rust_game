use ggez;
// use ggez::graphics;
use ggez::graphics::{draw_ex, DrawParam, Point2, Vector2};
use ggez_goodies::scene;
use specs::{self, Join};
use warmy;

use components as c;
// use ggez_goodies::input::InputEffect;
use scenes::*;
use setup::{input, resources};
use systems;
use world::World;

// use entities::level::Tile;

use assets::sprite::TileManager;
use assets::tileparser::{parse_tilemap, parse_tileset, Tilemap, Tileset};
use entities::level::{LevelMap, Tile};
use specs::Builder;

pub struct LevelScene {
    done: bool,
    image: warmy::Res<resources::Image>,
    dispatcher: specs::Dispatcher<'static, 'static>,
}

fn get_dungeon() -> Tileset {
    let path = "resources/dungeon.json".to_string();
    parse_tileset(path).unwrap_or_else(|_| {
        panic!("Didn't find file");
    })
}

fn get_lvl() -> Tilemap {
    let path = "resources/lvl1.json";
    parse_tilemap(path).unwrap_or_else(|_| {
        panic!("Didn't find file");
    })
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;
        let tilemap = get_lvl();
        let tileset = get_dungeon();

        let tile_manager = TileManager::new(tileset);
        let level = LevelMap::new(tilemap, tile_manager.clone());
        let player = tile_manager.by_id(132).unwrap().to_owned();

        // let first = tile_manager.by_id(115).unwrap().to_owned();

        // let mut t = Tile::default();

        // t.dest = Point2::new(100.0, 100.0);
        // t.set_src(first.src);

        // world
        //     .specs_world
        //     .create_entity()
        //     .with(c::Render { src: t.src })
        //     .with(c::Position(t.dest))
        //     .build();

        for tile in level.get_grid().iter() {
            if let Some(id) = tile.sprite_id {
                if id > 0 {
                    world
                        .specs_world
                        .create_entity()
                        .with(c::Render { src: tile.src })
                        .with(c::Position(tile.dest))
                        .build();
                }
            }
        }

        world
            .specs_world
            .create_entity()
            .with(c::Render { src: player.src })
            .with(c::Position(Point2::new(0.0, 0.0)))
            .with(c::Controllable)
            .with(c::Motion {
                velocity: Vector2::new(0.0, 0.0),
                acceleration: Vector2::new(0.0, 0.0),
            }).build();

        let image = world
            .assets
            .get::<_, resources::Image>(&warmy::FSKey::new(&tile_manager.image), ctx)
            .unwrap();
        let dispatcher = Self::register_systems();

        LevelScene {
            done,
            image,
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
        let pos = gameworld.specs_world.read_storage::<c::Position>();
        let render = gameworld.specs_world.read_storage::<c::Render>();

        for (p, r) in (&pos, &render).join() {
            draw_ex(
                ctx,
                &(self.image.borrow().0),
                DrawParam {
                    src: r.src,
                    // scale: Point2::new(4.0, 4.0),
                    dest: p.0,
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
