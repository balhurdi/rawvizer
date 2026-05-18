use image::{DynamicImage, RgbImage};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{
    error::{Error, Result},
    file_loader::FileLoader,
};

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
}

impl Tape {
    pub fn new(file_loader: FileLoader) -> Self {
        Self { file_loader }
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
                        let image = RgbImage::from_raw(1920, 1080, f.data().to_vec())
                            .ok_or(Error::InvalidBufferSize);

                        match image {
                            Ok(img) => {
                                let dynamic_image = DynamicImage::from(img);
                                let _ = frame_receiver_tx
                                    .send(FrameReceiverEvent::Frame(dynamic_image));
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
