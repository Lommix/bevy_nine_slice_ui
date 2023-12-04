use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_nine_slice_ui::{prelude::*, NineSliceUiMaterialBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(NineSliceUiPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            super_simple_animation.run_if(on_timer(Duration::from_millis(250))),
        )
        .run();
}

// keeps track of the current animation frame
#[derive(Component)]
pub struct Animation(usize);

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
                    .spawn(NineSliceUiMaterialBundle {
                        style: Style {
                            width: Val::Percent(50.),
                            height: Val::Percent(100.),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        nine_slice_texture: NineSliceUiTexture::from_image(
                            server.load("panel.png"),
                        ),
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
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    })
                    .insert(
                        NineSliceUiTexture::from_slice(
                            server.load("panel_atlas.png"),
                            Rect::new(0., 0., 32., 32.),
                        )
                        .with_blend_color(Color::RED)
                        .with_blend_mix(0.5),
                    )
                    .with_children(|builder| {
                        builder.spawn(TextBundle {
                            text: Text::from_section(
                                "50 % mix Red",
                                TextStyle {
                                    font_size: 50.,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            ),
                            ..default()
                        });
                    });
            });

        builder
            .spawn(NineSliceUiMaterialBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                nine_slice_texture: NineSliceUiTexture::from_slice(
                    server.load("panel_atlas.png"),
                    Rect::new(32., 0., 32. + 48., 48.),
                ),
                ..default()
            })
            .with_children(|builder| {
                builder
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(180.),
                            height: Val::Px(50.),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Animation(0))
                    .insert(NineSliceUiTexture::from_slice(
                        server.load("panel_animation.png"),
                        Rect::new(0., 0., 32., 32.),
                    ))
                    .with_children(|builder| {
                        builder.spawn(TextBundle {
                            text: Text::from_section(
                                "Animated button",
                                TextStyle {
                                    font_size: 20.,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            ),
                            ..default()
                        });
                    });
                builder
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(180.),
                            height: Val::Px(50.),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(
                        NineSliceUiTexture::from_slice(
                            server.load("panel_animation.png"),
                            Rect::new(0., 0., 32., 32.),
                        )
                        .with_lookup_gradient(server.load("4-color-palette.png"))
                        .with_gradient_mix(1.0),
                    )
                    .with_children(|builder| {
                        builder.spawn(TextBundle {
                            text: Text::from_section(
                                "Lookup gradient",
                                TextStyle {
                                    font_size: 20.,
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

fn super_simple_animation(mut query: Query<(&mut NineSliceUiTexture, &mut Animation)>) {
    query.iter_mut().for_each(|(mut nine_slice, mut frame)| {
        match &mut nine_slice.bounds {
            Some(bounds) => {
                bounds.min.x = frame.0 as f32 * 32.;
                bounds.max.x = 32. + frame.0 as f32 * 32.;
                frame.0 = (frame.0 + 1) % 4;
            }
            None => (),
        };
    });
}
