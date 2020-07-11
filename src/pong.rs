use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct PongGame;

const IDENTITY: f32 = 1.0;
const Z_FRONT: f32 = 0.0;

const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;
const HALVE_WIDTH: f32 = ARENA_WIDTH * 0.5;
const HALVE_HEIGHT: f32 = ARENA_HEIGHT * 0.5;

const PADDLE_WIDTH: f32 = 4.0;
const PADDLE_HEIGHT: f32 = 16.0;

const PADDLE_PIVOT_X: f32 = PADDLE_WIDTH * 0.5;
const PADDLE_LEFT_X: f32 = PADDLE_PIVOT_X;
const PADDLE_RIGHT_X: f32 = ARENA_WIDTH - PADDLE_PIVOT_X;
const PADDLE_INITIAL_Y: f32 = HALVE_HEIGHT;
const PADDLE_Z: f32 = Z_FRONT;
const PADDLE_SPRITE_NUM: usize = 0;

const GAME_SPRITE_SHEET_TEXTURE: &str = "texture/pong_spritesheet.png";
const GAME_SPRITE_SHEET_RON: &str = "texture/pong_spritesheet.ron";

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(HALVE_WIDTH, HALVE_HEIGHT, IDENTITY);

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

impl SimpleState for PongGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        let sprite_sheet_handle = load_sprite_sheet(world);

        initialise_paddles(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}
