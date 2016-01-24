use viewport::Viewport;
use opengl_graphics::GlGraphics;
use world::{ 
    Camera, 
};

use super::{ 
    Body, 
    BodyTrait, 
    BodyType,
    CollisionBehavior,
};
use world::batch::Batch;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Character {
    body: Body,
    aim: f64,
}

pub const WIDTH: f64 = 10.;
pub const HEIGHT: f64 = 10.;
pub const WEIGHT: f64 = 1.;
pub const MASK: u32 = !0;
pub const GROUP: u32 = 2;


impl Character {
    pub fn new(id: usize, x: f64, y: f64, angle: f64) -> Character {
        Character {
            body: Body {
                id: id,
                x: x,
                y: y,
                width2: WIDTH/2.,
                height2: HEIGHT/2.,
                weight: WEIGHT,
                velocity: 0.,
                angle: angle,
                mask: MASK,
                group: GROUP,
                collision_behavior: CollisionBehavior::Persist,
                body_type: BodyType::Character,
            },
            aim: angle,
        }
    }
}

pub trait CharacterTrait {
    fn aim(&self) -> f64;
    fn set_aim(&self, a: f64);
    fn shoot(&self);
}

impl CharacterTrait for RefCell<Character> {
    fn aim(&self) -> f64 {
        self.borrow().aim
    }

    fn set_aim(&self, a: f64) {
        self.borrow_mut().aim = a;
    }

    fn shoot(&self) {
    }
}

impl BodyTrait for RefCell<Character> {
    delegate!{
        body:
            id() -> usize,
            body_type() -> BodyType,
            damage(d: f64) -> (),
            width2() -> f64,
            height2() -> f64,
            x() -> f64,
            mut set_x(x: f64) -> (),
            y() -> f64,
            mut set_y(y: f64) -> (),
            weight() -> f64,
            velocity() -> f64,
            mut set_velocity(v: f64) -> (),
            angle() -> f64,
            mut set_angle(a: f64) -> (),
            mask() -> u32,
            group() -> u32,
            mut update(dt: f64, batch: &Batch<Rc<BodyTrait>>) -> (),
            collision_behavior() -> CollisionBehavior,
            render(viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) -> (),
            on_collision(other: &BodyTrait) -> (),
    }
}
