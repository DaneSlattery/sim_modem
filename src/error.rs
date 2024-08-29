#[derive(Debug)]
pub enum ModemError {
    IO,
    DigestError,
    ATParse(at_commands::parser::ParseError),
    ATBuild(usize),
}

impl core::fmt::Display for ModemError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ModemError {}

// #[cfg(feature = "std")]
// impl core::fmt::Display for ModemError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

impl From<usize> for ModemError {
    fn from(value: usize) -> Self {
        ModemError::ATBuild(value)
    }
}

impl From<at_commands::parser::ParseError> for ModemError {
    fn from(value: at_commands::parser::ParseError) -> Self {
        ModemError::ATParse(value)
    }
}
