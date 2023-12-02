use bevy::prelude::*;
use bevy_nine_slice_ui::{prelude::*, NineSliceMaterialBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
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
                    .spawn(NineSliceMaterialBundle {
                        style: Style {
                            width: Val::Percent(50.),
                            height: Val::Percent(100.),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        nine_slice_texture: NineSliceTexture::from_image(server.load("panel.png")),
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn(TextBundle {
                            text: Text::from_section(
                                "Hello World",
                                TextStyle {
                                    font_size: 32.,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ),
                            ..default()
                        });
                    });

                builder
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(50.),
                            height: Val::Percent(100.),
                            display: Display::Flex,
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    })
                    .insert(NineSliceTexture::from_slice(
                        server.load("panel_atlas.png"),
                        Rect::new(0., 0., 32., 32.),
                    ));
            });

        builder
            .spawn(NineSliceMaterialBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                nine_slice_texture: NineSliceTexture::from_slice(
                    server.load("panel_atlas.png"),
                    Rect::new(32., 0., 32. + 48., 48.),
                ),
                ..default()
            })
            .with_children(|builder| {
                builder
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(150.),
                            height: Val::Px(50.),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(NineSliceTexture::from_slice(
                        server.load("panel_atlas.png"),
                        Rect::new(0., 0., 32., 32.),
                    ))
                    .with_children(|builder| {
                        builder.spawn(TextBundle {
                            text: Text::from_section(
                                "Button",
                                TextStyle {
                                    font_size: 32.,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            ),
                            ..default()
                        });
                    });
            });
    });

    cmd.spawn(Camera2dBundle::default());
}
