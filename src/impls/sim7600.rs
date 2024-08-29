use core::fmt::Display;

use at_commands::{builder::CommandBuilder, parser::CommandParser};

use crate::{error::ModemError, CommunicationMode, SimModem};

pub struct SIM7600(CommunicationMode);

impl SIM7600 {
    pub fn new() -> Self {
        Self(CommunicationMode::Command)
    }
}

impl Default for SIM7600 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "std"))]
impl<T: embedded_io::Read + embedded_io::Write> SimModem<T> for SIM7600 {
    fn negotiate(&mut self, comm: &mut T, buffer: &mut [u8]) -> Result<(), ModemError> {
        reset(comm, buffer)?;

        // //disable echo
        // set_echo(comm, &mut buffer, false)?;

        // // get signal quality
        // let (rssi, ber) = get_signal_quality(comm, &mut buffer)?;
        // log::info!("RSSI = {rssi}");
        // log::info!("BER = {ber}");
        // // get iccid
        // let iccid = get_iccid(comm, &mut buffer)?;
        // log::info!("ICCID = [{}]", iccid);

        // // check pdp network reg
        // read_gprs_registration_status(comm, &mut buffer)?;

        // //configure apn
        // set_pdp_context(comm, &mut buffer)?;

        // // start ppp
        // set_data_mode(comm, &mut buffer)?;

        self.0 = CommunicationMode::Data;
        Ok(())
    }

    fn get_mode(&self) -> &CommunicationMode {
        &self.0
    }
}

#[cfg(feature = "std")]
impl<T: std::io::Read + std::io::Write> SimModem<T> for SIM7600 {
    fn negotiate(&mut self, comm: &mut T, buffer: &mut [u8]) -> Result<(), ModemError> {
        reset(comm, buffer)?;

        // //disable echo
        // set_echo(comm, &mut buffer, false)?;

        // // get signal quality
        // let (rssi, ber) = get_signal_quality(comm, &mut buffer)?;
        // log::info!("RSSI = {rssi}");
        // log::info!("BER = {ber}");
        // // get iccid
        // let iccid = get_iccid(comm, &mut buffer)?;
        // log::info!("ICCID = [{}]", iccid);

        // // check pdp network reg
        // read_gprs_registration_status(comm, &mut buffer)?;

        // //configure apn
        // set_pdp_context(comm, &mut buffer)?;

        // // start ppp
        // set_data_mode(comm, &mut buffer)?;

        self.0 = CommunicationMode::Data;
        Ok(())
    }

    fn get_mode(&self) -> &CommunicationMode {
        &self.0
    }
}

/// Bit Error Rate as a percentage
pub enum BitErrorRate {
    /// < 0.01%
    LT001,
    /// 0.01% - 0.1%
    LT01,
    /// 0.1% - 0.5%
    LT05,
    /// 0.5% - 1%
    LT1,
    /// 1% - 2%
    LT2,
    /// 2% - 4%
    LT4,
    /// 4% - 8%
    LT8,
    /// >=8%
    GT8,
    /// unknown or undetectable
    Unknown,
}

impl Display for BitErrorRate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            BitErrorRate::GT8 => write!(f, ">= 8%"),
            BitErrorRate::LT001 => write!(f, "< 0.01%"),
            BitErrorRate::LT01 => write!(f, "0.01% - 0.1%"),
            BitErrorRate::LT05 => write!(f, "0.1% - 0.5%"),
            BitErrorRate::LT1 => write!(f, "0.5% - 1%"),
            BitErrorRate::LT2 => write!(f, "1% - 2%"),
            BitErrorRate::LT4 => write!(f, "2% - 4%"),
            BitErrorRate::LT8 => write!(f, "4% - 8%"),
            BitErrorRate::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<i32> for BitErrorRate {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::LT001,
            1 => Self::LT01,
            2 => Self::LT05,
            3 => Self::LT1,
            4 => Self::LT2,
            5 => Self::LT4,
            6 => Self::LT8,
            7 => Self::GT8,
            _ => Self::Unknown,
        }
    }
}

/// Received Signal Strength Indication
pub enum RSSI {
    /// -113 dBm or less
    DBMLT113,
    /// -111 dBm
    DBM111,
    /// -109 to -53 dBm
    DBM109_53(i32),
    /// -51 dBm or greater
    DBMGT51,
    /// not known or not detectable
    Unknown,
    /// -116 dBm or less
    DBMLT116,
    /// -115 dBm
    DBM115,
    /// -114 to -26 dBm
    DBM114_26(i32),
    /// -25 dBm or greater
    DBMGT25,
}

impl Display for RSSI {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            RSSI::DBMLT113 => write!(f, "<= -113 dBm"),
            RSSI::DBM111 => write!(f, "-111 dBm"),
            RSSI::DBM109_53(x) => write!(f, "{} dBm", x),
            RSSI::DBMGT51 => write!(f, ">= -51 dBm"),
            RSSI::DBM114_26(x) => write!(f, "{} dBm", x),
            RSSI::DBM115 => write!(f, "-115 dBm"),
            RSSI::DBMGT25 => write!(f, ">= -25 dBm"),
            RSSI::DBMLT116 => write!(f, "<= -116 dBm"),
            RSSI::Unknown => write!(f, "Unknown"),
        }
    }
}

impl RSSI {
    pub fn parse(raw: i32) -> RSSI {
        match raw {
            0 => Self::DBMLT113,
            1 => Self::DBM111,
            2..=30 => Self::DBM109_53(RSSI::map2_30_to_109_53(raw)),
            31 => Self::DBMGT51,
            99 => Self::Unknown,
            100 => Self::DBMLT116,
            101 => Self::DBM115,
            102..=191 => Self::DBM114_26(RSSI::map102_191_to_114_26(raw)),
            _ => Self::Unknown,
        }
    }

    fn map2_30_to_109_53(raw: i32) -> i32 {
        const X1: i32 = 2;
        const Y1: i32 = -109;
        const X2: i32 = 30;
        const Y2: i32 = -53;
        const GRAD: i32 = (Y2 - Y1) / (X2 - X1); // 56/28 = 2
        const OFFSET: i32 = Y1 - (GRAD * X1); // -113
        (GRAD * raw) + OFFSET
    }

    fn map102_191_to_114_26(raw: i32) -> i32 {
        const X1: i32 = 102;
        const Y1: i32 = -114;
        // const X2: i32 = 191;
        // const Y2: i32 = -26;
        const GRAD: i32 = 1;
        // requires #![feature(int_roundings)]
        // const GRAD: i32 = (Y2 - Y1).div_ceil((X2 - X1)); // would be 88/89, so truncated to 0
        const OFFSET: i32 = Y1 - (GRAD * X1); // -216
        (GRAD * raw) + OFFSET
    }
}

fn reset<T: std::io::Read + std::io::Write>(
    comm: &mut T,
    buff: &mut [u8],
) -> Result<(), ModemError> {
    let cmd = CommandBuilder::create_execute(buff, false)
        .named("ATZ0")
        .finish()?;
    log::info!("Send Reset");

    comm.write(cmd).map_err(|_| ModemError::IO)?;

    // read until terminator
    let len = read_until_term(comm, buff)?;

    log::info!("got response{:?}", core::str::from_utf8(&buff[..len]));
    CommandParser::parse(&buff[..len])
        .expect_identifier(b"ATZ0\r")
        .expect_identifier(b"\r\nOK\r\n")
        .finish()?;
    Ok(())
}

// fn reset<T: embedded_io::Read + embedded_io::Write + embedded_io::BufRead>(
//     comm: &mut T,
//     buff: &mut [u8],
// ) -> Result<(), ModemError> {
//     let cmd = CommandBuilder::create_execute(buff, false)
//         .named("ATZ0")
//         .finish()?;
//     log::info!("Send Reset");

//     comm.write(cmd).map_err(|_| ModemError::IO)?;

//     // read until terminator
//     let len = read_until_term(comm, buff)?;

//     log::info!("got response{:?}", core::str::from_utf8(&buff[..len]));
//     CommandParser::parse(&buff[..len])
//         .expect_identifier(b"ATZ0\r")
//         .expect_identifier(b"\r\nOK\r\n")
//         .finish()?;
//     Ok(())
// }
const OK_TERMINATOR: &'static str = "\r\nOK\r\n";
const ERR_TERMINATOR: &'static str = "\r\nERR\r\n";

fn read_until_term<T: std::io::Read>(comm: &mut T, buff: &mut [u8]) -> Result<usize, ModemError> {
    let mut pos = 0;

    loop {
        // start filling the buffer
        let len = comm.read(&mut buff[pos..]).map_err(|_| ModemError::IO)?;
        if len == 0 {
            continue;
        }
        pos += len;
        log::info!(
            "Len: {}, data = {:?}",
            len,
            std::str::from_utf8(&buff[..pos])
        );

        let ok_len = OK_TERMINATOR.len();
        if pos >= ok_len {
            let end = &buff[pos - ok_len..pos];
            log::info!(" end = {:?}", std::str::from_utf8(&buff[pos - ok_len..pos]));
            if end == OK_TERMINATOR.as_bytes() {
                // buff[..pos].copy_from_slice(data);
                return Ok(pos);
            }
        }

        let err_len = ERR_TERMINATOR.len();
        if pos >= err_len {
            let end = &buff[pos - err_len..pos];

            if end == ERR_TERMINATOR.as_bytes() {
                return Err(ModemError::DigestError);
            }
        }
        // return Err(ModemError::DigestError);
    }
}

// fn read_until_term<T: embedded_io::Read + BufRead>(
//     comm: &mut T,
//     buff: &mut [u8],
// ) -> Result<usize, ModemError> {
//     // let pos = 0;
//     loop {
//         let data = comm.fill_buf().map_err(|_| ModemError::IO)?;

//         let ok_len = OK_TERMINATOR.len();
//         let end = &data[data.len() - ok_len..];
//         if end == OK_TERMINATOR.as_bytes() {
//             buff[..data.len()].copy_from_slice(data);
//             return Ok(data.len());
//         }

//         let err_len = ERR_TERMINATOR.len();

//         let end = &data[data.len() - err_len..];

//         if end == ERR_TERMINATOR.as_bytes() {
//             buff.copy_from_slice(data);
//             return Err(ModemError::DigestError);
//         }
//     }
// }
