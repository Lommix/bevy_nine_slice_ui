use std::time::Duration;

use bevy::{
    asset::embedded_asset,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    time::common_conditions::on_timer,
};

pub mod prelude {
    pub use crate::{NineSliceMaterial, NineSlicePlugin, NineSliceTexture};
}

pub struct NineSlicePlugin {
    sync_rate_ms: u64,
}

impl Default for NineSlicePlugin {
    fn default() -> Self {
        Self { sync_rate_ms: 100 }
    }
}
impl Plugin for NineSlicePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiMaterialPlugin::<NineSliceMaterial>::default());
        app.add_systems(Update, spawn_nine_slice);
        app.add_systems(
            Update,
            sync_nine_slice.run_if(on_timer(Duration::from_millis(self.sync_rate_ms))),
        );

        embedded_asset!(app, "nine_slice.wgsl");
    }
}

fn sync_nine_slice(
    nodes: Query<(&Node, &Handle<NineSliceMaterial>)>,
    mut materials: ResMut<Assets<NineSliceMaterial>>,
) {
    nodes.iter().for_each(|(node, handle)| {
        if let Some(mat) = materials.get_mut(handle) {
            mat.size = node.size();
        }
    });
}

fn spawn_nine_slice(
    nodes: Query<(Entity, &NineSliceTexture, &Node), Without<Handle<NineSliceMaterial>>>,
    mut cmd: Commands,
    mut materials: ResMut<Assets<NineSliceMaterial>>,
) {
    nodes.iter().for_each(|(entity, nine_slice, node)| {
        let material = materials.add(NineSliceMaterial {
            atlas: nine_slice.0.clone(),
            size: node.size(),
        });
        cmd.entity(entity)
            .remove::<NineSliceTexture>()
            .insert(material);
    });
}

#[derive(Component)]
pub struct NineSliceTexture(Handle<Image>);
impl NineSliceTexture {
    pub fn new(image: Handle<Image>) -> Self {
        Self(image)
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct NineSliceMaterial {
    #[texture(0)]
    #[sampler(1)]
    atlas: Handle<Image>,

    #[uniform(2)]
    size: Vec2,
}

impl UiMaterial for NineSliceMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_nine_slice_ui/nine_slice.wgsl".into()
    }
}
