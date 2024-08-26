#[derive(Debug)]
pub enum ModemError {
    IO,
    ATParse(at_commands::parser::ParseError),
    ATBuild(usize),
}

impl core::fmt::Display for ModemError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// impl std::error::Error for ModemError {}

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
