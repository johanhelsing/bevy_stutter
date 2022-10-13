use bevy::prelude::*;
use bevy_stutter::{RenderStutter, Stutter, StutterPlugin, UpdateStutter};

fn main() {
    App::new()
        // .insert_resource(WindowDescriptor {
        //     mode: bevy::window::WindowMode::Fullscreen,
        //     ..default()
        // })
        .add_plugins(DefaultPlugins)
        .add_plugin(StutterPlugin)
        .add_startup_system(setup)
        .add_system(move_player_time_since_startup)
        .add_system(move_player_delta_time)
        .add_system(move_player_fixed_delta_time)
        .add_system(move_player_catch_up)
        .add_system(move_player_catch_up_simple)
        .add_system(move_player_golden)
        .run();
}

const MOVE_SPEED: f32 = 0.5;
const TARGET_REFRESH_RATE: f64 = 60.0;
const TARGET_REFRESH_INTERVAL: f64 = 1.0 / TARGET_REFRESH_RATE;

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
    *acc += TARGET_REFRESH_INTERVAL;
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

    if (TARGET_REFRESH_INTERVAL - dt).abs() < 0.001 {
        *good_frames += 1;
        if *good_frames >= 2 {
            if *good_frames == 2 {
                info!("locking frame rate again")
            }
            // healthy frame rate
            *acc = 0.0;
            *t += TARGET_REFRESH_INTERVAL
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

    while *acc > TARGET_REFRESH_INTERVAL {
        *t += TARGET_REFRESH_INTERVAL;
        *acc -= TARGET_REFRESH_INTERVAL;
    }

    for mut tx in players.iter_mut() {
        tx.translation.x = f32::sin(*t as f32 * MOVE_SPEED) * 500.0;
    }
}

#[derive(Component)]
struct CatchUpSimple;

fn move_player_catch_up_simple(
    mut players: Query<&mut Transform, With<CatchUpSimple>>,
    mut t: Local<f64>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds_f64();

    if dt > TARGET_REFRESH_INTERVAL * 1.3 {
        info!("dropped frame, catching up {:.1}", dt * 1000.0);
        *t += TARGET_REFRESH_INTERVAL;
    }

    *t += TARGET_REFRESH_INTERVAL;

    for mut tx in players.iter_mut() {
        tx.translation.x = f32::sin(*t as f32 * MOVE_SPEED) * 500.0;
    }
}

#[derive(Component)]
struct Golden4AndASixth;

fn move_player_golden(
    mut players: Query<&mut Transform, With<Golden4AndASixth>>,
    mut t: Local<f64>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds_f64();

    let delta_updates = dt * 240.0;

    info!("delta_updates {delta_updates}");

    let delta_updates = delta_updates.round() as i32;

    // don't run slower than 30 Hz or faster than 240 Hz
    let delta_updates = delta_updates.clamp(1, 8);

    *t += (delta_updates as f64) / 240.0;

    for mut tx in players.iter_mut() {
        tx.translation.x = f32::sin(*t as f32 * MOVE_SPEED) * 500.0;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn().insert(RenderStutter(Stutter {
        probability: 0.00004,
        millis: 32,
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
            transform: Transform::from_translation(Vec3::Y * 200.0),
            ..default()
        })
        .insert(Golden4AndASixth);

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

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::Y * -300.0),
            ..default()
        })
        .insert(CatchUpSimple);
}
