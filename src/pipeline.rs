use log::debug;
use num_enum::TryFromPrimitive;
use wgpu::VertexFormat;

use crate::STATE;

/// Fragment Shader
pub struct TexturePipeline {}

pub struct VertexLayout {
    pub attributes: [Option<VertexAttribute>; std::mem::variant_count::<VertexParam>()],
}

impl VertexLayout {
    pub fn generate_vertex_shader(&self) -> String {
        let mut layout = String::new();
        let mut main = String::new();

        for attr in self.attributes.iter() {
            if let Some(attr) = attr {
                let param = attr.param;
                layout.push_str(&param.wgsl_layout());
                let computation = match param {
                    VertexParam::PositionXy => {
                        format!("v_position = vec4({}, 0.0, 1.0);", param.label())
                    }
                    VertexParam::PositionZ => format!("v_position.z = {};", param.label()),
                    VertexParam::HomogenousCoordinate => {
                        format!("v_position.w = {};", param.label())
                    }
                    VertexParam::Color => format!("v_color = vec4({}, 1.0);", param.label()),
                    VertexParam::Alpha => format!("v_color.w = {};", param.label()),
                    VertexParam::PackedColor => format!("v_color = vec4({});", param.label()),
                    _ => todo!(),
                };
                main.push_str(&computation);
            }
        }

        format!(
            "#version 450
        {layout}
        layout(location=0) out vec4 v_position;
        layout(location=1) out vec4 v_color;

        void main() {{
            {main}
            gl_Position = v_position;
        }}
        "
        )
    }
}

/// grVertexLayout() is called once for each value of param, chosen from the values in the first column of
/// Table 2.1 or Table 2.2 (there is a table for each coordinate space option).
/// offset is either the offset in bytes of the parameter data from the vertex pointer. The offset can be either
/// positive or negative. Align data on word boundaries for optimal performance.
/// mode is either GR_PARAM_ENABLE or GR_PARAM_DISABLE. Disabling a parameter will potentially cause
/// it to inherit the last known value. When a parameter is disabled, the offset argument is ignored.
/// Disabling a mandatory parameter like GR_PARAM_XY will cause a fatal Glide error.
/// Page 23 and 24 in the programming manual.
pub struct VertexAttribute {
    param: VertexParam,
    offset: u64,
}

impl From<VertexAttribute> for wgpu::VertexAttribute {
    fn from(attr: VertexAttribute) -> Self {
        let VertexAttribute { param, offset } = attr;
        let format = param.format();
        let shader_location = param.shader_location();
        wgpu::VertexAttribute {
            format,
            offset,
            shader_location,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum VertexParam {
    PositionXy,
    PositionZ,
    /// Used to create the clip space, coordinates outside of -1.0 and 1.0 are clipped
    HomogenousCoordinate,
    DepthIndicator,
    Color,
    PackedColor,
    Alpha,
    TexCoordinates1,
    TexCoordinates2,
    TexCoordinates3,
    FogIndex,
}

impl VertexParam {
    pub const fn shader_location(&self) -> u32 {
        match self {
            Self::PositionXy => 0,
            Self::PositionZ => 1,
            Self::HomogenousCoordinate => 2,
            Self::DepthIndicator => 3,
            Self::Color => 4,
            Self::PackedColor => 5,
            Self::Alpha => 6,
            Self::TexCoordinates1 => 7,
            Self::TexCoordinates2 => 8,
            Self::TexCoordinates3 => 9,
            Self::FogIndex => 10,
        }
    }

    pub const fn format(&self) -> VertexFormat {
        match self {
            Self::PositionXy => VertexFormat::Float32x2,
            Self::PositionZ => VertexFormat::Float32,
            Self::HomogenousCoordinate => VertexFormat::Float32,
            Self::DepthIndicator => VertexFormat::Float32,
            Self::Color => VertexFormat::Float32x3,
            Self::PackedColor => VertexFormat::Uint8x4,
            Self::Alpha => VertexFormat::Float32,
            Self::TexCoordinates1 => VertexFormat::Float32x2,
            Self::TexCoordinates2 => VertexFormat::Float32x2,
            Self::TexCoordinates3 => VertexFormat::Float32x2,
            Self::FogIndex => VertexFormat::Float32,
        }
    }

    pub const fn label(&self) -> &'static str {
        match self {
            VertexParam::PositionXy => "xy_position",
            VertexParam::PositionZ => "z_position",
            VertexParam::HomogenousCoordinate => "w_coordinate",
            VertexParam::Alpha => "alpha",
            VertexParam::Color => "color",
            VertexParam::PackedColor => "packed_color",
            VertexParam::FogIndex => "fog",
            VertexParam::DepthIndicator => "depth",
            VertexParam::TexCoordinates1 => "tex_coordinates_1",
            VertexParam::TexCoordinates2 => "tex_coordinates_2",
            VertexParam::TexCoordinates3 => "tex_coordinates_3",
        }
    }

    pub fn wgsl_layout(&self) -> String {
        let label = self.label();
        let format = match self.format() {
            VertexFormat::Float32 => "f32",
            VertexFormat::Float32x2 => "vec2",
            VertexFormat::Float32x3 => "vec3",
            VertexFormat::Uint8x4 => "uvec4",
            _ => unreachable!(),
        };
        let location = self.shader_location();

        format!("layout(location={location}) in {format} {label};\n")
    }
}

pub struct PixelPipelineDescriptor {
    pub vertex_layouts: Vec<VertexLayout>,
}

impl PixelPipelineDescriptor {
    fn gr_vertex_layout(&mut self, param: GrParam, offset: i32) {
        todo!()
    }
}

/// Vertex Shader
pub struct PixelPipeline<'a> {
    vertex_buffer_layout: wgpu::VertexBufferLayout<'a>,
}

impl<'a> PixelPipeline<'a> {
    /// grGlideGetVertexLayout() makes a copy of the current vertex layout established by calls to grVertexLayout()
    pub fn get_vertex_layout(&self) -> &wgpu::VertexBufferLayout {
        &self.vertex_buffer_layout
    }
    /// The application can restore the saved layout by calling grGlideSetVertexLayout().
    /// Use grGet(GR_GLIDE_VERTEXLAYOUT_SIZE,…) to determine how much space is needed (and hence,
    /// how big the layout buffer should be).
    pub fn set_vertex_layout(&mut self, layout: wgpu::VertexBufferLayout) {
        // is this even possible in wgpu to change the layout ?
        // maybe when changing the shader at runtime ?
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
enum GrParam {
    XY = 0x01,
    Z = 0x02,
    W = 0x03,
    Q = 0x04,
    FogExt = 0x05,
    A = 0x10,
    RGB = 0x20,
    PARGB = 0x30,
    ST0 = 0x40,
    ST1 = 0x41,
    ST2 = 0x42,
    Q0 = 0x50,
    Q1 = 0x51,
    Q2 = 0x52,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
enum GrMode {
    Disable = 0x00,
    Enable = 0x01,
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grVertexLayout(param: u32, offset: i32, mode: u32) {
    debug!("grVertexLayout");
    let mode = GrMode::try_from(mode).unwrap();
    if mode == GrMode::Enable {
        let state = unsafe { STATE.as_mut() }.unwrap();

        let param = GrParam::try_from(param).unwrap();
        state
            .pixel_pipeline_descriptor
            .gr_vertex_layout(param, offset);
    }
}
