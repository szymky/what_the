use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_player, setup_camera))
        .add_systems(Update, player_movement_system)
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MovementStats {
    pub speed: f32,
    pub rotation_speed: f32,
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct WeaponCooldown(pub Timer);

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub movement: MovementStats,
    pub health: Health,
    pub cooldown: WeaponCooldown,

    pub sprite: Sprite,
    pub transform: Transform,
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        player: Player,
        movement: MovementStats {
            speed: 300.0,
            rotation_speed: 4.0,
        },
        health: Health {
            current: 100.0,
            max: 100.0,
        },
        cooldown: WeaponCooldown(Timer::from_seconds(0.25, TimerMode::Repeating)),
        sprite: Sprite::from_image(asset_server.load("sprites/ship.png")),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
    });
}

fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&MovementStats, &mut Transform), With<Player>>,
) {
    let Ok((stats, mut transform)) = query.single_mut() else {
        return;
    };

    let mut rotation = 0.0;
    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation -= 1.0;
    }
    transform.rotate_z(rotation * stats.rotation_speed * time.delta_secs());

    if keyboard_input.pressed(KeyCode::KeyW) {
        let forward = transform.up();
        transform.translation += forward * stats.speed * time.delta_secs();
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
