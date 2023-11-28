# Bevy nine slice/patch Material Plugin

Quick and easy auto-scaling nine slice/patch material for bevy ui nodes implemented as Fragment Shader.

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
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(50.),
            display: Display::Flex,
            ..default()
        },
        ..default()
    })
    .insert(NineSliceTexture::from_image(server.load("panel.png")));
}
```

### Using an atlas instead of a single image

Also added atlas capabilities. Instead of `from_image`, use `from_slice` method and pass the texture bounds.

```rust
.insert(NineSliceTexture::from_slice(
    server.load("panel_atlas.png"),
    Rect::new(0., 0., 32., 32.),
));
```

Check out the example

```rust
cargo run --example ui
```

result:

![Example](docs/example.jpeg)

### Compatibility

-   **Bevy 0.12**
