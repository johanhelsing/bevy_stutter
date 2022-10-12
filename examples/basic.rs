use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player_time_since_startup)
        .run();
}

fn move_player_time_since_startup(
    mut players: Query<&mut Transform, With<Sprite>>,
    time: Res<Time>,
) {
    for mut tx in players.iter_mut() {
        tx.translation = f32::sin(time.time_since_startup().as_secs_f32() * 3.0) * 500.0 * Vec3::X;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        ..default()
    });
}
