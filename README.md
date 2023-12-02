# Bevy nine slice/patch Material Plugin

Quick and easy auto-scaling nine slice/patch material for bevy ui nodes implemented as Fragment Shader.

**Now WASM compatible!**

```bash
cargo add bevy_nine_slice_ui
```

## Usage

It's a single component.

```rust
app.add_plugin(NineSliceUiPlugin::default());
```

```rust
fn spawn_ui(mut cmd: Commands, server: Res<AssetServer>) {
    commands.spawn(NineSliceMaterialBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(50.),
            display: Display::Flex,
            ..default()
        },
        nine_slice_texture: NineSliceTexture::from_image(server.load("panel_atlas.png")),
        ..default()
    });
}
```

### Using an atlas instead of a single image

Also added atlas capabilities. Instead of `from_image`, use `from_slice` method and pass the texture bounds.

```rust
nine_slice_texture: NineSliceTexture::from_slice(
    server.load("panel_atlas.png"),
    Rect::new(32., 0., 32. + 48., 48.),
),
```

### Adding a material to an existing Bundle

You can also add a nine slice material to an existing bundle, like a button. Just beware, this might not always work. Just in the recent 12.1 update,
the background color component broke the material. The background color component will now be removed from elements where the material is added.

```rust
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
```

Check out the example

```rust
cargo run --example ui
```

result:

![Example](docs/example.jpeg)

### Compatibility

-   **Bevy 0.12**
