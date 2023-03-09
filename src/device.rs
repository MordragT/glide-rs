use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawWindowHandle};
use thiserror::Error;
use wgpu::CreateSurfaceError;
use winit::{
    dpi::{PhysicalSize, Size},
    error::OsError,
    event_loop::EventLoop,
    window::Window,
};

use crate::{
    command::CommandEncoder,
    pipeline::{PixelPipeline, TexturePipeline},
    State,
};

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("OsError: {0}")]
    Os(#[from] OsError),
    #[error("CreateSurfaceError: {0}")]
    CreateSurface(#[from] CreateSurfaceError),
}

/*
grSstWinOpen() must be called once for each graphics subsystem that will be used. In Glide 3.0, the
current graphics context must be closed (by calling grSstWinClose(), described below) before
grSstWinOpen() can be called to open a context for another subsystem. Note that two graphics
subsystems linked together in a scanline interleaving configuration are treated in software as a single
unit.
 */

pub struct DeviceBuilder {
    size: Option<PhysicalSize<u32>>,
    refresh: Option<u32>,
}

impl DeviceBuilder {
    pub fn new() -> Self {
        Self {
            size: None,
            refresh: None,
        }
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.size = Some(PhysicalSize { width, height });
        self
    }

    pub fn with_refresh(mut self, refresh: u32) -> Self {
        self.refresh = Some(refresh);
        self
    }

    pub async fn build(self) -> Result<Device, DeviceError> {
        let event_loop = EventLoop::new();
        let window_builder = winit::window::WindowBuilder::new();
        let window_builder = match self.size {
            Some(size) => window_builder.with_inner_size(size),
            None => window_builder,
        };
        let window = window_builder.build(&event_loop)?;

        let instance = wgpu::Instance::new(Default::default());
        let surface = unsafe { instance.create_surface(&window) }?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();
        // TODO config für color format siehe glide grSstWinOpen
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);

        Ok(Device {
            event_loop,
            window,
            device,
            queue,
            surface,
        })
    }
}

pub struct Device {
    event_loop: EventLoop<()>,
    window: Window,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
}

impl Device {
    // Copy state
    pub fn get_state(&self) -> State {
        todo!()
    }
    pub fn set_state(&mut self, state: State) {
        todo!()
    }
    pub fn create_pixel_pipeline(&mut self) -> PixelPipeline {
        // let layout = self
        //     .device
        //     .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        //         label: None,
        //         bind_group_layouts: &[],
        //         push_constant_ranges: &[],
        //     });
        // self.device
        //     .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        //         label: None,
        //         layout,

        //     })
        todo!()
    }

    pub fn create_texture_pipeline(&mut self) -> TexturePipeline {
        todo!()
    }

    pub fn create_command_encoder(&mut self) -> CommandEncoder {
        todo!()
    }
}

/// Applications that are written to run on a variety of hardware configurations can query for available
/// resolutions before calling grSstWinOpen().
/// grQueryResolutions() returns all available frame buffer configurations that match the constraints
/// specified in the template resTemplate. The constraints are specified as either GR_QUERY_ANY or a
/// specific value in each of the four fields in the GrResolution structure. If output is NULL,
pub struct Resolution {
    size: PhysicalSize<u32>,
    refresh: u32,
    num_color_buffers: u32,
    num_aux_buffers: u32,
}

impl Default for Resolution {
    fn default() -> Self {
        Self {
            size: PhysicalSize {
                width: 640,
                height: 480,
            },
            refresh: 60,
            num_color_buffers: todo!(),
            num_aux_buffers: todo!(),
        }
    }
}

pub struct Tmu {}
