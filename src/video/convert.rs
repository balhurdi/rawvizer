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
        (super::PixelFormat::V210, super::PixelFormat::RGB8) => todo!(),
        (super::PixelFormat::RGB8, super::PixelFormat::V210) => {
            unimplemented!("There are no plans to support this conversion in the future")
        }
        _ => identity(input_buffer, output_buffer),
    }
}

fn identity(input_buffer: &[u8], output_buffer: &mut [u8]) {
    // There must be a way around this copy
    output_buffer.copy_from_slice(input_buffer);
}

fn v210_to_rgb8(input_buffer: &[u8], output_buffer: &mut [u8]) {}
