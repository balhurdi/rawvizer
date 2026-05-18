use crate::video::VideoFrameFormat;

pub fn convert_frame(
    input_frame_format: VideoFrameFormat,
    input_buffer: &[u8],
    output_frame_format: VideoFrameFormat,
    output_buffer: &mut [u8],
) {
    match (
        input_frame_format.pixel_format,
        output_frame_format.pixel_format,
    ) {
        (super::PixelFromat::RGB8, super::PixelFromat::RGB8) => {
            identity(input_buffer, output_buffer)
        }
    }
}

fn identity(input_buffer: &[u8], output_buffer: &mut [u8]) {
    // There must be a way around this copy
    output_buffer.copy_from_slice(input_buffer);
}
