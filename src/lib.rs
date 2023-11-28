use std::time::Duration;

use bevy::{
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

const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1211396483470153564541);

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

        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        let shader = Shader::from_wgsl(include_str!("./nine_slice.wgsl"), "./nine_slice.wgsl");
        shaders.insert(SHADER_HANDLE, shader);
    }
}

fn sync_nine_slice(
    nodes: Query<(&Node, &Handle<NineSliceMaterial>)>,
    mut materials: ResMut<Assets<NineSliceMaterial>>,
) {
    nodes.iter().for_each(|(node, handle)| {
        if let Some(mat) = materials.get_mut(handle) {
            mat.surface_size = node.size();
        }
    });
}

fn spawn_nine_slice(
    nodes: Query<(Entity, &NineSliceTexture, &Node), Without<Handle<NineSliceMaterial>>>,
    images: Res<Assets<Image>>,
    mut cmd: Commands,
    mut materials: ResMut<Assets<NineSliceMaterial>>,
) {
    nodes.iter().for_each(|(entity, nine_slice, node)| {
        let bounds = match nine_slice.bounds {
            Some(bounds) => bounds,
            None => match images.get(&nine_slice.atlas) {
                Some(img) => Rect::from_corners(Vec2::ZERO, img.size_f32()),
                // return if the image hasn't loaded yet
                None => return,
            },
        };

        let material = materials.add(NineSliceMaterial {
            atlas: nine_slice.atlas.clone(),
            surface_size: node.size(),
            bound_min: bounds.min,
            bound_max: bounds.max,
        });

        cmd.entity(entity)
            .remove::<NineSliceTexture>()
            .insert(material);
    });
}

#[derive(Component)]
pub struct NineSliceTexture {
    atlas: Handle<Image>,
    bounds: Option<Rect>,
}

impl NineSliceTexture {
    /// Create a new NineSliceTexture from an image
    pub fn from_image(image: Handle<Image>) -> Self {
        Self {
            atlas: image,
            bounds: None,
        }
    }

    /// Create a new NineSliceTexture from a slice of an atlas
    pub fn from_slice(atlas: Handle<Image>, bounds: Rect) -> Self {
        Self {
            atlas,
            bounds: Some(bounds),
        }
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct NineSliceMaterial {
    #[texture(0)]
    #[sampler(1)]
    atlas: Handle<Image>,
    #[uniform(2)]
    surface_size: Vec2,
    #[uniform(3)]
    bound_min: Vec2,
    #[uniform(4)]
    bound_max: Vec2,
}

impl UiMaterial for NineSliceMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(SHADER_HANDLE)
    }
}
