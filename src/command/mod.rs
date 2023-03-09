use self::render::RenderPass;

pub mod draw;
pub mod render;

pub struct CommandEncoder {}

impl CommandEncoder {
    pub fn begin_render_pass(&mut self) -> RenderPass {
        todo!()
    }
}
