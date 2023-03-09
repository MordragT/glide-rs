#![feature(variant_count)]

use device::{Device, DeviceBuilder};
use pipeline::{PixelPipeline, PixelPipelineDescriptor, TexturePipeline};

pub mod command;
pub mod device;
pub mod pipeline;
pub mod sdk;

pub static mut STATE: *mut State = 0 as *mut State;

pub struct State {
    pub device_builder: DeviceBuilder,
    pub device: Option<Device>,
    pub pixel_pipeline_descriptor: PixelPipelineDescriptor,
    pub pixel_pipeline: Option<PixelPipeline<'static>>,
    pub texture_pipeline: Option<TexturePipeline>,
}

impl State {
    pub fn new() -> Self {
        let device_builder = DeviceBuilder::new();
        let pixel_pipeline_descriptor = PixelPipelineDescriptor {
            vertex_layouts: Vec::new(),
        };

        Self {
            device_builder,
            pixel_pipeline_descriptor,
            device: None,
            pixel_pipeline: None,
            texture_pipeline: None,
        }
    }

    pub fn update(&mut self) {
        todo!()
    }

    pub fn render(&mut self) {
        todo!()
    }
}

/// grGlideInit initializes the Glide library, performing tasks such as finding any installed graphics
/// subsystems, allocating memory, and initializing state variables. grGlideInit must be called before any
/// other Glide routines are called (the one exception is noted below).
#[allow(non_snake_case)]
pub unsafe extern "C" fn grGlideInit() {
    let mut state = State::new();
    STATE = &mut state as *mut State;
    std::mem::forget(state);
}
