use wgpu::TextureFormat;

// My understanding is that this format in part tells us what is layout of bytes in actual texture data
// and in part how shader interprets it.
//
// Rgba8UnormSrgb means that
// - in texture data there are 4 channels (rgba) each has 8 bits which means that each pixel has 32 bits
// - color data is normalized in shader to range (0, 1) so that if you access (for instance sample) texture
//      in shader then you pick values from this range instead of exact color values
// - Srgb refers to sRGB color space.
//      This setting results is some transformation (gamma correction) done by graphic card on the shader.
//      It has to do with the fact that our eyes can distinguish colors
//      better when they are dark so I guess there is bigger color range
//      assigned to dark color and bright colors got compressed together because it does not matter.
//      Note that Srgb formats are not supported by texture storage array.
pub const TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba8UnormSrgb;