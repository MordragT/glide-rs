#![feature(unsized_locals)]

use glam::UVec2;
use num_enum::TryFromPrimitive;
use raw_window_handle::HasRawWindowHandle;
use wgpu::util::DeviceExt;

mod api;
//mod constants;
mod kinds;

pub static mut STATE: *mut State = 0 as *mut State;

// pub struct GlideState {
//     pub wgpu_state: Option<WgpuState>,
// }

// impl Default for GlideState {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl GlideState {
//     pub const fn new() -> Self {
//         Self { wgpu_state: None }
//     }
// }

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    resolution: UVec2,
    //vertex_buffer: wgpu::Buffer,
    pub vertex_layout: wgpu::VertexBufferLayout<'static>,
    pub vertex_attributes: Vec<wgpu::VertexAttribute>,
    pub color_format: wgpu::TextureFormat,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new<T: HasRawWindowHandle>(
        window: &T,
        resolution: UVec2,
        color_format: wgpu::TextureFormat,
    ) -> Self {
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: resolution.x,
            height: resolution.y,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        // let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Vertex Buffer"),
        //     contents: &[],
        //     usage: wgpu::BufferUsage::VERTEX,
        // });
        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            resolution,
            vertex_layout: wgpu::VertexBufferLayout {
                array_stride: 0,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &[],
            },
            vertex_attributes: vec![],
            color_format,
        }
    }

    fn resize(&mut self, new_resolution: UVec2) {
        self.resolution = new_resolution;
        self.sc_desc.width = new_resolution.x;
        self.sc_desc.height = new_resolution.y;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
                label: Some("Render Pass"),
            });
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum GrEnableMode {
    //Disable = 0x0,
    //Enable = 0x1,
    AAOrdered = 0x01,
    AllowMipMapDither = 0x02,
    Passthru = 0x03,
    ShamelessPlug = 0x04,
    VideoSmoothing = 0x05,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum GrParam {
    //Disable = 0x00,
    //Enable = 0x01,
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
