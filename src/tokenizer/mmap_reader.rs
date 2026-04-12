use memmap2::Mmap;

pub struct MmapReader {
    file:Mmap,
    byte_index:usize
}

impl MmapReader {

    pub fn new(file: Mmap) -> Self {
        Self { file, byte_index: 0 }
    }

    fn utf8_to_char(slice:&[u8]) -> Result<char, UTF8ReadError> {
        let chunk = slice.utf8_chunks().next().ok_or(UTF8ReadError::Eof)?;
        chunk.valid().chars().next().ok_or(UTF8ReadError::InvalidByte)
    }


}

impl Iterator for MmapReader {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let end_index = (self.file.len() - self.byte_index).clamp(0, 4) + self.byte_index;
            let next_slice = self.file.get(self.byte_index..end_index)?;
            match Self::utf8_to_char(next_slice) {
                Ok(char) => {
                    self.byte_index+=char.len_utf8();
                    return Some(char)
                },
                Err(UTF8ReadError::Eof) => return None,
                Err(UTF8ReadError::InvalidByte) => self.byte_index+=1
            }
        }
    }
}

enum UTF8ReadError {
    Eof,
    InvalidByte
}