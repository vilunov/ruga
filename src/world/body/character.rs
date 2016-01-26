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
    gun: ModularGun,
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
            gun: ModularGun::new(),
        }
    }
}

const MODULAR_GUN_RANGE_UNIT: f64 = 10.;
const MODULAR_GUN_WIDTH_UNIT: f64 = 10.;
const MODULAR_GUN_DAMAGE_UNIT: f64 = 1.;
const MODULAR_GUN_DISTANCE_FACTOR: f64 = 1.;
const MODULAR_GUN_REALODING_FACTOR: f64 = 1.;
const MODULAR_GUN_MODULING_FACTOR: f64 = 1.;
const MODULAR_GUN_MAX_BULLET: u32 = 12;

#[derive(Clone)]
pub struct ModularGunSettings {
    pub nbr_of_cannon: u32,
    pub range: u32,
    pub width: u32,
    pub damage: u32,
}

impl ModularGunSettings {
    pub fn distance(&self, other: &ModularGunSettings) -> f64 {
        (((self.nbr_of_cannon - other.nbr_of_cannon) as f64).abs()
        + ((self.range - other.range) as f64).abs()
        + ((self.width - other.width) as f64).abs()
        + ((self.damage - other.damage) as f64).abs())
            * MODULAR_GUN_DISTANCE_FACTOR
    }
}

struct ModularGun {
    settings: ModularGunSettings,
    nbr_of_bullet: u32,
    reloading: f64,
    moduling: f64,
}

impl ModularGun {
    pub fn new() -> ModularGun {
        ModularGun {
            settings: ModularGunSettings {
                nbr_of_cannon: 0,
                range: 0,
                width: 0,
                damage: 0,
            },
            nbr_of_bullet: 0,
            reloading: 0.,
            moduling: 0.,
        }
    }

    pub fn settings(&self) -> ModularGunSettings {
        self.settings.clone()
    }

    pub fn set(&mut self, settings: &ModularGunSettings) {
        self.moduling = self.settings.distance(settings);
        self.settings = settings.clone();
    }

    pub fn update(&mut self, dt: f64) {
        if self.moduling > 0. {
            self.moduling -= dt * MODULAR_GUN_MODULING_FACTOR;
        } else if self.nbr_of_bullet != MODULAR_GUN_MAX_BULLET {
            self.reloading += dt * MODULAR_GUN_REALODING_FACTOR;

            while self.reloading > 1. {
                self.reloading -= 1.;
                self.nbr_of_bullet += 1;
            }

            if self.nbr_of_bullet >= MODULAR_GUN_MAX_BULLET {
                self.reloading = 0.;
                self.nbr_of_bullet = MODULAR_GUN_MAX_BULLET;
            }
        }
    }

    pub fn range(&self) -> f64 {
        (self.settings.range as f64) * MODULAR_GUN_RANGE_UNIT
    }

    pub fn width(&self) -> f64 {
        (self.settings.width as f64) * MODULAR_GUN_WIDTH_UNIT
    }

    pub fn damage(&self) -> f64 {
        (self.settings.damage as f64) * MODULAR_GUN_DAMAGE_UNIT
    }

    pub fn shoot(&mut self) {
    }

    pub fn ready(&self) -> bool {
        self.nbr_of_bullet > 0
    }
}

pub trait CharacterTrait {
    fn aim(&self) -> f64;
    fn set_aim(&self, a: f64);
    fn gun_shoot(&self);
    fn set_gun(&self,&ModularGunSettings);
    fn gun_settings(&self) -> ModularGunSettings;
    fn gun_ready(&self) -> bool;
}

impl CharacterTrait for RefCell<Character> {
    fn aim(&self) -> f64 {
        self.borrow().aim
    }

    fn set_aim(&self, a: f64) {
        self.borrow_mut().aim = a;
    }

    fn gun_shoot(&self) {
        self.borrow_mut().gun.shoot();
    }

    fn set_gun(&self,settings: &ModularGunSettings) {
        self.borrow_mut().gun.set(settings);
    }

    fn gun_settings(&self) -> ModularGunSettings {
        self.borrow().gun.settings()
    }

    fn gun_ready(&self) -> bool {
        self.borrow().gun.ready()
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
            collision_behavior() -> CollisionBehavior,
            render(viewport: &Viewport, camera: &Camera, gl: &mut GlGraphics) -> (),
            on_collision(other: &BodyTrait) -> (),
    }
    fn update(&self, dt: f64, batch: &Batch<Rc<BodyTrait>>) {
        let mut this = self.borrow_mut();
        this.body.update(dt,batch);
        this.gun.update(dt);
    }
}
