use bevy::{
    prelude::*, 
    render::render_resource::{
        AsBindGroup, 
        ShaderRef
    }, 
    reflect::TypeUuid, 
    sprite::Material2d
};


///  A simple sprite material, which can handle UV tiling.
#[derive(AsBindGroup, Debug, Clone, Asset, TypePath, TypeUuid)]
#[uuid = "add9eda6-340b-451f-8349-4a41c8e6adb7"]
pub struct TiledSpriteMaterial {
    #[uniform(0)]
    pub color: Color,

}

impl Material2d for TiledSpriteMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tiled_sprite.wgsl".into()
    }
}