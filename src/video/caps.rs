#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    RGB8,
    V210,
}

impl PixelFormat {
    pub fn pixel_size_bytes(self) -> usize {
        match self {
            Self::RGB8 => 3,
            Self::V210 => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VideoFrameFormat {
    pub pixel_format: PixelFormat,
    pub width: u16,
    pub height: u16,
}

impl VideoFrameFormat {
    pub fn frame_size(self) -> usize {
        match self.pixel_format {
            PixelFormat::RGB8 => {
                (self.width as usize)
                    * (self.height as usize)
                    * self.pixel_format.pixel_size_bytes()
            }
            PixelFormat::V210 => {
                let width = self.width as usize;
                let height = self.height as usize;
                let groups = (width + 5) / 6;
                groups * 16 * height
            }
        }
    }
}
