use pinocchio::program_error::ProgramError;

#[derive(Clone, PartialEq, shank::ShankType)]
pub enum MyProgramError {
    // overflow error
    WriteOverflow,
    // invalid instruction data
    InvalidInstructionData,
    // pda mismatch
    PdaMismatch,
    // Invalid Owner
    InvalidOwner,
    // Invalid Hex
    InvalidHex,
    // Invalid conversion price
    InvalidPriceConversion,
    // Time too short
    TimeTooShort,
    // Amount too small
    DepositTooSmall,
}

impl From<MyProgramError> for ProgramError {
    fn from(e: MyProgramError) -> Self {
        Self::Custom(e as u32)
    }
}
