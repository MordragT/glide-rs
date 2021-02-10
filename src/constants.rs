use crate::kinds::FxI32;
use num_enum::TryFromPrimitive;

/* Types of data in strips */
pub const GR_FLOAT: FxI32 = 0;
pub const GR_U8: FxI32 = 1;

/* Parameters for strips */
// pub const GR_PARAM_XY: FxI32 = 0x01;
// pub const GR_PARAM_Z: FxI32 = 0x02;
// pub const GR_PARAM_W: FxI32 = 0x03;
// pub const GR_PARAM_Q: FxI32 = 0x04;
// pub const GR_PARAM_FOG_EXT: FxI32 = 0x05;

// pub const GR_PARAM_A: FxI32 = 0x10;

// pub const GR_PARAM_RGB: FxI32 = 0x20;

// pub const GR_PARAM_PARGB: FxI32 = 0x30;

// pub const GR_PARAM_ST0: FxI32 = 0x40;
// pub const GR_PARAM_ST1: FxI32 = GR_PARAM_ST0 + 1;
// pub const GR_PARAM_ST2: FxI32 = GR_PARAM_ST0 + 2;

// pub const GR_PARAM_Q0: FxI32 = 0x50;
// pub const GR_PARAM_Q1: FxI32 = GR_PARAM_Q0 + 1;
// pub const GR_PARAM_Q2: FxI32 = GR_PARAM_Q0 + 2;

pub const GR_PARAM_DISABLE: FxI32 = 0x00;
pub const GR_PARAM_ENABLE: FxI32 = 0x01;

/*
** grDrawVertexArray/grDrawVertexArrayContiguous primitive type
*/
pub const GR_POINTS: FxI32 = 0;
pub const GR_LINE_STRIP: FxI32 = 1;
pub const GR_LINES: FxI32 = 2;
pub const GR_POLYGON: FxI32 = 3;
pub const GR_TRIANGLE_STRIP: FxI32 = 4;
pub const GR_TRIANGLE_FAN: FxI32 = 5;
pub const GR_TRIANGLES: FxI32 = 6;
pub const GR_TRIANGLE_STRIP_CONTINUE: FxI32 = 7;
pub const GR_TRIANGLE_FAN_CONTINUE: FxI32 = 8;

/*
** grGet/grReset types
*/
pub const GR_BITS_DEPTH: FxI32 = 0x01;
pub const GR_BITS_RGBA: FxI32 = 0x02;
pub const GR_FIFO_FULLNESS: FxI32 = 0x03;
pub const GR_FOG_TABLE_ENTRIES: FxI32 = 0x04;
pub const GR_GAMMA_TABLE_ENTRIES: FxI32 = 0x05;
pub const GR_GLIDE_STATE_SIZE: FxI32 = 0x06;
pub const GR_GLIDE_VERTEXLAYOUT_SIZE: FxI32 = 0x07;
pub const GR_IS_BUSY: FxI32 = 0x08;
pub const GR_LFB_PIXEL_PIPE: FxI32 = 0x09;
pub const GR_MAX_TEXTURE_SIZE: FxI32 = 0x0a;
pub const GR_MAX_TEXTURE_ASPECT_RATIO: FxI32 = 0x0b;
pub const GR_MEMORY_FB: FxI32 = 0x0c;
pub const GR_MEMORY_TMU: FxI32 = 0x0d;
pub const GR_MEMORY_UMA: FxI32 = 0x0e;
pub const GR_NUM_BOARDS: FxI32 = 0x0f;
pub const GR_NON_POWER_OF_TWO_TEXTURES: FxI32 = 0x10;
pub const GR_NUM_FB: FxI32 = 0x11;
pub const GR_NUM_SWAP_HISTORY_BUFFER: FxI32 = 0x12;
pub const GR_NUM_TMU: FxI32 = 0x13;
pub const GR_PENDING_BUFFERSWAPS: FxI32 = 0x14;
pub const GR_REVISION_FB: FxI32 = 0x15;
pub const GR_REVISION_TMU: FxI32 = 0x16;
pub const GR_STATS_LINES: FxI32 = 0x17; /* grGet/grReset */
pub const GR_STATS_PIXELS_AFUNC_FAIL: FxI32 = 0x18;
pub const GR_STATS_PIXELS_CHROMA_FAIL: FxI32 = 0x19;
pub const GR_STATS_PIXELS_DEPTHFUNC_FAIL: FxI32 = 0x1a;
pub const GR_STATS_PIXELS_IN: FxI32 = 0x1b;
pub const GR_STATS_PIXELS_OUT: FxI32 = 0x1c;
pub const GR_STATS_PIXELS: FxI32 = 0x1d; /* grReset */
pub const GR_STATS_POINTS: FxI32 = 0x1e; /* grGet/grReset */
pub const GR_STATS_TRIANGLES_IN: FxI32 = 0x1f;
pub const GR_STATS_TRIANGLES_OUT: FxI32 = 0x20;
pub const GR_STATS_TRIANGLES: FxI32 = 0x21; /* grReset */
pub const GR_SWAP_HISTORY: FxI32 = 0x22;
pub const GR_SUPPORTS_PASSTHRU: FxI32 = 0x23;
pub const GR_TEXTURE_ALIGN: FxI32 = 0x24;
pub const GR_VIDEO_POSITION: FxI32 = 0x25;
pub const GR_VIEWPORT: FxI32 = 0x26;
pub const GR_WDEPTH_MIN_MAX: FxI32 = 0x27;
pub const GR_ZDEPTH_MIN_MAX: FxI32 = 0x28;
pub const GR_VERTEX_PARAMETER: FxI32 = 0x29;
pub const GR_BITS_GAMMA: FxI32 = 0x2a;
pub const GR_GET_RESERVED_1: FxI32 = 0x1000;

/*
** grGetString types
*/
pub const GR_EXTENSION: FxI32 = 0xa0;
pub const GR_HARDWARE: FxI32 = 0xa1;
pub const GR_RENDERER: FxI32 = 0xa2;
pub const GR_VENDOR: FxI32 = 0xa3;
pub const GR_VERSION: FxI32 = 0xa4;
