#![no_std]
mod error;
mod impls;

use embedded_io::{BufRead, ErrorType, Read};
use error::ModemError;

pub use impls::*;

pub trait SimModem<T, const N: usize> {
    fn get_mode(&self) -> &CommunicationMode;

    fn negotiate(&mut self, comm: &mut T, buffer: [u8; N]) -> Result<(), ModemError>;
}

pub struct BufReader<const N: usize, R: ?Sized> {
    buf: [u8; N],
    pos: usize,
    inner: R,
}

impl<const N: usize, R: Read> ErrorType for BufReader<N, R> {
    type Error = R::Error;
}

impl<const N: usize, R: Read> BufRead for BufReader<N, R> {
    fn consume(&mut self, amt: usize) {
        // self
        self.pos += amt;
    }

    fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
        // fill the inner buffer
        let read_count = self.inner.read(&mut self.buf[self.pos..])?;
        // return the read bytes
        Ok(&self.buf[self.pos..(self.pos + read_count)])
    }
}

pub enum DigestError {
    Error,
    Continue,
    BufferOverflow,
}

pub trait Digester {
    /// Termination characters if the response indicates the OK state.
    const OK_TERMINATOR: &'static str;
    /// Termination characters if the response indicates the Error state.
    const ERR_TERMINATOR: &'static str;
    /// parse the buffer, searching for finaliser.
    /// If the
    fn digest(&mut self, data: &[u8]) -> Result<&[u8], DigestError>;
}

pub struct SimDigester<const N: usize> {
    buf: [u8; N],
    pos: usize,
}

impl<const N: usize> Digester for SimDigester<N> {
    const OK_TERMINATOR: &'static str = "\r\nOK\r\n";
    const ERR_TERMINATOR: &'static str = "\r\nERROR\r\n";

    fn digest(&mut self, data: &[u8]) -> Result<&[u8], DigestError> {
        let incoming_len = data.len();
        if self.pos + incoming_len > N {
            return Err(DigestError::BufferOverflow);
        }
        self.buf[self.pos..self.pos + incoming_len].copy_from_slice(data);
        self.pos += incoming_len;
        // look for the terminator, get to the choppa
        let ok_len = Self::OK_TERMINATOR.len();
        let end = &self.buf[(self.pos - ok_len)..self.pos];
        if end == Self::OK_TERMINATOR.as_bytes() {
            // ready to parse
            return Ok(&self.buf[..self.pos]);
        }
        let err_len = Self::ERR_TERMINATOR.len();
        let end: &[u8] = &self.buf[(self.pos - err_len)..self.pos];
        if end == Self::ERR_TERMINATOR.as_bytes() {
            // result indicated failure
            return Err(DigestError::Error);
        }
        Err(DigestError::Continue)
    }
}

/// State of the modem.
///
/// In [CommunicationMode::Command] mode, AT commands will function,
/// serving to put the modem into [CommunicationMode::Data].
///
/// In [CommunicationMode::Data] the modem device will act as a Point-To-Point over Serial (PPPoS)
/// server.
pub enum CommunicationMode {
    Command,
    Data,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
