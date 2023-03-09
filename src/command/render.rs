pub enum CoordinateSpace {
    /// Window coordinates are relative to the origin of the window
    Window,
    /// Clip coordinates are relative to a viewport defined with the new command grViewport()
    Clip,
}

struct Viewport {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    min_depth: f32,
    max_depth: f32,
}

pub struct RenderPassDescriptor {
    pub coordinate_space: CoordinateSpace,
}

/*
When window coordinates are used, the application performs the coordinate divisions by w, providing
x/w, y/w, z/w, 1/w, s/w, t/w, and q/w as necessary in the vertex structure (only x/w and y/w are
mandatory). Window coordinates may be less than optimal on future hardware that can perform
perspective division and viewport transformations.
When clip coordinates are used, the division by w is performed automatically. The minimal vertex
specifies x, y, and w. If z buffering is enabled, z should be in the range [−w..+w]; otherwise, z data need
not be given. Glide will automatically compute x/w, y/w, z/w, and 1/w, perform vertex snapping on the
results, and then apply the viewport transformation to get window coordinates. Texture coordinates s
and t are in the range [0..1] for all texture sizes and aspect ratios. Glide automatically computes s/w,
t/w, and q/w.
Clip space coordinates are recommended for all new applications. It is likely that future hardware will
perform the viewport transformation and depth range computations to further off-load the CPU.
 */

/// We need to specify how many color and depth buffers there will be,
/// how many samples to use for each of them and how their contents should be handled throughout the rendering operations.
/// All of this information is wrapped in a render pass object
pub struct RenderPass<'a> {
    render_pass: wgpu::RenderPass<'a>,
    viewport: Viewport,
    coordinate_space: CoordinateSpace,
}

impl<'a> RenderPass<'a> {
    /// Specifies the viewport transformation. The current grSstOrigin() setting determines
    /// whether x and y specify the upper left corner or the lower left corner. Negative width and height are
    /// allowed and mirror the image about the x or y axis
    pub fn set_viewport(&mut self, x: f32, y: f32, width: f32, height: f32) {
        todo!()
    }
    /// When using clip coordinates, the grDepthRange() command specifies the viewport parameters for the
    /// depth component.
    /// If z buffering, clip-space z is in the range [-w..+w]. After division by w, z is in the range [-1..1] which is
    /// mapped to the depth buffer according to [near.. far], where [near=0.. far=1] represents the entire range
    /// of the depth buffer. grDepthRange() is ignored unless clip coordinates are being used and z buffering is
    /// enabled.
    pub fn set_depth_range(&mut self, near: f32, far: f32) {
        todo!()
    }
}
