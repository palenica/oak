//!  Decoding and Encoding of BMP Images
//!
//!  A decoder and encoder for BMP (Windows Bitmap) images
//!
//!  # Related Links
//!  * <https://msdn.microsoft.com/en-us/library/windows/desktop/dd183375%28v=vs.85%29.aspx>
//!  * <https://en.wikipedia.org/wiki/BMP_file_format>
//!

pub use self::decoder::BmpDecoder;
#[allow(deprecated)] // TODO: when `BMPEncoder` is removed, remove this flag
pub use self::encoder::{BmpEncoder, BMPEncoder};

mod decoder;
mod encoder;
