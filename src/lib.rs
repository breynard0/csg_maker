pub mod graphics;
pub mod utils;
pub mod object;

pub struct AppData<'a> {
    pub rendering_object: graphics::rendering_object::RenderingObject<'a>,
    pub world_object: object::world_object::WorldObject,
}