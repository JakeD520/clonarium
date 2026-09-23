use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 1. Position camera at the center of your 480x384 level
    commands.spawn((
        Camera2d,
        Transform::from_xyz(240.0, 192.0, 0.0),
    ));

    // 2. Select Level_0
    commands.insert_resource(LevelSelection::index(0));

    // 3. Spawn LDtk World
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle{handle:
            asset_server.load("maps/starter_base/starter_base.ldtk")
        },
        ..Default::default()
    });
}