use configuration::{ self, BitflagU32, VecStringPath };
use toml;
use std;

use levels as levelss;
use graphics::{ Color, Layer };

pub type VecVecStringPath = Vec<VecStringPath>;
pub type Dimension = [u32;2];
pub type Array4F32 = [f32;4];
pub type Array4F64 = [f64;4];
pub type VecF32 = Vec<f32>;
pub type VecU8 = Vec<u8>;
pub type Dungeons = Vec<levelss::Dungeon>;
pub type Array3U8 = [u8;3];

fn config_constraint(conf: &Config) -> Result<(),String> {
    if conf.keys.up.len() == 0
       || conf.keys.down.len() == 0
       || conf.keys.left.len() == 0
       || conf.keys.right.len() == 0 {
           return Err("ERROR: configuration file invalid: keys mustn't be empty".into());
    }

    // assert persistent snd and static snd doesn't overlap
    if conf.entities.monster_persistent_snd == conf.entities.laser_persistent_snd {
        return Err("ERROR: configuration file invalid: monster_persistent_snd and laser_persistent_snd must be different".into());
    }

    Ok(())
}

configure!(
    file = "config.toml";
    debug_file = "config.toml";
    save_file = "save.toml";

    constraint = config_constraint;

    general: {
        number_of_thread: t usize,
        persistent_snd_cooldown: t usize,
        difficulty: t f32 save difficulty,
    },
    keys: {
        up: t VecU8,
        down: t VecU8,
        left: t VecU8,
        right: t VecU8,
        escape: t VecU8,
    },
    effect: {
        color: t Color,
        angles: t VecF32,
        persistance: t f32,
        thickness: t f32,
        inner_length: t f32,
        length: t f32,
    },
    physic: {
        rate: t f32,
        unit: t f32,
    },
    touch: {
        joystick_rec: t Array4F64,
        joystick_radius: t f64,
        escape_rec: t Array4F64,
    },
    joystick: {
        time_to_repeat: t f32,
        time_to_start_repeating: t f32,
        press_epsilon: t f32,
        release_epsilon: t f32,
    },
    menu:{
        entry_color: t Color,
        cursor_color: t Color,
        background_color: t Color,
        clic_snd: t usize,

        background_width: t f32,
        background_height: t f32,
    },
    entities: {
        text_color: t Color,

        ball_group: t BitflagU32,
        ball_mask: t BitflagU32,
        ball_killer_mask: t BitflagU32,
        ball_kill_snd: t usize,
        ball_die_snd: t usize,
        ball_radius: t f32,
        ball_velocity: t f32,
        ball_time: t f32,
        ball_weight: t f32,
        ball_color: t Color,
        ball_layer: t Layer,
        ball_vel_snd_coef: t f32,
        ball_vel_snd: t usize,

        laser_group: t BitflagU32,
        laser_mask: t BitflagU32,
        laser_killer_mask: t BitflagU32,
        laser_kill_snd: t usize,
        laser_radius: t f32,
        laser_color: t Color,
        laser_layer: t Layer,
        laser_persistent_snd: t usize,

        column_group: t BitflagU32,
        column_mask: t BitflagU32,
        column_radius: t f32,
        column_color: t Color,
        column_layer: t Layer,
        column_cooldown: t f32,
        column_spawn_snd: t usize,

        char_group: t BitflagU32,
        char_mask: t BitflagU32,
        char_radius: t f32,
        char_velocity: t f32,
        char_time: t f32,
        char_weight: t f32,
        char_color: t Color,
        char_layer: t Layer,
        char_die_snd: t usize,
        char_restart: t f32,

        wall_group: t BitflagU32,
        wall_mask: t BitflagU32,
        wall_radius: t f32,
        wall_color: t Color,
        wall_layer: t Layer,

        monster_vision_mask: t BitflagU32,
        monster_killer_mask: t BitflagU32,
        monster_kill_snd: t usize,
        monster_die_snd: t usize,
        monster_group: t BitflagU32,
        monster_mask: t BitflagU32,
        monster_vision_time: t f32,
        monster_radius: t f32,
        monster_velocity: t f32,
        monster_time: t f32,
        monster_weight: t f32,
        monster_color: t Color,
        monster_layer: t Layer,
        monster_persistent_snd: t usize,

        portal_end_color: t Color,
        portal_end_layer: t Layer,
        portal_start_color: t Color,
        portal_start_layer: t Layer,
        portal_snd: t usize,
    },
    levels: {
        hall_length: t usize,
        corridor_length: t usize,
        dir: t VecStringPath,
        entry_music: t VecStringPath,
        check_level: e String [always,debug,never],

        empty_col: t Array3U8,
        char_col: t Array3U8,
        portal_col: t Array3U8,
        laser_col: t Array3U8,
        monster_col: t Array3U8,
        column_col: t Array3U8,
        wall_col: t Array3U8,
    },
    audio: {
        effect_dir: t VecStringPath,
        music_dir: t VecStringPath,
        global_volume: t f32 save global_volume,
        music_volume: t f32 save music_volume,
        effect_volume: t f32 save effect_volume,
        distance_model: e String [linear,pow2],
        distance_model_min: t f32,
        distance_model_max: t f32,
        short_effects: t VecVecStringPath,
        persistent_effects: t VecVecStringPath,
        transition_type: e String [instant,smooth,overlap],
        transition_time: t u64,
    },
    window: {
        dimension: t Dimension,
        vsync: t bool,
        multisampling: t u16,
        fullscreen: t bool,
        fullscreen_on_primary_monitor: t bool,
        fullscreen_monitor: t usize,
    },
    graphics: {
        base03: t Array4F32,
        base02: t Array4F32,
        base01: t Array4F32,
        base00: t Array4F32,
        base0: t Array4F32,
        base1: t Array4F32,
        base2: t Array4F32,
        base3: t Array4F32,
        yellow: t Array4F32,
        orange: t Array4F32,
        red: t Array4F32,
        magenta: t Array4F32,
        violet: t Array4F32,
        blue: t Array4F32,
        cyan: t Array4F32,
        green: t Array4F32,
        mode: e String [light,dark] save mode,
        luminosity: t f32 save luminosity,
        circle_precision: t usize,
        font_file: t VecStringPath,
        billboard_font_scale: t f32,
        font_scale: t f32,
    },
    text: {
        top: t i32,
        bottom: t i32,
        right: t i32,
        left: t i32,
    },
    camera: {
        zoom: t f32,
    },
    event_loop: {
        ups: t u64,
        max_fps: t u64,
    },
);

