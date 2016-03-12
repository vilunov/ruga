use frame_manager::{FrameManager, color};
use world::body::{Body, CollisionBehavior, PhysicType};
use world::{Entity, EntityCell};
use std::f64;
use std::cell::{RefCell, Ref, RefMut};

pub struct Wall {
    body: Body,
}

impl Wall {
    pub fn new(x: i32, y: i32, unit: f64) -> Wall {
        Wall {
            body : Body {
                id: 0,
                x: (x as f64 + 0.5)*unit,
                y: (y as f64 + 0.5)*unit,
                life: f64::MAX,
                width: unit,
                height: unit,
                weight: f64::MAX,
                velocity: 0.,
                items: Vec::new(),
                angle: 0.,
                mask: !0,
                group: super::group::WALL,
                collision_behavior: CollisionBehavior::Stop,
                physic_type: PhysicType::Dynamic,
            }
        }
    }
}

impl EntityCell for RefCell<Wall> {
    fn borrow(&self) -> Ref<Entity> {
        (self as &RefCell<Entity>).borrow()
    }
    fn borrow_mut(&self) -> RefMut<Entity> {
        (self as &RefCell<Entity>).borrow_mut()
    }
}

impl Entity for Wall {
    fn body(&self) -> &Body {
        &self.body
    }
    fn mut_body(&mut self) -> &mut Body {
        &mut self.body
    }
    fn render(&self, frame_manager: &mut FrameManager) {
        self.body.render(color::RED, frame_manager);
    }
}

