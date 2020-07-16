use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{palette::Srgba, resources::Tint},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use crate::audio::initialise_audio;

pub struct PongGame {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

const Z_FRONT: f32 = 1.0;
const Z_BACK: f32 = 0.0;

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;
pub const HALVE_WIDTH: f32 = ARENA_WIDTH * 0.5;
pub const HALVE_HEIGHT: f32 = ARENA_HEIGHT * 0.5;
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
const BALL_VELOCITY_X: f32 = 30.0;
const BALL_VELOCITY_Y: f32 = 15.0;
const BALL_ACCELERATION: f32 = 0.2;
const MAX_BALL_VELOCITY_X: f32 = BALL_VELOCITY_X * 2.0;
const MAX_BALL_VELOCITY_Y: f32 = BALL_VELOCITY_Y * 2.0;
pub const BALL_Z: f32 = Z_BACK;
const BALL_SPRITE_NUM: usize = 1;

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

#[derive(PartialEq, Eq)]
pub enum BallState {
    Waiting,
    Moving,
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub acceleration: f32,
    pub radius: f32,
    pub state: BallState,
    pub waiting_time: f32,
}

impl Ball {
    pub fn wait(&mut self) {
        self.state = BallState::Waiting;
        self.waiting_time = 2.0;
        self.velocity = [BALL_VELOCITY_X, BALL_VELOCITY_Y]
    }
    pub fn accelerate(&mut self) {
        let velocity_x =
            (self.velocity[0] + (self.velocity[0] * self.acceleration)).min(MAX_BALL_VELOCITY_X);
        let velocity_y =
            (self.velocity[1] + (self.velocity[1] * self.acceleration)).min(MAX_BALL_VELOCITY_Y);
        self.velocity = [velocity_x, velocity_y]
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

/// ScoreBoard contains the actual score data
#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

/// ScoreText contains the ui text components that display the score
pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
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

    // White shows the sprite as normal.
    // You can change the color at any point to modify the sprite's tint.
    let tint = Tint(Srgba::new(1.0, 1.0, 1.0, 1.0));
    let mut cycling = CyclingColor::new(Srgba::new(1.0, 0.0, 0.0, 1.0), 0.5);
    cycling.start();

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
            acceleration: BALL_ACCELERATION,
            state: BallState::Waiting,
            waiting_time: 2.0,
        })
        .with(local_transform)
        .with(tint)
        .with(cycling)
        .build();
}

/// Initialises a ui scoreboard
fn initialise_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -50.,
        -50.,
        1.,
        200.,
        50.,
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        50.,
        -50.,
        1.,
        200.,
        50.,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(font, "0".to_string(), [1., 1., 1., 1.], 50.))
        .build();

    world.insert(ScoreText { p1_score, p2_score });
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

#[derive(PartialEq, Eq)]
pub enum CyclingState {
    Stopped,
    Cycling,
}

pub struct CyclingColor {
    pub state: CyclingState,
    pub cycle_time: f32,
    pub color: Srgba,
    pub from: Srgba,
    pub to: Srgba,
    pub current_cycle: f32,
}

impl Component for CyclingColor {
    type Storage = DenseVecStorage<Self>;
}

impl CyclingColor {
    fn new(to: Srgba, cycle_time: f32) -> CyclingColor {
        CyclingColor {
            state: CyclingState::Stopped,
            color: to,
            from: Srgba::new(1.0, 1.0, 1.0, 1.0),
            to,
            cycle_time: cycle_time / 2.0,
            current_cycle: 0.0,
        }
    }

    pub fn stop(&mut self) {
        self.state = CyclingState::Stopped;
    }

    pub fn start(&mut self) {
        self.state = CyclingState::Cycling;
        self.from = Srgba::new(1.0, 1.0, 1.0, 1.0);
        self.to = self.color;
        self.current_cycle = 0.0;
    }
}

impl PongGame {
    pub(crate) fn new() -> PongGame {
        PongGame {
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
        initialise_ball(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_camera(world);
        initialise_scoreboard(world);
        initialise_audio(world);
    }
}
