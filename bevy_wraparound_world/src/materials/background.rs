use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;


/// A simple gradient material for the background of the scene.
#[derive(AsBindGroup, Debug, Clone, Asset, TypePath, TypeUuid)]
#[uuid = "7fdd80cb-0dc2-40b5-ab81-9b8f632b3eae"]
pub struct BackgroundMaterial {
    #[uniform(0)]
    pub color_bottom: Color,
    #[uniform(0)]
    pub color_top: Color, 
    #[uniform(0)]
    pub y_scalar: f32,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}