use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
    system_program,
};

pub fn process_transfer(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let from_account = next_account_info(account_info_iter)?;
    let to_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let transfer_amount = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );

    invoke_transfer(from_account, to_account, system_program, transfer_amount)?;

    Ok(())
}

#[cfg(not(feature = "mockcpis"))]
fn invoke_transfer<'a>(
    from_account: &AccountInfo<'a>,
    to_account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    transfer_amount: u64,
) -> Result<(), ProgramError> {
    if system_program.key != &system_program::ID {
        return Err(solana_program::program_error::ProgramError::InvalidArgument);
    }
    invoke(
        &system_instruction::transfer(from_account.key, to_account.key, transfer_amount),
        // &francesco_transfer(from_account.key, to_account.key, transfer_amount),
        &[
            from_account.clone(),
            to_account.clone(),
            system_program.clone(),
        ],
    )?;
    Ok(())
}

#[cfg(feature = "mockcpis")]
fn invoke_transfer<'a>(
    from_account: &AccountInfo<'a>,
    to_account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    transfer_amount: u64,
) -> Result<(), ProgramError> {
    if system_program.key != &system_program::ID {
        return Err(solana_program::program_error::ProgramError::InvalidArgument);
    }
    **from_account.try_borrow_mut_lamports()? -= transfer_amount;
    **to_account.try_borrow_mut_lamports()? += transfer_amount;
    Ok(())
}
