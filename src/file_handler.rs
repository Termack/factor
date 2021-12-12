use std::io::{Read, Write};
use std::fs::File;
use std::error;
use numext_fixed_uint::U1024;

type Result<T> = std::result::Result<T,Box<dyn error::Error>>;

pub struct FileHandler{
    input: File,
    buffer: Vec<u8>,
    bytes: usize,
    filename: String
}



impl FileHandler {
    pub fn new(filename: &str) -> FileHandler{
        let buffer = vec![0;128];
        let f = File::open(filename).unwrap();
        FileHandler {
            input: f,
            buffer: buffer,
            bytes: 0,
            filename: String::from(filename)
        }
    }

    pub fn read_int(&mut self) -> Result<U1024> {
        let bytes = self.input.read(&mut self.buffer)?;
        let num = U1024::from_big_endian(&self.buffer[0..bytes])?;
        self.bytes = bytes;
        Ok(num)
    }

    pub fn write_from_int(&mut self, num: U1024) -> Result<()> {
        num.into_big_endian(&mut self.buffer)?;
        let mut file = File::create(format!("{}.fcmp",self.filename))?;
        file.write(&self.buffer[self.buffer.len()-self.bytes..])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use numext_fixed_uint::u1024;
    use super::*;

    fn filename<'a>() -> &'a str{
        "resources/test/test.txt"
    }

    #[test]
    fn read_into_int() {
        
        let mut r = FileHandler::new(filename());

        assert_eq!(r.read_int().unwrap(),u1024!("0b11000010110001001100011"));
    }

}