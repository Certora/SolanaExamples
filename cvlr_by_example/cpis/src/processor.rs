use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction, system_program,
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

pub fn process_create_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let owner = program_id;
    let payer = &accounts[0];
    let new_account = &accounts[1];

    let lamports = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );
    let space = u64::from_le_bytes(
        instruction_data[8..16]
            .try_into()
            .expect("Invalid slice length"),
    );

    invoke_create_account(owner, payer, new_account, lamports, space)?;
    Ok(())
}

#[cfg(not(feature = "mockcpis"))]
fn invoke_create_account<'a>(
    owner: &Pubkey,
    payer: &AccountInfo<'a>,
    new_account: &AccountInfo<'a>,
    lamports: u64,
    space: u64,
) -> Result<(), ProgramError> {
    invoke(
        &system_instruction::create_account(payer.key, new_account.key, lamports, space, owner),
        &[payer.clone(), new_account.clone()],
    )?;
    Ok(())
}

#[cfg(feature = "mockcpis")]
fn invoke_create_account<'a>(
    _owner: &Pubkey,
    payer: &AccountInfo<'a>,
    new_account: &AccountInfo<'a>,
    lamports: u64,
    space: u64,
) -> Result<(), ProgramError> {
    **payer.try_borrow_mut_lamports()? -= lamports;
    **new_account.try_borrow_mut_lamports()? = lamports;
    cvlr::cvlr_assume!(new_account.try_borrow_mut_data()?.len() == space as usize);
    Ok(())
}
