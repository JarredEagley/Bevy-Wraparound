// A simple background gradient using a full-screen quad.
// I could add a texture, but not doing that right now.

#import bevy_render::view::View;

// From: https://github.com/bevyengine/bevy/blob/c2da7800e3671ad92e775529070a814d0bc2f5f8/crates/bevy_sprite/src/mesh2d/mesh2d.wgsl
struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>;
#endif
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) debug: vec3<f32>,
#ifdef VERTEX_TANGENTS
    @location(3) world_tangent: vec4<f32>,
#endif
};

@group(0) @binding(0) var<uniform> view: View;

@vertex
fn vertex(
    vertex: Vertex
    ) -> VertexOutput {

    var out: VertexOutput;
    out.clip_position = vec4<f32>(vertex.position*2.0, 1.0);
    
    out.uv = vertex.uv;
    return out;
}

// Fragment below //

// From: https://github.com/bevyengine/bevy/blob/c2da7800e3671ad92e775529070a814d0bc2f5f8/crates/bevy_sprite/src/mesh2d/color_material.wgsl
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

// My custom material data struct.
struct BackgroundMaterial {
    color_bottom: vec4<f32>,    
    color_top: vec4<f32>, 
    y_scalar: f32,
}
@group(1) @binding(0) var<uniform> material: BackgroundMaterial;

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4f {
    let screen_coords = (in.uv - vec2(0.5, 0.5)) * vec2(2.0, -2.0); // now more like NDC.. We will "c" how that works out
    let world_space = view.inverse_view_proj * vec4(screen_coords, 0.0, 1.0);

    if (world_space.y < -50.0) {
        return vec4(0.03, 0.01, 0.01, 1.0);
    }

    let y: vec4<f32> = vec4<f32>(world_space.y * 0.01 * material.y_scalar);

    var atmosphere: vec4<f32> = mix(
        material.color_bottom,
        material.color_top, 
         y
        );
    atmosphere.w = mix(
        1.0,
        0.0,
        y.x * 0.5
    );

    let color = atmosphere;

    return color;
}