use bevy::prelude::*;
use bevy_stutter::{RenderStutter, Stutter, StutterPlugin, UpdateStutter};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StutterPlugin)
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PreUpdate, detect_refresh_rate)
        .add_system(move_player_time_since_startup)
        .add_system(move_player_delta_time)
        .add_system(move_player_fixed_delta_time)
        .add_system(move_player_catch_up)
        .run();
}

const MOVE_SPEED: f32 = 0.5;

struct RefreshRate {
}

#[derive(Component)]
struct TimeSinceStartup;

fn move_player_time_since_startup(
    mut players: Query<&mut Transform, With<TimeSinceStartup>>,
    time: Res<Time>,
) {
    for mut tx in players.iter_mut() {
        tx.translation.x = f32::sin(time.time_since_startup().as_secs_f32() * MOVE_SPEED) * 500.0
    }
}

#[derive(Component)]
struct DeltaTime;

fn move_player_delta_time(
    mut players: Query<&mut Transform, With<DeltaTime>>,
    time: Res<Time>,
    mut acc: Local<f64>,
) {
    *acc += time.delta_seconds_f64();
    for mut tx in players.iter_mut() {
        tx.translation.x = f32::sin(*acc as f32 * MOVE_SPEED) * 500.0;
    }
}

#[derive(Component)]
struct FixedDeltaTime;

fn move_player_fixed_delta_time(
    mut players: Query<&mut Transform, With<FixedDeltaTime>>,
    mut acc: Local<f64>,
) {
    *acc += 1.0 / 60.0;
    for mut tx in players.iter_mut() {
        tx.translation.x = f32::sin(*acc as f32 * MOVE_SPEED) * 500.0;
    }
}

#[derive(Component)]
struct CatchUp;

fn move_player_catch_up(
    mut players: Query<&mut Transform, With<CatchUp>>,
    mut acc: Local<f64>,
    mut good_frames: Local<u32>,
    mut t: Local<f64>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds_f64();

    if (1.0 / 60.0 - dt).abs() < 0.001 {
        *good_frames += 1;
        if *good_frames >= 2 {
            if *good_frames == 2 {
                info!("locking frame rate again")
            }
            // healthy frame rate
            *acc = 0.0;
            *t += 1.0 / 60.0
        } else {
            // still recovering
            *acc += dt;
        }
    } else {
        *good_frames = 0;
        info!(
            "wacko frame rate {:.1}ms ({:.1}Hz)",
            dt * 1000.0,
            dt.recip()
        );
        *acc += dt;
    };

    while *acc > 1.0 / 60.0 {
        *t += 1.0 / 60.0;
        *acc -= 1.0 / 60.0;
    }

    for mut tx in players.iter_mut() {
        tx.translation.x = f32::sin(*t as f32 * MOVE_SPEED) * 500.0;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn().insert(RenderStutter(Stutter {
        probability: 0.00004,
        millis: 16,
    }));

    commands.spawn().insert(UpdateStutter(Stutter {
        probability: 0.04,
        millis: 16,
    }));

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            ..default()
        })
        .insert(TimeSinceStartup);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::Y * 100.0),
            ..default()
        })
        .insert(DeltaTime);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::Y * -100.0),
            ..default()
        })
        .insert(FixedDeltaTime);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::Y * -200.0),
            ..default()
        })
        .insert(CatchUp);
}
