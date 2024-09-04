use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum RNGProgramError {
  /// Invalid Instruction
  #[error("Invalid Instruction")]
  InvalidInstruction,

  #[error("Collection operation error")]
  CollectionoperationError,

  #[error("subtraction error")]
  SubtractionError,

  #[error("Authorization")]
  AuthorityError,

  #[error("Ownership error")]
  OwnershipError,

  #[error("Wrong PDA Address")]
  WrongPdaAddressError,

  #[error("No books to delete")]
  NoBooksToDeleteError,

  #[error("No books in circulation")]
  NoBooksInCirculation,
  
  #[error("Invalid Owner Err")]
  InvalidOwnerErr,

  #[error("Unauthorized user")]
  UnauthorizedUser,

  #[error("Book Not Available")]
  BookNotAvailable,
}

impl From<RNGProgramError> for ProgramError {
  fn from(e: RNGProgramError) -> Self {
    ProgramError::Custom(e as u32)
  }
}