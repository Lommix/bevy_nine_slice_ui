use std::time::Duration;

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    time::common_conditions::on_timer,
    ui::FocusPolicy,
};

pub mod prelude {
    pub use crate::{NineSliceMaterial, NineSliceUiTexture, NineSliceUiPlugin};
}

pub struct NineSliceUiPlugin {
    sync_rate_ms: u64,
}

const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1211396483470153564541);

impl Default for NineSliceUiPlugin {
    fn default() -> Self {
        Self { sync_rate_ms: 100 }
    }
}
impl Plugin for NineSliceUiPlugin {
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

#[derive(Bundle, Clone, Debug)]
pub struct NineSliceUiMaterialBundle {
    /// Describes the logical size of the node
    pub node: Node,
    /// Styles which control the layout (size and position) of the node and it's children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// The nine slice component
    pub nine_slice_texture: NineSliceUiTexture,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    // pub background_color: BackgroundColor,
    /// The transform of the node
    ///
    /// This field is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This field is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

impl Default for NineSliceUiMaterialBundle {
    fn default() -> Self {
        Self {
            node: Default::default(),
            style: Default::default(),
            nine_slice_texture: NineSliceUiTexture::from_image(Handle::default()),
            focus_policy: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            z_index: Default::default(),
        }
    }
}

fn sync_nine_slice(
    nodes: Query<(&Node, &NineSliceUiTexture, &Handle<NineSliceMaterial>)>,
    images: Res<Assets<Image>>,
    mut materials: ResMut<Assets<NineSliceMaterial>>,
) {
    nodes.iter().for_each(|(node, nine_slice, handle)| {
        if let Some(mat) = materials.get_mut(handle) {
            let bounds = match nine_slice.bounds {
                Some(bounds) => bounds,
                None => match images.get(&nine_slice.atlas) {
                    Some(img) => Rect::from_corners(Vec2::ZERO, img.size_f32()),
                    None => return,
                },
            };

            mat.surface_size = node.size().extend(0.).extend(0.);
            mat.bounds.x = bounds.min.x;
            mat.bounds.y = bounds.min.y;
            mat.bounds.z = bounds.max.x;
            mat.bounds.w = bounds.max.y;
            mat.blend_color = nine_slice.blend_color.into();
            mat.mix.x = nine_slice.blend_mix;
            mat.mix.y = nine_slice.gradient_mix;

            if mat.atlas != nine_slice.atlas {
                mat.atlas = nine_slice.atlas.clone();
            }

            if mat.lookup_gradient != nine_slice.gradient {
                mat.lookup_gradient = nine_slice.gradient.clone();
            }
        }
    });
}

fn spawn_nine_slice(
    nodes: Query<(Entity, &NineSliceUiTexture, &Node), Without<Handle<NineSliceMaterial>>>,
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
            surface_size: node.size().extend(0.).extend(0.),
            bounds: Vec4::new(bounds.min.x, bounds.min.y, bounds.max.x, bounds.max.y),
            blend_color: nine_slice.blend_color.into(),
            lookup_gradient: nine_slice.gradient.clone(),
            mix: Vec4::new(nine_slice.blend_mix, nine_slice.gradient_mix, 0., 0.),
        });

        cmd.entity(entity)
            .remove::<BackgroundColor>()
            .insert(material);
    });
}

/// A component that describes a nine slice texture
#[derive(Component, Debug, Clone)]
pub struct NineSliceUiTexture {
    /// The atlas to use for the nine slice
    pub atlas: Handle<Image>,
    /// The bounds of the nine slice in the atlas
    pub bounds: Option<Rect>,
    /// The color to blend the nine slice with, alpha is used for blending
    pub blend_color: Color,
    pub blend_mix: f32,
    /// A 1D texture to use as color lookup, the grayscale value of the original color is used as UV
    /// dark to light, left to right
    pub gradient: Handle<Image>,
    pub gradient_mix: f32,
}

impl NineSliceUiTexture {
    /// Create a new NineSliceTexture from an image
    pub fn from_image(image: Handle<Image>) -> Self {
        Self {
            atlas: image,
            bounds: None,
            blend_color: Color::default(),
            blend_mix: 0.0,
            gradient: Handle::default(),
            gradient_mix: 0.0,
        }
    }

    /// Create a new NineSliceTexture from a slice of an atlas
    pub fn from_slice(atlas: Handle<Image>, bounds: Rect) -> Self {
        Self {
            atlas,
            bounds: Some(bounds),
            blend_color: Color::default(),
            blend_mix: 0.0,
            gradient: Handle::default(),
            gradient_mix: 0.0,
        }
    }

    pub fn with_lookup_gradient(mut self, gradiant: Handle<Image>) -> Self {
        self.gradient = gradiant;
        self
    }

    pub fn with_blend_mix(mut self, blend: f32) -> Self {
        self.blend_mix = blend;
        self
    }

    pub fn with_gradient_mix(mut self, blend: f32) -> Self {
        self.gradient_mix = blend;
        self
    }

    pub fn with_blend_color(mut self, color: Color) -> Self {
        self.blend_color = color;
        self
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct NineSliceMaterial {
    #[texture(0)]
    #[sampler(1)]
    atlas: Handle<Image>,
    #[uniform(2)]
    surface_size: Vec4,
    #[uniform(3)]
    bounds: Vec4,
    #[uniform(4)]
    blend_color: Vec4,
    #[texture(5)]
    #[sampler(6)]
    lookup_gradient: Handle<Image>,
    #[uniform(7)]
    mix: Vec4,
}

impl UiMaterial for NineSliceMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(SHADER_HANDLE)
    }
}
