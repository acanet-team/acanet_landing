use borsh::{BorshDeserialize};
use solana_program::{program_error::ProgramError};

pub enum PriceInstruction {
    FetchCurrentPrice {
        round: u8
    }
}

#[derive(BorshDeserialize)]
struct PricePayload {
  round: u8
}

impl PriceInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
          let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
          let payload = PricePayload::try_from_slice(rest).unwrap();
          Ok(match variant {
              0 => Self::FetchCurrentPrice {round: payload.round },
              _ => return Err(ProgramError::InvalidInstructionData)
          })
      }
  }