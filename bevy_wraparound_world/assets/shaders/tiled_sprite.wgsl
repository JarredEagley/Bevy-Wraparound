// Tiled sprite material, simply multiplies the uv coordinates.

#import bevy_render::view::View;


struct TiledSpriteMaterial {
    color: vec4<f32>,
    uv_scale: vec2<f32>,
};

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) debug: vec3<f32>,
#ifdef VERTEX_TANGENTS
    @location(3) world_tangent: vec4<f32>,
#endif
};

@group(0) @binding(0) var<uniform> view: View;

@group(1) @binding(0) var<uniform> material: TiledSpriteMaterial;
@group(1) @binding(1) texture: texture_2d<f32>;
@group(1) @binding(2) texutre_sampler: sampler;

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec3f {
    return vec4<f32>(1.0, 1.0, 0.0, 1.0);
}