
use ring::digest::{Context, SHA256};

pub struct HashWriter {
    ctx: Context
}

impl HashWriter {
    pub fn new() -> Self {
        HashWriter{
            ctx: Context::new(&SHA256),
        }
    }

    pub fn close(self) -> Vec::<u8> {
        self.ctx.finish().as_ref().to_vec()
    }
}

impl Default for HashWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl std::io::Write for HashWriter {

    /// Write a buffer into this writer, returning how many bytes were written.
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.ctx.update(buf);
        Ok(buf.len())
    }
  
    /// Flush this output stream, ensuring that all intermediately buffered contents reach their destination.
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }    
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use super::HashWriter;
    #[test]
    fn create() {
       let mut h = HashWriter::new();
       h.write_all(&[0, 1, 2]).unwrap();
    }
}
