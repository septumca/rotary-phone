use std::f32::consts::FRAC_PI_2;

use bevy::{prelude::*, window::WindowResolution, math::{vec2}};

#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_rapier2d::{
    prelude::{
        RapierPhysicsPlugin,
        NoUserData,
        Collider,
        RigidBody,
        RapierConfiguration,
        KinematicCharacterController,
        LockedAxes,
        ActiveEvents,
        ActiveCollisionTypes,
    },
    render::RapierDebugRenderPlugin
};
use components::{Wall, Character, PlayerControlled, Controls, EquippedSkill};
use plugins::{movement::MovementPlugin, timers::TimersPlugin, collision::CollisionPlugin, player_input::PlayerInputPlugin, events::EventsPlugin, skills::SkillsPlugin};

pub mod components;
pub mod plugins;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const SPRITE_SIZE: f32 = 16.0;
const SCALE_FACTOR: f32 = 3.0;
const SPRITE_DRAW_SIZE: f32 = SPRITE_SIZE * SCALE_FACTOR;
const CHARACTER_Z_INDEX: f32 = 1.0;
const ATTACK_Z_INDEX: f32 = 1.5;
const FIREBALL_SPEED: f32 = 800.0;
const SLASH_SPEED: f32 = FRAC_PI_2;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
}

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
        .add_state::<GameState>()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugin(EventsPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(PlayerInputPlugin)
        .add_plugin(TimersPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(SkillsPlugin)
        .add_startup_system(setup)
        .add_system(setup_world.in_schedule(OnEnter(GameState::Playing)))
        ;

    #[cfg(debug_assertions)]
    {
        app.add_plugin(WorldInspectorPlugin::new());
        app.add_plugin(RapierDebugRenderPlugin::default());
    }

    app.run();
}


#[derive(Resource)]
pub struct GameResources {
    image_handle: Handle<Image>,
    font_handle: Handle<Font>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let image_handle = asset_server.load("sprites.png");
    let font_handle = asset_server.load("QuinqueFive.ttf");

    commands.insert_resource(GameResources {
        image_handle,
        font_handle,
    });
    commands.spawn(Camera2dBundle::default());
}


fn setup_world(
    mut commands: Commands,
    game_resources: Res<GameResources>,
) {
    commands.spawn((
            RigidBody::Fixed,
            Collider::cuboid(SPRITE_DRAW_SIZE / 2.0, SPRITE_DRAW_SIZE / 2.0),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                    rect: Some(Rect::new(6.0 * SPRITE_SIZE, 0., 7.0 * SPRITE_SIZE, SPRITE_SIZE)),
                    ..default()
                },
                texture: game_resources.image_handle.clone(),
                transform: Transform::from_xyz(0., -100.0, CHARACTER_Z_INDEX),
                ..default()
            },
            ActiveCollisionTypes::all(),
            Wall,
        ));

    commands.spawn((
            Character {
                attack_cd: Timer::from_seconds(0.2, TimerMode::Once),
            },
            PlayerControlled,
            Controls::Idle,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(vec2(SPRITE_DRAW_SIZE, SPRITE_DRAW_SIZE)),
                    rect: Some(Rect::new(0.0 * SPRITE_SIZE, 0., 1.0 * SPRITE_SIZE, SPRITE_SIZE)),
                    ..default()
                },
                texture: game_resources.image_handle.clone(),
                transform: Transform::from_xyz(0., 0., CHARACTER_Z_INDEX),
                ..default()
            },
            EquippedSkill::Fire,
            RigidBody::KinematicPositionBased,
            Collider::cuboid(SPRITE_DRAW_SIZE / 2.0, SPRITE_DRAW_SIZE / 2.0),
            KinematicCharacterController::default(),
            LockedAxes::ROTATION_LOCKED,
            ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
            ActiveEvents::COLLISION_EVENTS
        ));
}
