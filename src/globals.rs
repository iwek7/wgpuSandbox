use wgpu::TextureFormat;

// I do not have too much clue about this setting,
// I believe it was explained in wgpu tutorial.
// It stores info how data is laid out in a array storage buffer.
//
// Note that Srgb formats are not supported by texture storage array.
pub const TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;