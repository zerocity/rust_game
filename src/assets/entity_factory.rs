use assets::tilemap::TilemapManager;
use components as c;
use ggez::graphics::{Point2, Vector2};
use specs::Builder;
use world::World;

pub fn create_map(tile_manager: &TilemapManager, world: &mut World) {
    for tile in tile_manager.get_grid().iter() {
        if let Some(id) = tile.sprite_id {
            // ignore tiles with 0 --> no tile
            if id > 0 {
                if let Some(id) = &tile.sprite_id {
                    let image = tile_manager.get_image_by_id(id);
                    if let Some(image) = image {
                        let e = world
                            .specs_world
                            .create_entity()
                            .with(c::Render {
                                src: tile.src,
                                image,
                            }).with(c::Position(tile.dest));

                        e.build();
                    }
                }
            }
        }
    }
}

pub fn create_player(tile_manager: &TilemapManager, world: &mut World) {
    let player = tile_manager.get_sprite_by_id(&973).unwrap().to_owned();
    let p_images = tile_manager.get_image_by_id(&973);

    if let Some(p_images) = p_images {
        world
            .specs_world
            .create_entity()
            .with(c::Render {
                src: player.src,
                image: p_images,
            }).with(c::Position(Point2::new(100.0, 100.0)))
            .with(c::Controllable)
            .with(c::Motion {
                velocity: Vector2::new(0.0, 0.0),
                acceleration: Vector2::new(0.0, 0.0),
            }).build();
    }
}
