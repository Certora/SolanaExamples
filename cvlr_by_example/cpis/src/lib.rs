use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, pubkey::Pubkey,
};
mod processor;

// Include formal verification module only if certora feature is enabled.
#[cfg(feature = "certora")]
mod certora;

#[cfg(not(feature = "certora"))]
use solana_program::msg;
// If certora feature is enabled, msg should be substituted with `clog!`.
#[cfg(feature = "certora")]
use cvlr::clog as msg;

declare_id!("4tjxVuepBgMVCbrdN3qx9pzt5zFy2bCwgyVrtidXY85u");

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (instruction_discriminant, instruction_data) = instruction_data.split_at(1);
    match instruction_discriminant[0] {
        0 => processor::process_transfer(accounts, instruction_data)?,
        1 => processor::process_create_account(program_id, accounts, instruction_data)?,
        2 => processor::process_transfer_token(accounts, instruction_data)?,
        3 => processor::process_transfer_token_2022(accounts, instruction_data)?,
        4 => processor::process_mint_token(accounts, instruction_data)?,
        5 => processor::process_mint_token_2022(accounts, instruction_data)?,
        _ => msg!("Error: unknown instruction"),
    }
    Ok(())
}
