// Sprites made by Penzilla ("https://penzilla.itch.io/hooded-protagonist")
// Powered by Bevy Game Engine

use bevy::prelude::*;

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 800.0;

pub const PLAYER_MOVE_SPEED: f32 = 100.;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct SpriteAnimation {
    len: usize,
    frame_time: f32,
}

#[derive(Component)]
struct FrameTime(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Jump and Run".into(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_player)
        .add_systems(FixedUpdate, animate_sprites)
        .add_systems(FixedUpdate, player_movement)
        .run();
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_player(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/AnimationSheet_Character.png"),
        Vec2::splat(32.),
        8, 9, None, None,
    );

    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas.add(atlas),
        sprite: TextureAtlasSprite { index: 0, ..Default::default() },
        ..Default::default()
    }, Player,
                    SpriteAnimation {
                        len: 2,
                        frame_time: 1. / 2.6,
                    },
                    FrameTime(0.0)
    ));
}

fn animate_sprites(
    mut query: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in query.iter_mut() {
        frame_time.0 += time.delta_seconds();
        if frame_time.0 > animation.frame_time {
            let frames = (frame_time.0 / animation.frame_time) as usize;
            sprite.index += frames;
            if sprite.index >= animation.len { sprite.index %= animation.len; }
            frame_time.0 -= animation.frame_time * frames as f32;
        }
    }
}

fn player_movement(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    let mut player = player.single_mut();
    if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        player.translation.x -= PLAYER_MOVE_SPEED * time.delta_seconds();
    } else if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        player.translation.x += PLAYER_MOVE_SPEED * time.delta_seconds();
    }
}

fn change_player_animation(
    mut player: Query<(&mut Handle<TextureAtlas>, &mut SpriteAnimation, &mut TextureAtlasSprite), With<Player>>,
    input: Res<Input<KeyCode>>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let (mut atlas, mut animation, mut sprite) = player.single_mut();


    // on key press change sprite animation
    if input.any_just_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        // Walk Animation
    }

    if input.any_just_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right])
        && !input.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        // Idle Animation
    }
}