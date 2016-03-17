//! https://en.wikipedia.org/wiki/Maze_generation_algorithm#Randomized_Kruskal.27s_algorithm

extern crate rand;

// use util::direction::Direction;
use self::rand::distributions::{IndependentSample, Range};
use world::{World, EntityCell};
use world::body::Item;
use entities::*;
use std::cell::RefCell;
use std::rc::Rc;

const AVERAGE_MOVING_WALL_PER_UNIT: f64 = 0.1;
const WEAPON_COEF: f64 = 0.1;
const SPIDER_COEF: f64 = 0.1;
const BOID_COEF: f64 = 0.1;
const SPAWN_DISTANCE: i32 = 3;

#[derive(Debug)]
enum WallPos {
    Vertical(usize,usize),
    Horizontal(usize,usize),
}

fn generate_partial_reverse_randomized_kruskal(width: usize, height: usize, percent: f64) -> Vec<bool> {
    assert_eq!(width.wrapping_rem(2), 1);
    assert_eq!(height.wrapping_rem(2), 1);

    let index = |x: usize, y: usize| y*width+x;

    let mut grid = Vec::with_capacity(width*height);
    for i in 0..width*height {
        grid.push((false,i));
    }

    for i in 0..width {
        grid[i] = (true, i);
        let j = height*(width-1)+i;
        grid[j] = (true, j);
    }

    for i in 0..height {
        grid[i*width] = (true, i*width);
        let j = (i+1)*width - 1;
        grid[j] = (true,j);
    }

    let horizontal_wall = (width-5)/2 * (height-3)/2;
    let vertical_wall = (width-3)/2 * (height-5)/2;
    let horizontal_wall_width = (width-5)/2;
    let vertical_wall_width = (width-3)/2;

    let mut walls = Vec::with_capacity(horizontal_wall+vertical_wall);
    for i in 0..vertical_wall {
        walls.push(WallPos::Vertical(i.wrapping_rem(vertical_wall_width)*2+2, (i/vertical_wall_width)*2+3));
    }
    for i in 0..horizontal_wall {
        walls.push(WallPos::Horizontal(i.wrapping_rem(horizontal_wall_width)*2+3, (i/horizontal_wall_width)*2+2));
    }

    let mut rng = rand::thread_rng();

    let stop = ((walls.len() as f64)*(1.-percent/100.)) as usize;

    while walls.len() > stop {
        let i = Range::new(0,walls.len()).ind_sample(&mut rng);
        assert!(i<walls.len());
        let (c1,c2,c3) = match walls.swap_remove(i) {
            WallPos::Vertical(x,y) => {
                (index(x,y-1), index(x,y), index(x,y+1))
            },
            WallPos::Horizontal(x,y) => {
                (index(x-1,y), index(x,y), index(x+1,y))
            },
        };

        let ((_,s1),(_,s2),(_,s3)) = (grid[c1],grid[c2],grid[c3]);

        if s1 != s3 {
            grid[c1] = (true,s1);
            grid[c2] = (true,s2);
            grid[c3] = (true,s3);
            for &mut(_,ref mut s) in &mut grid {
                if *s == s2 || *s == s3 {
                    *s = s1;
                }
            }
        }
    }

    grid.iter().map(|&(b,_)| b).collect::<Vec<bool>>()
}

pub fn generate() -> (World,Rc<RefCell<Character>>) {
    let width = 17;
    let height = 17;
    let unit = 16.;
    let percent = 30.;

    let maze = generate_partial_reverse_randomized_kruskal(width,height,percent);

    let mut world = World::new(unit);

    let mut rng = rand::thread_rng();
    let zero_un_range = Range::new(0.,1.);

    for i in 0..maze.len() {
        let x = (i.wrapping_rem(width)) as i32;
        let y = (i/width) as i32;

        if maze[i] {
            world.insert(&(Rc::new(RefCell::new(Wall::new(x,y,unit))) as Rc<EntityCell>));
        } else {
            if zero_un_range.ind_sample(&mut rng) < AVERAGE_MOVING_WALL_PER_UNIT {
                world.insert(&(Rc::new(RefCell::new(BurningWall::new(x,y,unit))) as Rc<EntityCell>));
            }
        }
    }

    world.insert(&(Rc::new(RefCell::new(Spider::new(unit*1.5,unit*1.5,0.))) as Rc<EntityCell>));

    let character = Rc::new(RefCell::new(Character::new(unit*1.5,unit*1.5,0.)));
    world.insert(&(Rc::new(RefCell::new(Armory::new(unit*1.5,unit*1.5,Item::Rifle(10)))) as Rc<EntityCell>));
    world.insert(&(Rc::new(RefCell::new(Armory::new(unit*2.5,unit*1.5,Item::Shotgun(10)))) as Rc<EntityCell>));
    world.insert(&(Rc::new(RefCell::new(Armory::new(unit*3.5,unit*1.5,Item::Sniper(10)))) as Rc<EntityCell>));
    world.insert(&(character.clone() as Rc<EntityCell>));

    (world,character)
}

pub fn update(character: &EntityCell, world: &mut World) {
    let mut rng = rand::thread_rng();
    let zero_un_range = Range::new(0.,1.);
    let char_x = character.borrow().body().x;
    let char_y = character.borrow().body().y;

    if zero_un_range.ind_sample(&mut rng) < AVERAGE_MOVING_WALL_PER_UNIT {
        if let Some((x,y)) = random_position(char_x,char_y,world) {
            // TODO
            // world.insert(&(Rc::new(RefCell::new(BurningWall::new(x,y,unit))) as Rc<EntityCell>));
        }
    }
}

fn random_position(x: f64, y: f64, world: &World) -> Option<(f64,f64)> {
    //TODO
    None
}
