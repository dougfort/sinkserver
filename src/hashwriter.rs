use ring::digest::{Context, SHA256};

/// implements std::io::Write for ring::digest Context
pub struct HashWriter {
    /// A context for multi-step (Init-Update-Finish) digest calculations.
    ctx: Context,
}

impl HashWriter {
    /// Returns a HashWriter ready to use
    pub fn new() -> Self {
        HashWriter {
            ctx: Context::new(&SHA256),
        }
    }

    /// Closes the HashWriter and returns the accumulated Digest
    pub fn close(self) -> Vec<u8> {
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
    use super::HashWriter;
    use std::io::Write;
    #[test]
    fn create() {
        let mut h = HashWriter::new();
        h.write_all(&[0, 1, 2]).unwrap();
    }
}
