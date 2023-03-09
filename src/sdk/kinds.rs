use std::ffi::c_void;

use winit::dpi::PhysicalSize;

pub type FxU8 = u8;
pub type FxI8 = i8;
pub type FxU16 = u16;
pub type FxI16 = i16;
pub type FxU32 = u32;
pub type FxI32 = i32;
pub type FxFloat = f32;
pub type FxBool = i32;

#[allow(non_camel_case_types)]
pub type GrColor_t = FxU32;
#[allow(non_camel_case_types)]
pub type GrAlpha_t = FxU8;
#[allow(non_camel_case_types)]
pub type GrMipMapId_t = FxU32;
#[allow(non_camel_case_types)]
pub type GrStipplePattern_t = FxU32;
#[allow(non_camel_case_types)]
pub type GrFog_t = FxU8;
#[allow(non_camel_case_types)]
pub type GrContext_t = FxU32;
pub type GrProc = *mut dyn Fn() -> FxI32;
#[allow(non_camel_case_types)]
pub type GrErrorCallbackFnc_t = *mut dyn Fn(*const char, FxBool);

#[allow(non_camel_case_types)]
pub struct GrLfbInfo_t {
    size: FxI32,
    lfbPtr: *mut c_void,
    strideInBytes: FxU32,
    writeMode: GrLfbWriteMode_t,
    origin: GrOriginLocation_t,
}

#[allow(non_camel_case_types)]
pub struct GrTexInfo {
    smallLodLog2: GrLOD_t,
    largeLodLog2: GrLOD_t,
    aspectRatioLog2: GrAspectRatio_t,
    format: GrTextureFormat_t,
    data: *mut c_void,
}
#[allow(non_camel_case_types)]
pub struct GrSstPerfStats_s {
    pixelsIn: FxU32,   /* # pixels processed (minus buffer clears) */
    chromaFail: FxU32, /* # pixels not drawn due to chroma key */
    zFuncFail: FxU32,  /* # pixels not drawn due to Z comparison */
    aFuncFail: FxU32,  /* # pixels not drawn due to alpha comparison */
    pixelsOut: FxU32,  /* # pixels drawn (including buffer clears) */
}

pub struct GrResolution {
    resolution: GrScreenResolution_t,
    refresh: GrScreenRefresh_t,
    numColorBuffers: FxI32,
    numAuxBuffers: FxI32,
}

type GlideResolution = GrResolution;

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrScreenRefresh_t {
    GR_REFRESH_60Hz = 0x0,
    GR_REFRESH_70Hz = 0x1,
    GR_REFRESH_72Hz = 0x2,
    GR_REFRESH_75Hz = 0x3,
    GR_REFRESH_80Hz = 0x4,
    GR_REFRESH_90Hz = 0x5,
    GR_REFRESH_100Hz = 0x6,
    GR_REFRESH_85Hz = 0x7,
    GR_REFRESH_120Hz = 0x8,
    GR_REFRESH_NONE = 0xff,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrScreenResolution_t {
    GR_RESOLUTION_320x200 = 0x0,
    GR_RESOLUTION_320x240 = 0x1,
    GR_RESOLUTION_400x256 = 0x2,
    GR_RESOLUTION_512x384 = 0x3,
    GR_RESOLUTION_640x200 = 0x4,
    GR_RESOLUTION_640x350 = 0x5,
    GR_RESOLUTION_640x400 = 0x6,
    GR_RESOLUTION_640x480 = 0x7,
    GR_RESOLUTION_800x600 = 0x8,
    GR_RESOLUTION_960x720 = 0x9,
    GR_RESOLUTION_856x480 = 0xa,
    GR_RESOLUTION_512x256 = 0xb,
    GR_RESOLUTION_1024x768 = 0xC,
    GR_RESOLUTION_1280x1024 = 0xD,
    GR_RESOLUTION_1600x1200 = 0xE,
    GR_RESOLUTION_400x300 = 0xF,
    GR_RESOLUTION_NONE = 0xff,
}

impl From<GrScreenResolution_t> for PhysicalSize<u32> {
    fn from(res: GrScreenResolution_t) -> Self {
        match res {
            GrScreenResolution_t::GR_RESOLUTION_320x200 => PhysicalSize::from([320, 200]),
            GrScreenResolution_t::GR_RESOLUTION_320x240 => PhysicalSize::from([320, 240]),
            GrScreenResolution_t::GR_RESOLUTION_400x256 => PhysicalSize::from([400, 256]),
            GrScreenResolution_t::GR_RESOLUTION_512x384 => PhysicalSize::from([512, 384]),
            GrScreenResolution_t::GR_RESOLUTION_640x200 => PhysicalSize::from([640, 200]),
            GrScreenResolution_t::GR_RESOLUTION_640x350 => PhysicalSize::from([640, 350]),
            GrScreenResolution_t::GR_RESOLUTION_640x400 => PhysicalSize::from([640, 400]),
            GrScreenResolution_t::GR_RESOLUTION_640x480 => PhysicalSize::from([640, 480]),
            GrScreenResolution_t::GR_RESOLUTION_800x600 => PhysicalSize::from([800, 600]),
            GrScreenResolution_t::GR_RESOLUTION_960x720 => PhysicalSize::from([960, 720]),
            GrScreenResolution_t::GR_RESOLUTION_856x480 => PhysicalSize::from([856, 480]),
            GrScreenResolution_t::GR_RESOLUTION_512x256 => PhysicalSize::from([512, 256]),
            GrScreenResolution_t::GR_RESOLUTION_1024x768 => PhysicalSize::from([1024, 768]),
            GrScreenResolution_t::GR_RESOLUTION_1280x1024 => PhysicalSize::from([1280, 1024]),
            GrScreenResolution_t::GR_RESOLUTION_1600x1200 => PhysicalSize::from([1600, 1200]),
            GrScreenResolution_t::GR_RESOLUTION_400x300 => PhysicalSize::from([400, 300]),
            GrScreenResolution_t::GR_RESOLUTION_NONE => PhysicalSize::from([1280, 1024]),
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrChipID_t {
    //GR_FBI = 0x0,
    GR_TMU0 = 0x0,
    GR_TMU1 = 0x1,
    GR_TMU2 = 0x2,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrCombineFunction_t {
    GR_COMBINE_FUNCTION_ZERO = 0x0,
    //GR_COMBINE_FUNCTION_NONE = GR_COMBINE_FUNCTION_ZERO,
    R_COMBINE_FUNCTION_LOCAL = 0x1,
    GR_COMBINE_FUNCTION_LOCAL_ALPHA = 0x2,
    GR_COMBINE_FUNCTION_SCALE_OTHER = 0x3,
    //GR_COMBINE_FUNCTION_BLEND_OTHER = GR_COMBINE_FUNCTION_SCALE_OTHER,
    GR_COMBINE_FUNCTION_SCALE_OTHER_ADD_LOCAL = 0x4,
    GR_COMBINE_FUNCTION_SCALE_OTHER_ADD_LOCAL_ALPHA = 0x5,
    GR_COMBINE_FUNCTION_SCALE_OTHER_MINUS_LOCAL = 0x6,
    GR_COMBINE_FUNCTION_SCALE_OTHER_MINUS_LOCAL_ADD_LOCAL = 0x7,
    //GR_COMBINE_FUNCTION_BLEND = GR_COMBINE_FUNCTION_SCALE_OTHER_MINUS_LOCAL_ADD_LOCAL,
    GR_COMBINE_FUNCTION_SCALE_OTHER_MINUS_LOCAL_ADD_LOCAL_ALPHA = 0x8,
    GR_COMBINE_FUNCTION_SCALE_MINUS_LOCAL_ADD_LOCAL = 0x9,
    //GR_COMBINE_FUNCTION_BLEND_LOCAL = GR_COMBINE_FUNCTION_SCALE_MINUS_LOCAL_ADD_LOCAL,
    GR_COMBINE_FUNCTION_SCALE_MINUS_LOCAL_ADD_LOCAL_ALPHA = 0x10,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrCombineFactor_t {
    GR_COMBINE_FACTOR_ZERO = 0x0,
    //GR_COMBINE_FACTOR_NONE = GR_COMBINE_FACTOR_ZERO,
    GR_COMBINE_FACTOR_LOCAL = 0x1,
    GR_COMBINE_FACTOR_OTHER_ALPHA = 0x2,
    GR_COMBINE_FACTOR_LOCAL_ALPHA = 0x3,
    GR_COMBINE_FACTOR_TEXTURE_ALPHA = 0x4,
    GR_COMBINE_FACTOR_TEXTURE_RGB = 0x5,
    //GR_COMBINE_FACTOR_DETAIL_FACTOR = GR_COMBINE_FACTOR_TEXTURE_ALPHA,
    //GR_COMBINE_FACTOR_LOD_FRACTION = 0x5,
    GR_COMBINE_FACTOR_ONE = 0x8,
    GR_COMBINE_FACTOR_ONE_MINUS_LOCAL = 0x9,
    GR_COMBINE_FACTOR_ONE_MINUS_OTHER_ALPHA = 0xa,
    GR_COMBINE_FACTOR_ONE_MINUS_LOCAL_ALPHA = 0xb,
    GR_COMBINE_FACTOR_ONE_MINUS_TEXTURE_ALPHA = 0xc,
    //GR_COMBINE_FACTOR_ONE_MINUS_DETAIL_FACTOR = GR_COMBINE_FACTOR_ONE_MINUS_TEXTURE_ALPHA,
    GR_COMBINE_FACTOR_ONE_MINUS_LOD_FRACTION = 0xd,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrCombineLocal_t {
    GR_COMBINE_LOCAL_ITERATED = 0x0,
    GR_COMBINE_LOCAL_CONSTANT = 0x1,
    //GR_COMBINE_LOCAL_NONE = GR_COMBINE_LOCAL_CONSTANT,
    GR_COMBINE_LOCAL_DEPTH = 0x2,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrCombineOther_t {
    GR_COMBINE_OTHER_ITERATED = 0x0,
    GR_COMBINE_OTHER_TEXTURE = 0x1,
    GR_COMBINE_OTHER_CONSTANT = 0x2,
    //GR_COMBINE_OTHER_NONE = GR_COMBINE_OTHER_CONSTANT,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrAlphaSource_t {
    GR_ALPHASOURCE_CC_ALPHA = 0x0,
    GR_ALPHASOURCE_ITERATED_ALPHA = 0x1,
    GR_ALPHASOURCE_TEXTURE_ALPHA = 0x2,
    GR_ALPHASOURCE_TEXTURE_ALPHA_TIMES_ITERATED_ALPHA = 0x3,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrColorCombineFnc_t {
    GR_COLORCOMBINE_ZERO = 0x0,
    GR_COLORCOMBINE_CCRGB = 0x1,
    GR_COLORCOMBINE_ITRGB = 0x2,
    GR_COLORCOMBINE_ITRGB_DELTA0 = 0x3,
    GR_COLORCOMBINE_DECAL_TEXTURE = 0x4,
    GR_COLORCOMBINE_TEXTURE_TIMES_CCRGB = 0x5,
    GR_COLORCOMBINE_TEXTURE_TIMES_ITRGB = 0x6,
    GR_COLORCOMBINE_TEXTURE_TIMES_ITRGB_DELTA0 = 0x7,
    GR_COLORCOMBINE_TEXTURE_TIMES_ITRGB_ADD_ALPHA = 0x8,
    GR_COLORCOMBINE_TEXTURE_TIMES_ALPHA = 0x9,
    GR_COLORCOMBINE_TEXTURE_TIMES_ALPHA_ADD_ITRGB = 0xa,
    GR_COLORCOMBINE_TEXTURE_ADD_ITRGB = 0xb,
    GR_COLORCOMBINE_TEXTURE_SUB_ITRGB = 0xc,
    GR_COLORCOMBINE_CCRGB_BLEND_ITRGB_ON_TEXALPHA = 0xd,
    GR_COLORCOMBINE_DIFF_SPEC_A = 0xe,
    GR_COLORCOMBINE_DIFF_SPEC_B = 0xf,
    GR_COLORCOMBINE_ONE = 0x10,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrAlphaBlendFnc_t {
    GR_BLEND_ZERO = 0x0,
    GR_BLEND_SRC_ALPHA = 0x1,
    GR_BLEND_SRC_COLOR = 0x2,
    //GR_BLEND_DST_COLOR = GR_BLEND_SRC_COLOR,
    GR_BLEND_DST_ALPHA = 0x3,
    GR_BLEND_ONE = 0x4,
    GR_BLEND_ONE_MINUS_SRC_ALPHA = 0x5,
    GR_BLEND_ONE_MINUS_SRC_COLOR = 0x6,
    //GR_BLEND_ONE_MINUS_DST_COLOR = GR_BLEND_ONE_MINUS_SRC_COLOR,
    GR_BLEND_ONE_MINUS_DST_ALPHA = 0x7,
    GR_BLEND_RESERVED_8 = 0x8,
    GR_BLEND_RESERVED_9 = 0x9,
    GR_BLEND_RESERVED_A = 0xa,
    GR_BLEND_RESERVED_B = 0xb,
    GR_BLEND_RESERVED_C = 0xc,
    GR_BLEND_RESERVED_D = 0xd,
    GR_BLEND_RESERVED_E = 0xe,
    GR_BLEND_ALPHA_SATURATE = 0xf,
    //GR_BLEND_PREFOG_COLOR = GR_BLEND_ALPHA_SATURATE,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrAspectRatio_t {
    GR_ASPECT_LOG2_8x1 = 3,  // 8W x 1H
    GR_ASPECT_LOG2_4x1 = 2,  // 4W x 1H
    GR_ASPECT_LOG2_2x1 = 1,  // 2W x 1H
    GR_ASPECT_LOG2_1x1 = 0,  // 1W x 1H
    GR_ASPECT_LOG2_1x2 = -1, // 1W x 2H
    GR_ASPECT_LOG2_1x4 = -2, // 1W x 4H
    GR_ASPECT_LOG2_1x8 = -3, // 1W x 8H
}
#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrBuffer_t {
    GR_BUFFER_FRONTBUFFER = 0x0,
    GR_BUFFER_BACKBUFFER = 0x1,
    GR_BUFFER_AUXBUFFER = 0x2,
    GR_BUFFER_DEPTHBUFFER = 0x3,
    GR_BUFFER_ALPHABUFFER = 0x4,
    GR_BUFFER_TRIPLEBUFFER = 0x5,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrChromakeyMode_t {
    GR_CHROMAKEY_DISABLE = 0x0,
    GR_CHROMAKEY_ENABLE = 0x1,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrChromaRangeMode_t {
    //GR_CHROMARANGE_RGB_ALL_EXT = 0x0,
    GR_CHROMARANGE_DISABLE_EXT = 0x00,
    GR_CHROMARANGE_ENABLE_EXT = 0x01,
}
#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrTexChromakeyMode_t {
    GR_TEXCHROMA_DISABLE_EXT = 0x0,
    GR_TEXCHROMA_ENABLE_EXT = 0x1,
    //GR_TEXCHROMARANGE_RGB_ALL_EXT = 0x0,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrCmpFnc_t {
    GR_CMP_NEVER = 0x0,
    GR_CMP_LESS = 0x1,
    GR_CMP_EQUAL = 0x2,
    GR_CMP_LEQUAL = 0x3,
    GR_CMP_GREATER = 0x4,
    GR_CMP_NOTEQUAL = 0x5,
    GR_CMP_GEQUAL = 0x6,
    GR_CMP_ALWAYS = 0x7,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrColorFormat_t {
    GR_COLORFORMAT_ARGB = 0x0,
    GR_COLORFORMAT_ABGR = 0x1,

    GR_COLORFORMAT_RGBA = 0x2,
    GR_COLORFORMAT_BGRA = 0x3,
}

impl From<GrColorFormat_t> for wgpu::TextureFormat {
    fn from(format: GrColorFormat_t) -> Self {
        match format {
            GrColorFormat_t::GR_COLORFORMAT_ABGR => panic!(),
            GrColorFormat_t::GR_COLORFORMAT_ARGB => panic!(),
            GrColorFormat_t::GR_COLORFORMAT_BGRA => wgpu::TextureFormat::Bgra8Unorm,
            GrColorFormat_t::GR_COLORFORMAT_RGBA => wgpu::TextureFormat::Rgba8Unorm,
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrCullMode_t {
    GR_CULL_DISABLE = 0x0,
    GR_CULL_NEGATIVE = 0x1,
    GR_CULL_POSITIVE = 0x2,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrDepthBufferMode_t {
    GR_DEPTHBUFFER_DISABLE = 0x0,
    GR_DEPTHBUFFER_ZBUFFER = 0x1,
    GR_DEPTHBUFFER_WBUFFER = 0x2,
    GR_DEPTHBUFFER_ZBUFFER_COMPARE_TO_BIAS = 0x3,
    GR_DEPTHBUFFER_WBUFFER_COMPARE_TO_BIAS = 0x4,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrDitherMode_t {
    GR_DITHER_DISABLE = 0x0,
    GR_DITHER_2x2 = 0x1,
    GR_DITHER_4x4 = 0x2,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrStippleMode_t {
    GR_STIPPLE_DISABLE = 0x0,
    GR_STIPPLE_PATTERN = 0x1,
    GR_STIPPLE_ROTATE = 0x2,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrFogMode_t {
    GR_FOG_DISABLE = 0x0,
    GR_FOG_WITH_TABLE_ON_FOGCOORD_EXT = 0x1,
    GR_FOG_WITH_TABLE_ON_Q = 0x2,
    //GR_FOG_WITH_TABLE_ON_W            = GR_FOG_WITH_TABLE_ON_Q,
    GR_FOG_WITH_ITERATED_Z = 0x3,
    GR_FOG_WITH_ITERATED_ALPHA_EXT = 0x4,
    GR_FOG_MULT2 = 0x100,
    GR_FOG_ADD2 = 0x200,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum GrLock_t {
    GR_LFB_READ_ONLY = 0x00,
    GR_LFB_WRITE_ONLY = 0x01,
    //GR_LFB_IDLE      = 0x00,
    GR_LFB_NOIDLE = 0x10,

    GR_LFB_WRITE_ONLY_EXPLICIT_EXT = 0x02, /* explicitly not allow reading from the lfb pointer */
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrLfbBypassMode_t {
    GR_LFBBYPASS_DISABLE = 0x0,
    GR_LFBBYPASS_ENABLE = 0x1,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrLfbWriteMode_t {
    GR_LFBWRITEMODE_565 = 0x0,  /* RGB:RGB */
    GR_LFBWRITEMODE_555 = 0x1,  /* RGB:RGB */
    GR_LFBWRITEMODE_1555 = 0x2, /* ARGB:ARGB */
    GR_LFBWRITEMODE_RESERVED1 = 0x3,
    GR_LFBWRITEMODE_888 = 0x4,  /* RGB */
    GR_LFBWRITEMODE_8888 = 0x5, /* ARGB */
    GR_LFBWRITEMODE_RESERVED2 = 0x6,
    GR_LFBWRITEMODE_RESERVED3 = 0x7,
    GR_LFBWRITEMODE_RESERVED4 = 0x8,
    GR_LFBWRITEMODE_RESERVED5 = 0x9,
    GR_LFBWRITEMODE_RESERVED6 = 0xa,
    GR_LFBWRITEMODE_RESERVED7 = 0xb,
    GR_LFBWRITEMODE_565_DEPTH = 0xc,  /* RGB:DEPTH */
    GR_LFBWRITEMODE_555_DEPTH = 0xd,  /* RGB:DEPTH */
    GR_LFBWRITEMODE_1555_DEPTH = 0xe, /* ARGB:DEPTH */
    GR_LFBWRITEMODE_ZA16 = 0xf,       /* DEPTH:DEPTH */
    GR_LFBWRITEMODE_ANY = 0xFF,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrOriginLocation_t {
    GR_ORIGIN_UPPER_LEFT = 0x0,
    GR_ORIGIN_LOWER_LEFT = 0x1,
    GR_ORIGIN_ANY = 0xFF,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrLOD_t {
    GR_LOD_LOG2_256 = 0x8,
    GR_LOD_LOG2_128 = 0x7,
    GR_LOD_LOG2_64 = 0x6,
    GR_LOD_LOG2_32 = 0x5,
    GR_LOD_LOG2_16 = 0x4,
    GR_LOD_LOG2_8 = 0x3,
    GR_LOD_LOG2_4 = 0x2,
    GR_LOD_LOG2_2 = 0x1,
    GR_LOD_LOG2_1 = 0x0,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrMipMapMode_t {
    GR_MIPMAP_DISABLE = 0x0,        /* no mip mapping  */
    GR_MIPMAP_NEAREST = 0x1,        /* use nearest mipmap */
    GR_MIPMAP_NEAREST_DITHER = 0x2, /* GR_MIPMAP_NEAREST + LOD dith */
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrSmoothingMode_t {
    GR_SMOOTHING_DISABLE = 0x0,
    GR_SMOOTHING_ENABLE = 0x1,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrTextureClampMode_t {
    GR_TEXTURECLAMP_WRAP = 0x0,
    GR_TEXTURECLAMP_CLAMP = 0x1,
    GR_TEXTURECLAMP_MIRROR_EXT = 0x2,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrTextureCombineFnc_t {
    GR_TEXTURECOMBINE_ZERO = 0x0,           /* texout = 0 */
    GR_TEXTURECOMBINE_DECAL = 0x1,          /* texout = texthis */
    GR_TEXTURECOMBINE_OTHER = 0x2,          /* this TMU in passthru mode */
    GR_TEXTURECOMBINE_ADD = 0x3,            /* tout = tthis + t(this+1) */
    GR_TEXTURECOMBINE_MULTIPLY = 0x4,       /* texout = tthis * t(this+1) */
    GR_TEXTURECOMBINE_SUBTRACT = 0x5,       /* Sutract from upstream TMU */
    GR_TEXTURECOMBINE_DETAIL = 0x6,         /* detail--detail on tthis */
    GR_TEXTURECOMBINE_DETAIL_OTHER = 0x7,   /* detail--detail on tthis+1 */
    GR_TEXTURECOMBINE_TRILINEAR_ODD = 0x8,  /* trilinear--odd levels tthis*/
    GR_TEXTURECOMBINE_TRILINEAR_EVEN = 0x9, /*trilinear--even levels tthis*/
    GR_TEXTURECOMBINE_ONE = 0xa,            /* texout = 0xFFFFFFFF */
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrTextureFilterMode_t {
    GR_TEXTUREFILTER_POINT_SAMPLED = 0x0,
    GR_TEXTUREFILTER_BILINEAR = 0x1,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum GrTextureFormat_t {
    /* KoolSmoky - */
    GR_TEXFMT_8BIT = 0x0,
    //GR_TEXFMT_RGB_332         =      GR_TEXFMT_8BIT,
    GR_TEXFMT_YIQ_422 = 0x1,
    GR_TEXFMT_ALPHA_8 = 0x2,     /* (0..0xFF) alpha     */
    GR_TEXFMT_INTENSITY_8 = 0x3, /* (0..0xFF) intensity */
    GR_TEXFMT_ALPHA_INTENSITY_44 = 0x4,
    GR_TEXFMT_P_8 = 0x5,   /* 8-bit palette */
    GR_TEXFMT_RSVD0 = 0x6, /* GR_TEXFMT_P_8_RGBA */
    //GR_TEXFMT_P_8_6666            =  GR_TEXFMT_RSVD0,
    //GR_TEXFMT_P_8_6666_EXT        =  GR_TEXFMT_RSVD0,
    GR_TEXFMT_RSVD1 = 0x7,
    GR_TEXFMT_16BIT = 0x8,
    //GR_TEXFMT_ARGB_8332           =  GR_TEXFMT_16BIT,
    GR_TEXFMT_AYIQ_8422 = 0x9,
    GR_TEXFMT_RGB_565 = 0xa,
    GR_TEXFMT_ARGB_1555 = 0xb,
    GR_TEXFMT_ARGB_4444 = 0xc,
    GR_TEXFMT_ALPHA_INTENSITY_88 = 0xd,
    GR_TEXFMT_AP_88 = 0xe, /* 8-bit alpha 8-bit palette */
    GR_TEXFMT_RSVD2 = 0xf,
    //GR_TEXFMT_RSVD4                = GR_TEXFMT_RSVD2,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum GrTexTable_t {
    GR_TEXTABLE_NCC0 = 0x0,
    GR_TEXTABLE_NCC1 = 0x1,
    GR_TEXTABLE_PALETTE = 0x2,
    GR_TEXTABLE_PALETTE_6666_EXT = 0x3,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum GrNCCTable_t {
    GR_NCCTABLE_NCC0 = 0x0,
    GR_NCCTABLE_NCC1 = 0x1,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum GrTexBaseRange_t {
    GR_TEXBASE_256 = 0x3,
    GR_TEXBASE_128 = 0x2,
    GR_TEXBASE_64 = 0x1,
    GR_TEXBASE_32_TO_1 = 0x0,
}

#[allow(non_camel_case_types)]
pub type GrEnableMode_t = FxU32;

pub const GR_MODE_DISABLE: FxU32 = 0x0;
pub const GR_MODE_ENABLE: FxU32 = 0x1;
pub const GR_AA_ORDERED: FxU32 = 0x01;
pub const GR_ALLOW_MIPMAP_DITHER: FxU32 = 0x02;
pub const GR_PASSTHRU: FxU32 = 0x03;
pub const GR_SHAMELESS_PLUG: FxU32 = 0x04;
pub const GR_VIDEO_SMOOTHING: FxU32 = 0x05;

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum GrCoordinateSpaceMode_t {
    GR_WINDOW_COORDS = 0x00,
    GR_CLIP_COORDS = 0x01,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum GrLfbSrcFmt_t {
    GR_LFB_SRC_FMT_565 = 0x00,
    GR_LFB_SRC_FMT_555 = 0x01,
    GR_LFB_SRC_FMT_1555 = 0x02,
    GR_LFB_SRC_FMT_888 = 0x04,
    GR_LFB_SRC_FMT_8888 = 0x05,
    GR_LFB_SRC_FMT_565_DEPTH = 0x0c,
    GR_LFB_SRC_FMT_555_DEPTH = 0x0d,
    GR_LFB_SRC_FMT_1555_DEPTH = 0x0e,
    GR_LFB_SRC_FMT_ZA16 = 0x0f,
    GR_LFB_SRC_FMT_RLE16 = 0x80,
}
