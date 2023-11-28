use bevy::prelude::*;
use bevy_nine_slice_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NineSlicePlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands, server: Res<AssetServer>) {
    cmd.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    })
    .with_children(|builder| {
        builder
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .with_children(|builder| {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(50.),
                            height: Val::Percent(100.),
                            display: Display::Flex,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(NineSliceTexture::new(server.load("panel.png")));

                builder
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(50.),
                            height: Val::Percent(100.),
                            display: Display::Flex,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(NineSliceTexture::new(server.load("panel.png")));
            });

        builder
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    display: Display::Flex,
                    ..default()
                },
                ..default()
            })
            .insert(NineSliceTexture::new(server.load("panel.png")));
    });

    cmd.spawn(Camera2dBundle::default());
}
