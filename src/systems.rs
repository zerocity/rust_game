//! specs systems.
use specs::{self, Join};

use components::*;

pub struct MovementSystem;

impl<'a> specs::System<'a> for MovementSystem {
    type SystemData = (
        // specs::Fetch<'a, Keyboard>,
        specs::WriteStorage<'a, Position>,
        specs::ReadStorage<'a, Motion>,
        specs::ReadStorage<'a, Render>,
    );

    fn run(&mut self, (mut pos, motion, render): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        for (pos, motion, _) in (&mut pos, &motion, &render).join() {
            pos.0 += motion.velocity;
        }
    }
}
