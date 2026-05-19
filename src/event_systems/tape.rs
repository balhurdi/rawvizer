use image::{DynamicImage, RgbImage};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{
    error::{Error, Result},
    file_loader::FileLoader,
    video::{ColorConverter, PixelFormat, VideoFrameFormat},
};

const THUMBNAIL_WIDTH: u32 = 1280;
const THUMBNAIL_HEIGHT: u32 = 720;
const INTERNAL_PIXEL_FORMAT: PixelFormat = PixelFormat::RGB8;

#[derive(Debug, Clone)]
pub enum TapeEvent {
    NextFrame,
    PreviousFrame,
}

pub enum FrameReceiverEvent {
    Frame(DynamicImage),
    Error(Error),
}

#[derive(Debug)]
pub struct Tape {
    file_loader: FileLoader,
    frame_format: VideoFrameFormat,
}

impl Tape {
    pub fn new(file_loader: FileLoader, frame_format: VideoFrameFormat) -> Self {
        Self {
            file_loader,
            frame_format,
        }
    }

    pub fn start(mut self) -> (TapeController, TapeFrameReceiver) {
        let (controller_tx, mut controller_rx) = mpsc::unbounded_channel::<TapeEvent>();
        let (frame_receiver_tx, frame_receiver_rx) =
            mpsc::unbounded_channel::<FrameReceiverEvent>();

        tokio::spawn(async move {
            let output_format = VideoFrameFormat {
                width: self.frame_format.width,
                height: self.frame_format.height,
                pixel_format: INTERNAL_PIXEL_FORMAT,
            };

            let mut color_converter = ColorConverter::new(self.frame_format, output_format);

            loop {
                let tape_event = controller_rx.recv().await;

                if let Some(te) = tape_event {
                    let frame = match te {
                        TapeEvent::NextFrame => self.file_loader.next(),
                        TapeEvent::PreviousFrame => todo!(),
                    };

                    if let Some(Ok(f)) = frame {
                        match self.create_dynamic_image(&mut color_converter, f.data()) {
                            Ok(img) => {
                                let _ = frame_receiver_tx.send(FrameReceiverEvent::Frame(img));
                            }
                            Err(err) => {
                                let _ = frame_receiver_tx.send(FrameReceiverEvent::Error(err));
                            }
                        }
                    }
                }
            }
        });

        (
            TapeController {
                inner: controller_tx,
            },
            TapeFrameReceiver {
                inner: frame_receiver_rx,
            },
        )
    }

    #[tracing::instrument]
    fn create_dynamic_image(
        &mut self,
        color_converter: &mut ColorConverter,
        input_buffer: &[u8],
    ) -> Result<DynamicImage> {
        let w = self.frame_format.width as u32;
        let h = self.frame_format.height as u32;

        if self.frame_format.pixel_format == INTERNAL_PIXEL_FORMAT {
            let thumbnail = thumbnail_rgb8(input_buffer, w, h, THUMBNAIL_WIDTH, THUMBNAIL_HEIGHT)
                .ok_or(Error::NoDynamicImage)?;
            return Ok(thumbnail);
        }

        let output_buffer = color_converter.convert_frame(input_buffer);
        let thumbnail = thumbnail_rgb8(&output_buffer, w, h, THUMBNAIL_WIDTH, THUMBNAIL_HEIGHT)
            .ok_or(Error::NoDynamicImage)?;
        return Ok(thumbnail);
    }
}

pub struct TapeController {
    inner: UnboundedSender<TapeEvent>,
}

impl TapeController {
    pub fn send_event(&self, event: TapeEvent) -> Result<()> {
        self.inner.send(event)?;
        Ok(())
    }
}

pub struct TapeFrameReceiver {
    inner: UnboundedReceiver<FrameReceiverEvent>,
}

impl TapeFrameReceiver {
    pub async fn receive_frame(&mut self) -> Option<FrameReceiverEvent> {
        self.inner.recv().await
    }
}

fn thumbnail_rgb8(
    src: &[u8],
    src_width: u32,
    src_height: u32,
    dst_width: u32,
    dst_height: u32,
) -> Option<DynamicImage> {
    let dst_size = (dst_width * dst_height * 3) as usize;
    let mut dst = vec![0u8; dst_size];

    for y in 0..dst_height {
        let src_y = (y * src_height / dst_height).min(src_height - 1);
        let src_row = (src_y * src_width * 3) as usize;
        let dst_row = (y * dst_width * 3) as usize;

        for x in 0..dst_width {
            let src_x = (x * src_width / dst_width).min(src_width - 1);
            let si = src_row + (src_x * 3) as usize;
            let di = dst_row + (x * 3) as usize;
            dst[di] = src[si];
            dst[di + 1] = src[si + 1];
            dst[di + 2] = src[si + 2];
        }
    }

    Some(DynamicImage::ImageRgb8(RgbImage::from_raw(
        dst_width, dst_height, dst,
    )?))
}
