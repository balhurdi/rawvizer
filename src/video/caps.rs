#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFromat {
    RGB8,
}

impl PixelFromat {
    pub fn pixel_size_bytes(self) -> usize {
        match self {
            Self::RGB8 => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VideoFrameFormat {
    pub pixel_format: PixelFromat,
    pub width: u16,
    pub height: u16,
}

impl VideoFrameFormat {
    pub fn frame_size(self) -> usize {
        match self.pixel_format {
            PixelFromat::RGB8 => {
                (self.width as usize)
                    * (self.height as usize)
                    * self.pixel_format.pixel_size_bytes()
            }
        }
    }
}
