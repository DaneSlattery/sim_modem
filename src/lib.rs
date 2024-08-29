#![cfg_attr(not(feature = "std"), no_std)]

mod error;
mod impls;

use error::ModemError;

pub use impls::*;

pub trait SimModem<T: ?Sized> {
    fn get_mode(&self) -> &CommunicationMode;

    fn negotiate(&mut self, comm: &mut T, buffer: &mut [u8]) -> Result<(), ModemError>;
}

// pub struct BufReader<'a, R: ?Sized> {
//     buf: &'a mut [u8],
//     pos: usize,
//     inner: R,
// }

// impl<'a, R: Read> BufReader<'a, R> {
//     pub fn new(reader: R, buff: &'a mut [u8]) -> Self {
//         Self {
//             buf: buff,
//             pos: 0,
//             inner: reader,
//         }
//     }
// }

// impl<'a, R: Read> ErrorType for BufReader<'a, R> {
//     type Error = R::Error;
// }

// impl<'a, R: Read> BufRead for BufReader<'a, R> {
//     fn consume(&mut self, amt: usize) {
//         // self
//         self.pos += amt;
//     }

//     fn fill_buf(&mut self) -> Result<&[u8], Self::Error> {
//         // fill the inner buffer
//         let read_count = self.inner.read(&mut self.buf[self.pos..])?;
//         // return the read bytes
//         Ok(&self.buf[self.pos..(self.pos + read_count)])
//     }
// }

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
