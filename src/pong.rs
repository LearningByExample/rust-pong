use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct PongGame {
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

const Z_FRONT: f32 = 1.0;
const Z_BACK: f32 = 0.0;

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;
const HALVE_WIDTH: f32 = ARENA_WIDTH * 0.5;
const HALVE_HEIGHT: f32 = ARENA_HEIGHT * 0.5;
const CAMERA_Z: f32 = Z_FRONT;

const PADDLE_WIDTH: f32 = 4.0;
const PADDLE_HEIGHT: f32 = 16.0;

const PADDLE_PIVOT_X: f32 = PADDLE_WIDTH * 0.5;
const PADDLE_LEFT_X: f32 = PADDLE_PIVOT_X;
const PADDLE_RIGHT_X: f32 = ARENA_WIDTH - PADDLE_PIVOT_X;
const PADDLE_INITIAL_Y: f32 = HALVE_HEIGHT;
const PADDLE_Z: f32 = Z_BACK;
const PADDLE_SPRITE_NUM: usize = 0;

const BALL_RADIUS: f32 = 2.0;
const BALL_VELOCITY_X: f32 = 75.0;
const BALL_VELOCITY_Y: f32 = 50.0;
const BALL_Z: f32 = Z_BACK;
const BALL_SPRITE_NUM: usize = 1;
const BALL_SPAWN_TIME: f32 = 2.0;

const GAME_SPRITE_SHEET_TEXTURE: &str = "texture/pong_spritesheet.png";
const GAME_SPRITE_SHEET_RON: &str = "texture/pong_spritesheet.ron";

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(HALVE_WIDTH, HALVE_HEIGHT, CAMERA_Z);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    left_transform.set_translation_xyz(PADDLE_LEFT_X, PADDLE_INITIAL_Y, PADDLE_Z);
    right_transform.set_translation_xyz(PADDLE_RIGHT_X, PADDLE_INITIAL_Y, PADDLE_Z);

    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: PADDLE_SPRITE_NUM,
    };

    // Create a left plank entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    // Create right plank entity.
    world
        .create_entity()
        .with(sprite_render)
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}

/// Initialises one ball in the middle-ish of the arena.
fn initialise_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(HALVE_WIDTH, HALVE_HEIGHT, BALL_Z);

    // Assign the sprite for the ball
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: BALL_SPRITE_NUM,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(local_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            GAME_SPRITE_SHEET_TEXTURE,
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the sprite sheet necessary to render the graphics.
    let sprite_sheet_handle = {
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            GAME_SPRITE_SHEET_RON, // Here we load the associated ron file
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        )
    };

    sprite_sheet_handle
}

impl PongGame {
    pub(crate) fn new() -> PongGame {
        PongGame {
            ball_spawn_timer: Some(BALL_SPAWN_TIME),
            sprite_sheet_handle: None,
        }
    }
}

impl SimpleState for PongGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        initialise_paddles(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(mut timer) = self.ball_spawn_timer.take() {
            // If the timer isn't expired yet, subtract the time that passed since the last update.
            {
                timer -= data.world.fetch::<Time>().delta_seconds();
            }
            if timer <= 0.0 {
                // When timer expire, spawn the ball
                // Since we are not put back the timer it will not enter on the if ... take()
                initialise_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
            } else {
                // If timer is not expired yet, put it back onto the state.
                self.ball_spawn_timer.replace(timer);
            }
        }
        Trans::None
    }
}
