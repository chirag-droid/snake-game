use std::f32::consts::PI;

use bevy::prelude::*;

use crate::GameState;
use crate::loading::TextureAssets;

// Sprites
const TILE_SIZE: Vec2 = Vec2::new(16., 16.);
const SPRITE_SHEET_COLUMNS: usize = 4;
const SPRITE_SHEET_ROWS: usize = 4;

pub struct SnakePlugin;

#[derive(Clone)]
pub enum Direction {
    Up, Down,
    Left, Right
}

#[derive(Component)]
pub struct SnakeHead {
    direction: Direction
}

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_snake);
        app.add_systems(Update, (
            move_snake,
            change_snake_direction
        ).run_if(in_state(GameState::Playing)));
    }
}

fn spawn_snake(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let texture = textures.sprite.clone();
    let layout = TextureAtlasLayout::from_grid(TILE_SIZE, SPRITE_SHEET_COLUMNS, SPRITE_SHEET_ROWS, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands
        .spawn(SpriteSheetBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(32f32, 32f32)),
                ..default()
            },
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0
            },
            ..default()
        })
        .insert(SnakeHead {
            direction: Direction::Up
        });
}

pub fn move_snake(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &SnakeHead), With<SnakeHead>>
) {
    let delta = 75. * time.delta_seconds();

    for (mut transform, snake) in query.iter_mut() {
        match snake.direction {
            Direction::Up => transform.translation.y += delta,
            Direction::Down => transform.translation.y -= delta,
            Direction::Left => transform.translation.x -= delta,
            Direction::Right => transform.translation.x += delta,
        }
    }
}

pub fn change_snake_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut SnakeHead), With<SnakeHead>>
) {
    for (mut transform, mut snake) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            snake.direction = Direction::Left;
            transform.rotation = Quat::from_rotation_z(PI/2.);
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            snake.direction = Direction::Right;
            transform.rotation = Quat::from_rotation_z(-PI/2.);
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            snake.direction = Direction::Down;
            transform.rotation = Quat::from_rotation_z(PI);
        }
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            snake.direction = Direction::Up;
            transform.rotation = Quat::from_rotation_z(0.);
        }
    }
}

