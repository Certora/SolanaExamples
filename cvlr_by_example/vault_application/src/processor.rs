use crate::state::Vault;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
};

pub fn process_deposit(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let tkn = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );

    let vault_info = next_account_info(account_info_iter)?;
    if vault_info.owner != &crate::id() {
        return Err(ProgramError::IllegalOwner);
    }

    let mut data = vault_info.data.borrow_mut();
    let vault: &mut Vault = bytemuck::from_bytes_mut(&mut data[..]);

    vault.deposit(tkn);

    Ok(())
}

pub fn process_withdraw(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let shares = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );

    let vault_info = next_account_info(account_info_iter)?;
    if vault_info.owner != &crate::id() {
        return Err(ProgramError::IllegalOwner);
    }

    let mut data = vault_info.data.borrow_mut();
    let vault: &mut Vault = bytemuck::from_bytes_mut(&mut data[..]);

    vault.withdraw(shares);

    Ok(())
}

pub fn process_reward(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let tkn = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );

    let vault_info = next_account_info(account_info_iter)?;
    if vault_info.owner != &crate::id() {
        return Err(ProgramError::IllegalOwner);
    }

    let mut data = vault_info.data.borrow_mut();
    let vault: &mut Vault = bytemuck::from_bytes_mut(&mut data[..]);

    vault.reward(tkn);

    Ok(())
}

pub fn process_slash(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let tkn = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );

    let vault_info = next_account_info(account_info_iter)?;
    if vault_info.owner != &crate::id() {
        return Err(ProgramError::IllegalOwner);
    }

    let mut data = vault_info.data.borrow_mut();
    let vault: &mut Vault = bytemuck::from_bytes_mut(&mut data[..]);

    vault.slash(tkn);

    Ok(())
}
