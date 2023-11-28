#import bevy_ui::ui_vertex_output::UiVertexOutput


@group(1) @binding(0)
var image: texture_2d<f32>;
@group(1) @binding(1)
var image_sampler: sampler;

@group(1) @binding(2)
var<uniform> surface_size: vec2<f32>;
@group(1) @binding(3)
var<uniform> bound_min: vec2<f32>;
@group(1) @binding(4)
var<uniform> bound_max: vec2<f32>;


@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
	let patch_uv = nine_patch_uv(in.uv);
	return get_tile(patch_uv, bound_min, bound_max);
}

fn nine_patch_uv(in_uv : vec2<f32>) -> vec2<f32> {
    let pixel_pos = in_uv * surface_size;
    let texture_size_px = bound_max - bound_min;
    let patch_size_px = texture_size_px / 3.0;
    let width = surface_size.x / patch_size_px.x;
    let height = surface_size.y / patch_size_px.y;
    let patch_pos = pixel_pos / patch_size_px;

    var patch_uv = patch_pos % 1.0;

	var border_x: f32;
	if (patch_pos.x < 1.0) {
		border_x = 0.0;
	} else if (pixel_pos.x > surface_size.x - patch_size_px.x) {
		patch_uv.x = 1. - ( surface_size.x - pixel_pos.x ) / patch_size_px.x;
		border_x = 2.0;
	} else {
		border_x = 1.0;
	}

	var border_y: f32;
	if (patch_pos.y < 1.0) {
		border_y = 0.0;
	} else if (pixel_pos.y > surface_size.y - patch_size_px.y) {
		patch_uv.y = 1. - ( surface_size.y - pixel_pos.y ) / patch_size_px.y;
		border_y = 2.0;
	} else {
		border_y = 1.0;
	}

    let border_uv = (patch_uv + vec2<f32>(border_x, border_y)) / 3.0;
	return border_uv;
}


fn get_tile(uv: vec2<f32>, min: vec2<f32>, max: vec2<f32>) -> vec4<f32> {
    var out: vec4<f32>;
    let texture_size = vec2<f32>(textureDimensions(image));
    let start_uv = min / texture_size;
    let end_uv = max / texture_size;
    let distance = abs(start_uv - end_uv);
    let tile_uv = start_uv + distance * uv;
    out = textureSample(image, image_sampler, tile_uv);
    return out;
}
