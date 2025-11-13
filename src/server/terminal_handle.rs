use std::io;
use tokio::sync::mpsc::UnboundedSender;

pub struct TerminalHandle {
    sender: UnboundedSender<Vec<u8>>,
    sink: Vec<u8>,
}

impl TerminalHandle {
    pub fn new_with_sender(sender: UnboundedSender<Vec<u8>>) -> Self {
        Self {
            sender,
            sink: Vec::new(),
        }
    }
}

impl std::io::Write for TerminalHandle {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.sink.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let result = self.sender.send(self.sink.clone());
        if result.is_err() {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                result.unwrap_err(),
            ));
        }

        self.sink.clear();
        Ok(())
    }
}
