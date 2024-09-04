use crate::{error::RNGProgramError::InvalidInstruction, state::{Book, Config, UserAccount}, };
use borsh::BorshDeserialize;
use solana_program::{msg, program_error::ProgramError};

#[derive(Debug, PartialEq)]
pub enum RNGProgramInstruction { 
  user_count_acc,
  create_user_acc,
  book_count_acc,
  create_book_acc,
  delete_book_acc,
  borrow_book_acc,
  return_book_acc,
  add_book_acc{data:u64},
  remove_book_acc{data:u64},
  config_acc,
  
}

impl RNGProgramInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
  
      let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
       
      Ok(match tag {
        0 => Self::user_count_acc,
        1 => Self::create_user_acc,
        2 => Self::book_count_acc,
        3 => Self::create_book_acc,
        4 => Self::delete_book_acc,
        5 => Self::borrow_book_acc,
        6 => Self::return_book_acc,
        7 => {
         if rest.len()!= 8 {
          return Err(InvalidInstruction.into());
        }
        let mut eightbyteaarr = [0u8;8];
        eightbyteaarr.copy_from_slice(&rest[..8]);
        let data = u64::from_le_bytes(eightbyteaarr);
        Self::add_book_acc { 
          data:data }
      },
      8 => {
        if rest.len()!= 8 {
         return Err(InvalidInstruction.into());
       }
       let mut bytearr = [0u8;8];
       bytearr.copy_from_slice(&rest[..8]);
       let data = u64::from_le_bytes(bytearr);
       Self::remove_book_acc { 
         data:data }
     },
     9 => Self::config_acc,
        _ => return Err(InvalidInstruction.into()),
      })
    }
  }
  
  