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
        (super::PixelFormat::V210, super::PixelFormat::RGB8) => v210_to_rgb8(
            input_buffer,
            output_buffer,
            input_frame_format.width as usize,
            input_frame_format.height as usize,
        ),
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

fn v210_to_rgb8(input_buffer: &[u8], output_buffer: &mut [u8], width: usize, height: usize) {
    let row_stride = input_buffer.len() / height;

    let full_groups = width / 6;
    let remaining = width % 6;

    for row in 0..height {
        let in_row = row * row_stride;
        let out_row = row * width * 3;

        for g in 0..full_groups {
            let off = in_row + g * 16;
            let out_off = out_row + g * 6 * 3;
            let w0 = u32::from_le_bytes(input_buffer[off..off + 4].try_into().unwrap());
            let w1 = u32::from_le_bytes(input_buffer[off + 4..off + 8].try_into().unwrap());
            let w2 = u32::from_le_bytes(input_buffer[off + 8..off + 12].try_into().unwrap());
            let w3 = u32::from_le_bytes(input_buffer[off + 12..off + 16].try_into().unwrap());
            let rgb = unpack_group(w0, w1, w2, w3);
            output_buffer[out_off..out_off + 18].copy_from_slice(&rgb);
        }

        if remaining > 0 {
            let off = in_row + full_groups * 16;
            let out_off = out_row + full_groups * 6 * 3;
            let w0 = u32::from_le_bytes(input_buffer[off..off + 4].try_into().unwrap());
            let w1 = u32::from_le_bytes(input_buffer[off + 4..off + 8].try_into().unwrap());
            let w2 = u32::from_le_bytes(input_buffer[off + 8..off + 12].try_into().unwrap());
            let w3 = u32::from_le_bytes(input_buffer[off + 12..off + 16].try_into().unwrap());
            let rgb = unpack_group(w0, w1, w2, w3);
            output_buffer[out_off..out_off + remaining * 3].copy_from_slice(&rgb[..remaining * 3]);
        }
    }
}

fn unpack_group(w0: u32, w1: u32, w2: u32, w3: u32) -> [u8; 18] {
    // Y from odd-numbered 10-bit positions, reordered from standard:
    //   Y0 = w0[10:19],  Y1 = w1[0:9],   Y2 = w2[10:19]
    //   Y3 = w1[20:29],  Y4 = w3[0:9],   Y5 = w3[20:29]
    // (w1[20:29] and w3[0:9] are swapped vs standard V210)
    let y0 = ((w0 >> 10) & 0x3FF) as i32;
    let y1 = (w1 & 0x3FF) as i32;
    let y2 = ((w2 >> 10) & 0x3FF) as i32;
    let y3 = ((w1 >> 20) & 0x3FF) as i32;
    let y4 = (w3 & 0x3FF) as i32;
    let y5 = ((w3 >> 20) & 0x3FF) as i32;

    // Chroma from even-numbered positions. w0/w1/w2 each contribute:
    //   Pixels (0,1): u0=w0[0:9], v0=w0[20:29]
    //   Pixels (2,3): u1=w1[10:19], v1=w2[0:9]
    //   Pixels (4,5): u2=w2[20:29], v2=w3[10:19]
    let u0 = (w0 & 0x3FF) as i32;
    let u1 = ((w1 >> 10) & 0x3FF) as i32;
    let u2 = ((w2 >> 20) & 0x3FF) as i32;

    let v0 = ((w0 >> 20) & 0x3FF) as i32;
    let v1 = (w2 & 0x3FF) as i32;
    let v2 = ((w3 >> 10) & 0x3FF) as i32;

    let ys = [y0, y1, y2, y3, y4, y5];
    let us = [u0, u0, u1, u1, u2, u2];
    let vs = [v0, v0, v1, v1, v2, v2];

    let mut out = [0u8; 18];
    for i in 0..6 {
        let cy = ((ys[i].max(64).min(940) - 64) * 255 / 876) as i32;
        let cu = ((us[i].max(64).min(960) - 64) * 255 / 896) as i32 - 128;
        let cv = ((vs[i].max(64).min(960) - 64) * 255 / 896) as i32 - 128;

        let r = (cy + ((403 * cv + 128) >> 8)).clamp(0, 255) as u8;
        let g = (cy - ((48 * cu + 120 * cv + 128) >> 8)).clamp(0, 255) as u8;
        let b = (cy + ((475 * cu + 128) >> 8)).clamp(0, 255) as u8;

        out[i * 3] = r;
        out[i * 3 + 1] = g;
        out[i * 3 + 2] = b;
    }
    out
}
