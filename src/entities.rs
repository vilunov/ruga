use components::*;
use specs;
use config;

pub fn add_character(world: &specs::World, pos: [f32;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new(pos))
        .with::<PhysicDynamic>(PhysicDynamic)
        .with::<PhysicType>(PhysicType::new_movable(
                config.entity.char_group,
                config.entity.char_mask,
                Shape::Circle(config.entity.char_radius),
                CollisionBehavior::Persist,
                config.entity.char_velocity,
                config.entity.char_time,
                config.entity.char_weight))
        .with::<PhysicForce>(PhysicForce::new())
        .with::<Color>(Color::from_str(&*config.entity.char_color))
        .with::<PlayerControl>(PlayerControl)
        .build();
}

pub fn add_wall(world: &specs::World, pos: [isize;2]) {
    world.create_now()
        .with::<PhysicState>(PhysicState::new_aligned(pos))
        .with::<PhysicStatic>(PhysicStatic)
        .with::<PhysicType>(PhysicType::new_static(
                config.entity.wall_group,
                config.entity.wall_mask,
                Shape::Square(config.entity.wall_radius)))
        .with::<Color>(Color::from_str(&*config.entity.wall_color))
        .build();
}

