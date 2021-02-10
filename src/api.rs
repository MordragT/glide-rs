#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use wgpu::VertexBufferLayout;

use crate::kinds::*;
use crate::*;
use std::{borrow::BorrowMut, convert::TryFrom, ffi::c_void};

#[allow(non_snake_case)]
pub unsafe extern "C" fn grDrawPoint(point: *const c_void) {
    // grEnable, grVertexLayout
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grDrawLine(v1: *const c_void, v2: *const c_void) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grDrawTriangle(a: *const c_void, b: *const c_void, c: *const c_void) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grVertexLayout(param: FxU32, offset: FxI32, mode: FxU32) {
    let param = match GrParam::try_from(param) {
        Ok(p) => p,
        Err(_) => panic!(), // TODO: error handling
    };
    let mode = match mode {
        0 => false,
        1 => true,
        _ => panic!(), // TODO: error handling
    };
    let format = match param {
        GrParam::XY => wgpu::VertexFormat::Float2,
        GrParam::Z => wgpu::VertexFormat::Float,
        GrParam::W => wgpu::VertexFormat::Float,
        GrParam::Q => wgpu::VertexFormat::Float,
        GrParam::ST0 => wgpu::VertexFormat::Float2,
        GrParam::ST1 => wgpu::VertexFormat::Float2,
        GrParam::ST2 => wgpu::VertexFormat::Float2,
        GrParam::Q0 => wgpu::VertexFormat::Float,
        GrParam::Q1 => wgpu::VertexFormat::Float,
        GrParam::Q2 => wgpu::VertexFormat::Float,
        GrParam::A => wgpu::VertexFormat::Float,
        GrParam::RGB => wgpu::VertexFormat::Float3,
        GrParam::PARGB => wgpu::VertexFormat::Uint,
        GrParam::FogExt => wgpu::VertexFormat::Float,
    };
    let buffer_layout = wgpu::VertexBufferLayout {
        array_stride: (*STATE).vertex_layout.array_stride + format.size(),
        step_mode: wgpu::InputStepMode::Vertex,
        attributes: {
            let last = (*STATE).vertex_attributes.last().unwrap();
            (*STATE).vertex_attributes.push(wgpu::VertexAttribute {
                offset: last.offset + last.format.size(),
                shader_location: last.shader_location + 1,
                format,
            });
            (*STATE).vertex_attributes.as_slice()
        },
    };
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grDrawVertexArray(mode: FxU32, Count: FxU32, pointers: *mut c_void) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grDrawVertexArrayContiguous(
    mode: FxU32,
    count: FxU32,
    pointers: *mut c_void,
    stride: FxU32,
) {
    todo!();
}

/*
**  Antialiasing Functions
*/

#[allow(non_snake_case)]
pub unsafe extern "C" fn grAADrawTriangle(
    a: *const c_void,
    b: *const c_void,
    c: *const c_void,
    antialiasAb: FxBool,
    antialiasBC: FxBool,
    antialiasCA: FxBool,
) {
    todo!();
}

/*
** buffer management
*/
#[allow(non_snake_case)]
pub unsafe extern "C" fn grBufferClear(color: GrColor_t, alpha: GrAlpha_t, depth: FxU32) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grBufferSwap(swap_interval: FxU32) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grRenderBuffer(buffer: GrBuffer_t) {
    todo!();
}

/*
** error management
*/

#[allow(non_snake_case)]
pub unsafe extern "C" fn grErrorSetCallback(fnc: GrErrorCallbackFnc_t) {
    todo!();
}

/*
** SST routines
*/
#[allow(non_snake_case)]
pub unsafe extern "C" fn grFinish() {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grFlush() {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grSstWinOpen(
    window_handle: FxU32,
    screen_resolution: GrScreenResolution_t,
    refresh_rate: GrScreenRefresh_t,
    color_format: GrColorFormat_t,
    origin_location: GrOriginLocation_t,
    nColBuffers: FxI32,
    nAuxBuffers: FxI32,
) -> GrContext_t {
    #[cfg(target_os = "windows")]
    let window_handle = {
        let h = if window_handle == 0 {
            winapi::um::winuser::GetActiveWindow() as *mut c_void
        } else {
            window_handle as *mut c_void
        };
        let instance = 
        raw_window_handle::windows::WindowsHandle 
    };

    #[cfg(target_os = "linux")]
    let window_handle = if window_handle == 0 {
        panic!()
    } else {
        window_handle
    };
    let color_format = match color_format {
        GrColorFormat_t::GR_COLORFORMAT_ABGR => panic!(),
        GrColorFormat_t::GR_COLORFORMAT_ARGB => panic!(),
        GrColorFormat_t::GR_COLORFORMAT_BGRA => wgpu::TextureFormat::Bgra8Unorm,
        GrColorFormat_t::GR_COLORFORMAT_RGBA => wgpu::TextureFormat::Rgba8Unorm,
    };
    let resolution = match screen_resolution {
        GrScreenResolution_t::GR_RESOLUTION_320x200 => UVec2::from([320, 200]),
        GrScreenResolution_t::GR_RESOLUTION_320x240 => UVec2::from([320, 240]),
        GrScreenResolution_t::GR_RESOLUTION_400x256 => UVec2::from([400, 256]),
        GrScreenResolution_t::GR_RESOLUTION_512x384 => UVec2::from([512, 384]),
        GrScreenResolution_t::GR_RESOLUTION_640x200 => UVec2::from([640, 200]),
        GrScreenResolution_t::GR_RESOLUTION_640x350 => UVec2::from([640, 350]),
        GrScreenResolution_t::GR_RESOLUTION_640x400 => UVec2::from([640, 400]),
        GrScreenResolution_t::GR_RESOLUTION_640x480 => UVec2::from([640, 480]),
        GrScreenResolution_t::GR_RESOLUTION_800x600 => UVec2::from([800, 600]),
        GrScreenResolution_t::GR_RESOLUTION_960x720 => UVec2::from([960, 720]),
        GrScreenResolution_t::GR_RESOLUTION_856x480 => UVec2::from([856, 480]),
        GrScreenResolution_t::GR_RESOLUTION_512x256 => UVec2::from([512, 256]),
        GrScreenResolution_t::GR_RESOLUTION_1024x768 => UVec2::from([1024, 768]),
        GrScreenResolution_t::GR_RESOLUTION_1280x1024 => UVec2::from([1280, 1024]),
        GrScreenResolution_t::GR_RESOLUTION_1600x1200 => UVec2::from([1600, 1200]),
        GrScreenResolution_t::GR_RESOLUTION_400x300 => UVec2::from([400, 300]),
        GrScreenResolution_t::GR_RESOLUTION_NONE => UVec2::from([1280, 1024]),
    };
    let state = futures::executor::block_on(State::new(&window_handle, resolution, color_format));
    STATE = &mut state as *mut State;
    std::mem::forget(state);
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grSstWinClose(context: GrContext_t) -> FxBool {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grSetNumPendingBuffers(NumPendingBuffers: FxI32) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grSelectContext(context: GrContext_t) -> FxBool {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grSstOrigin(origin: GrOriginLocation_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grSstSelect(_which_sst: FxI32) {
    /*The ordinal number of the graphics subsystem to make current. This value must be
    between 0 and the number of installed subsystems returned by
    grGet(GR_NUM_BOARDS,...) .*/
    //todo!();
}

/*
** Glide configuration and special effect maintenance functions
*/
#[allow(non_snake_case)]
pub unsafe extern "C" fn grAlphaBlendFunction(
    rgb_sf: GrAlphaBlendFnc_t,
    rgb_df: GrAlphaBlendFnc_t,
    alpha_sf: GrAlphaBlendFnc_t,
    alpha_df: GrAlphaBlendFnc_t,
) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grAlphaCombine(
    function: GrCombineFunction_t,
    factor: GrCombineFactor_t,
    local: GrCombineLocal_t,
    other: GrCombineOther_t,
    invert: FxBool,
) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grAlphaControlsITRGBLighting(enable: FxBool) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grAlphaTestFunction(function: GrCmpFnc_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grAlphaTestReferenceValue(value: GrAlpha_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grChromakeyMode(mode: GrChromakeyMode_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grChromakeyValue(value: GrColor_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grClipWindow(minx: FxU32, miny: FxU32, maxx: FxU32, maxy: FxU32) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grColorCombine(
    function: GrCombineFunction_t,
    factor: GrCombineFactor_t,
    local: GrCombineLocal_t,
    other: GrCombineOther_t,
    invert: FxBool,
) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grColorMask(rgb: FxBool, a: FxBool) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grCullMode(mode: GrCullMode_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grConstantColorValue(value: GrColor_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grDepthBiasLevel(level: FxI32) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grDepthBufferFunction(function: GrCmpFnc_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grDepthBufferMode(mode: GrDepthBufferMode_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grDepthMask(mask: FxBool) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grDisableAllEffects() {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grDitherMode(mode: GrDitherMode_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grFogColorValue(fogcolor: GrColor_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grFogMode(mode: GrFogMode_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grFogTable(ft: *const GrFog_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grLoadGammaTable(
    nentries: FxU32,
    red: *mut FxU32,
    green: *mut FxU32,
    blue: *mut FxU32,
) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grSplash(
    x: FxFloat,
    y: FxFloat,
    width: FxFloat,
    height: FxFloat,
    frame: FxU32,
) {
    todo!();
}
#[allow(non_snake_case)]
pub unsafe extern "C" fn grGet(pname: FxU32, plength: FxU32, params: *mut FxI32) -> FxU32 {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grGetString(pname: FxU32) -> *mut char {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grQueryResolutions(
    resTemplate: *const GrResolution,
    output: *mut GrResolution,
) -> FxI32 {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grReset(what: FxU32) -> FxBool {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grGetProcAddress(procName: *mut char) -> GrProc {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grEnable(mode: GrEnableMode_t) {
    let mode = match GrEnableMode::try_from(mode) {
        Ok(m) => m,
        Err(_) => panic!(), // TODO: Error Handling
    };
    //STATE.mode = mode;
    //todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grDisable(mode: GrEnableMode_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grCoordinateSpace(mode: GrCoordinateSpaceMode_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grDepthRange(n: FxFloat, f: FxFloat) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grStippleMode(mode: GrStippleMode_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grStipplePattern(mode: GrStipplePattern_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grViewport(x: FxI32, y: FxI32, width: FxI32, height: FxI32) {
    todo!();
}

/*
** texture mapping control functions
*/
#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexCalcMemRequired(
    lodmin: GrLOD_t,
    lodmax: GrLOD_t,
    aspect: GrAspectRatio_t,
    fmt: GrTextureFormat_t,
) -> FxU32 {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexTextureMemRequired(evenOdd: FxU32, info: *mut GrTexInfo) -> FxU32 {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexMinAddress(tmu: GrChipID_t) -> FxU32 {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexMaxAddress(tmu: GrChipID_t) -> FxU32 {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexNCCTable(table: GrNCCTable_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexSource(
    tmu: GrChipID_t,
    startAddress: FxU32,
    evenOdd: FxU32,
    info: *mut GrTexInfo,
) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexClampMode(
    tmu: GrChipID_t,
    s_clampmode: GrTextureClampMode_t,
    t_clampmode: GrTextureClampMode_t,
) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexCombine(
    tmu: GrChipID_t,
    rgb_function: GrCombineFunction_t,
    rgb_factor: GrCombineFactor_t,
    alpha_function: GrCombineFunction_t,
    alpha_factor: GrCombineFactor_t,
    rgb_invert: FxBool,
    alpha_invert: FxBool,
) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexDetailControl(
    tmu: GrChipID_t,
    lod_bias: FxI32,
    detail_scale: FxU8,
    detail_max: FxFloat,
) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexFilterMode(
    tmu: GrChipID_t,
    minfilter_mode: GrTextureFilterMode_t,
    magfilter_mode: GrTextureFilterMode_t,
) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexLodBiasValue(tmu: GrChipID_t, bias: FxFloat) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexDownloadMipMap(
    tmu: GrChipID_t,
    startAddress: FxU32,
    evenOdd: FxU32,
    info: *mut GrTexInfo,
) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexDownloadMipMapLevel(
    tmu: GrChipID_t,
    startAddress: FxU32,
    thisLod: GrLOD_t,
    largeLod: GrLOD_t,
    aspectRatio: GrAspectRatio_t,
    format: GrTextureFormat_t,
    evenOdd: FxU32,
    data: *mut c_void,
) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexDownloadMipMapLevelPartial(
    tmu: GrChipID_t,
    startAddress: FxU32,
    thisLod: GrLOD_t,
    largeLod: GrLOD_t,
    aspectRatio: GrAspectRatio_t,
    format: GrTextureFormat_t,
    evenOdd: FxU32,
    data: *mut c_void,
    start: FxI32,
    end: FxI32,
) -> FxBool {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexDownloadTable(kind: GrTexTable_t, data: *mut c_void) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexDownloadTablePartial(
    kind: GrTexTable_t,
    data: *mut c_void,
    start: FxI32,
    end: FxI32,
) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexMipMapMode(tmu: GrChipID_t, mode: GrMipMapMode_t, lodBlend: FxBool) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexMultibase(tmu: GrChipID_t, enable: FxBool) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grTexMultibaseAddress(
    tmu: GrChipID_t,
    range: GrTexBaseRange_t,
    startAddress: FxU32,
    evenOdd: FxU32,
    info: *mut GrTexInfo,
) {
    todo!();
}

/*
** linear frame buffer functions
*/

#[allow(non_snake_case)]
pub unsafe extern "C" fn grLfbLock(
    kind: GrLock_t,
    buffer: GrBuffer_t,
    writeMode: GrLfbWriteMode_t,
    origin: GrOriginLocation_t,
    pixelPipeline: FxBool,
    info: *mut GrLfbInfo_t,
) -> FxBool {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grLfbUnlock(kind: GrLock_t, buffer: GrBuffer_t) -> FxBool {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grLfbConstantAlpha(alpha: GrAlpha_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grLfbConstantDepth(depth: FxU32) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grLfbWriteColorSwizzle(swizzleBytes: FxBool, swapWords: FxBool) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grLfbWriteColorFormat(colorFormat: GrColorFormat_t) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grLfbWriteRegion(
    dst_buffer: GrBuffer_t,
    dst_x: FxU32,
    dst_y: FxU32,
    src_format: GrLfbSrcFmt_t,
    src_width: FxU32,
    src_height: FxU32,
    pixelPipeline: FxBool,
    src_stride: FxI32,
    src_data: *mut c_void,
) -> FxBool {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grLfbReadRegion(
    src_buffer: GrBuffer_t,
    src_x: FxU32,
    src_y: FxU32,
    src_width: FxU32,
    src_height: FxU32,
    dst_stride: FxU32,
    dst_data: *mut c_void,
) -> FxBool {
    todo!();
}

/*
** glide management functions
*/
#[allow(non_snake_case)]
pub unsafe extern "C" fn grGlideInit() {
    /*grGlideInit initializes the Glide library, performing tasks such as finding any installed graphics
    subsystems, allocating memory, and initializing state variables. grGlideInit must be called before any
    other Glide routines are called (the one exception is noted below).*/
    //todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grGlideShutdown() {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grGlideGetState(state: *mut c_void) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grGlideSetState(state: *const c_void) {
    todo!();
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grGlideGetVertexLayout(layout: *mut c_void) {
    unsafe {
        std::ptr::copy_nonoverlapping(
            &mut (*STATE).vertex_layout,
            layout as *mut VertexBufferLayout,
            std::mem::size_of::<VertexBufferLayout>(),
        )
    }
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn grGlideSetVertexLayout(layout: *const c_void) {
    (*STATE).vertex_layout = *(layout as *const VertexBufferLayout);
}
