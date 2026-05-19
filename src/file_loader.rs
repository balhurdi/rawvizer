use crate::error::{Error, Result};
use std::{fs::File, io::Read, os::unix::fs::FileExt};

pub struct FileBuffer {
    inner: *const u8,
    size: usize,
}

impl FileBuffer {
    fn new(buff: &[u8]) -> Self {
        Self {
            inner: buff.as_ptr(),
            size: buff.len(),
        }
    }

    pub fn data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.inner, self.size) }
    }
}

#[derive(Debug)]
pub struct FileLoader {
    file: File,
    cache: Vec<u8>,
    block_size: usize,
    path: String,
    current_block: usize,
    enable_loop: bool,
}

impl FileLoader {
    pub fn new(path: &str, block_size: usize, enable_loop: bool) -> Result<Self> {
        let mut file = File::open(path).map_err(|e| Error::OpenFile(path.to_string(), e))?;
        let mut cache = vec![0; block_size];

        file.read(&mut cache)
            .map_err(|e| Error::ReadFile(path.to_string(), e))?;

        Ok(Self {
            file,
            cache,
            block_size,
            path: path.to_string(),
            current_block: 0,
            enable_loop,
        })
    }
}

impl Iterator for FileLoader {
    type Item = Result<FileBuffer>;

    #[tracing::instrument]
    fn next(&mut self) -> Option<Self::Item> {
        match self
            .file
            .read_at(
                &mut self.cache,
                (self.current_block * self.block_size) as u64,
            )
            .map_err(|e| Error::ReadFile(self.path.clone(), e))
        {
            Err(e) => Some(Err(e)),
            Ok(x) if x < self.block_size => {
                if self.enable_loop {
                    self.current_block = 0;
                    self.next()
                } else {
                    None
                }
            }
            _ => {
                self.current_block += 1;
                Some(Ok(FileBuffer::new(&self.cache)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::file_loader::FileLoader;

    fn buff_to_u32(buff: &[u8]) -> u32 {
        std::str::from_utf8(buff).unwrap().parse::<u32>().unwrap()
    }

    fn file_loader_next_u32(file_loader: &mut FileLoader) -> u32 {
        buff_to_u32(file_loader.next().unwrap().unwrap().data())
    }

    #[test]
    fn load_file() {
        let block_size = 3;
        let file_path = "tests/file-loader-basic";
        let file_loader = FileLoader::new(file_path, block_size, false).unwrap();

        file_loader.enumerate().for_each(|(index, buff)| {
            let buff = buff.unwrap();
            let n: usize = std::str::from_utf8(buff.data()).unwrap().parse().unwrap();
            assert_eq!(n, index)
        });
    }

    #[test]
    fn loop_file() {
        let block_size = 3;
        let file_path = "tests/file-loop";
        let mut file_loader = FileLoader::new(file_path, block_size, true).unwrap();

        assert_eq!(file_loader_next_u32(&mut file_loader), 1);
        assert_eq!(file_loader_next_u32(&mut file_loader), 2);
        assert_eq!(file_loader_next_u32(&mut file_loader), 3);

        assert_eq!(file_loader_next_u32(&mut file_loader), 1);
        assert_eq!(file_loader_next_u32(&mut file_loader), 2);
        assert_eq!(file_loader_next_u32(&mut file_loader), 3);
    }
}
