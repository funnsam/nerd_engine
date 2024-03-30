use std::io;

pub trait ReadChar {
    fn read_char(&mut self) -> io::Result<char>;
}

impl<R: io::Read> ReadChar for R {
    fn read_char(&mut self) -> io::Result<char> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;

        match buf[0] >> 3 {
            0b00000..=0b01111 => 
        }
    }
}
