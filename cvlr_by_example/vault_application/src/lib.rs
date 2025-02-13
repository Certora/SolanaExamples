use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};
mod certora;
mod processor;
mod state;

declare_id!("4tjxVuepBgMVCbrdN3qx9pzt5zFy2bCwgyVrtidXY85o");

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (instruction_discriminant, instruction_data) = instruction_data.split_at(1);
    match instruction_discriminant[0] {
        0 => {
            msg!("Instruction: deposit");
            processor::process_deposit(accounts, instruction_data)?;
        }
        1 => {
            msg!("Instruction: withdraw");
            processor::process_withdraw(accounts, instruction_data)?;
        }
        2 => {
            msg!("Instruction: reward");
            processor::process_reward(accounts, instruction_data)?;
        }
        3 => {
            msg!("Instruction: slash");
            processor::process_slash(accounts, instruction_data)?;
        }
        _ => {
            msg!("Error: unknown instruction")
        }
    }
    Ok(())
}
