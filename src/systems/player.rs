use components::*;
use ggez::event::Keycode;
use ggez::graphics::Vector2;
use keyboard;
use specs::{self, HashMapStorage, Join, Read};

pub struct PlayerMovementSystem;

impl<'a> specs::System<'a> for PlayerMovementSystem {
    type SystemData = (
        specs::WriteStorage<'a, Position>,
        specs::ReadStorage<'a, Motion>,
        specs::ReadStorage<'a, Render>,
        Read<'a, keyboard::Keyboard>,
    );

    fn run(&mut self, (mut pos, motion, render, kbd): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.

        let mut pv = Vector2::new(0.0, 0.0);
        let mut speed = 1.;

        if kbd.is_pressed(Keycode::LShift) {
            speed = 2.;
        }

        if kbd.is_pressed(Keycode::D) {
            pv.x = 1. * speed;
        }
        if kbd.is_pressed(Keycode::A) {
            pv.x = -1. * speed;
        }
        if kbd.is_pressed(Keycode::W) {
            pv.y = -1. * speed;
        }

        if kbd.is_pressed(Keycode::S) {
            pv.y = 1. * speed;
        }

        for (pos, motion, _) in (&mut pos, &motion, &render).join() {
            pos.0 += motion.velocity + pv;
        }
    }
}
