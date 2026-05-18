use image::{DynamicImage, RgbImage};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{
    error::{Error, Result},
    file_loader::FileLoader,
    video::{PixelFormat, VideoFrameFormat, convert_frame},
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

pub struct Tape {
    file_loader: FileLoader,
    frame_format: VideoFrameFormat,
    staging_buffer: Vec<u8>,
}

impl Tape {
    pub fn new(file_loader: FileLoader, frame_format: VideoFrameFormat) -> Self {
        let buffer_size = (frame_format.width as usize)
            * (frame_format.height as usize)
            * INTERNAL_PIXEL_FORMAT.pixel_size_bytes();
        Self {
            file_loader,
            frame_format,
            staging_buffer: vec![0u8; buffer_size],
        }
    }

    pub fn start(mut self) -> (TapeController, TapeFrameReceiver) {
        let (controller_tx, mut controller_rx) = mpsc::unbounded_channel::<TapeEvent>();
        let (frame_receiver_tx, frame_receiver_rx) =
            mpsc::unbounded_channel::<FrameReceiverEvent>();

        tokio::spawn(async move {
            loop {
                let tape_event = controller_rx.recv().await;

                if let Some(te) = tape_event {
                    let frame = match te {
                        TapeEvent::NextFrame => self.file_loader.next(),
                        TapeEvent::PreviousFrame => todo!(),
                    };

                    if let Some(Ok(f)) = frame {
                        match create_dynamic_image(
                            self.frame_format,
                            f.data(),
                            &mut self.staging_buffer,
                        ) {
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

fn create_dynamic_image(
    input_format: VideoFrameFormat,
    input_buffer: &[u8],
    output_buffer: &mut [u8],
) -> Result<DynamicImage> {
    let output_format = VideoFrameFormat {
        width: input_format.width,
        height: input_format.height,
        pixel_format: PixelFormat::RGB8,
    };

    convert_frame(input_format, input_buffer, output_format, output_buffer);

    let image = RgbImage::from_raw(
        input_format.width as u32,
        input_format.height as u32,
        output_buffer.to_vec(),
    )
    .ok_or(Error::InvalidBufferSize)?;

    let dynamic_image = DynamicImage::from(image).thumbnail(THUMBNAIL_WIDTH, THUMBNAIL_HEIGHT);

    Ok(dynamic_image)
}
