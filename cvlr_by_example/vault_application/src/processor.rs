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
    let authority_info = next_account_info(account_info_iter)?;

    if vault_info.owner != &crate::id() {
        return Err(ProgramError::IllegalOwner);
    }

    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut data = vault_info.data.borrow_mut();
    let vault: &mut Vault = bytemuck::from_bytes_mut(&mut data[..]);

    if authority_info.key != &vault.owner {
        return Err(ProgramError::Custom(101));
    }

    let mut shares_total: u64 = vault.shares_total.into();
    let mut token_total: u64 = vault.token_total.into();
    let shares_for_user = if shares_total == token_total {
        tkn
    } else {
        mul_div_floor(tkn, token_total, token_total)
    };

    mint_shares(&mut shares_total, shares_for_user);
    add_token(&mut token_total, tkn);

    vault.shares_total = shares_total.into();
    vault.token_total = token_total.into();

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

    let mut shares_total: u64 = vault.shares_total.into();
    let mut token_total: u64 = vault.token_total.into();
    let tkn_for_user = if shares_total == token_total {
        shares
    } else {
        mul_div_floor(shares, token_total, shares_total)
    };

    burn_shares(&mut shares_total, shares);
    del_token(&mut token_total, tkn_for_user);

    vault.shares_total = shares_total.into();
    vault.token_total = token_total.into();

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

    let mut token_total: u64 = vault.token_total.into();
    add_token(&mut token_total, tkn);

    vault.token_total = token_total.into();

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

    let mut token_total: u64 = vault.token_total.into();
    del_token(&mut token_total, tkn);

    Ok(())
}

fn add_token(token_total: &mut u64, tkn: u64) {
    assert!(tkn > 0);
    *token_total = token_total.checked_add(tkn).unwrap();
}

fn mint_shares(shares_total: &mut u64, shares_for_user: u64) {
    assert!(shares_for_user > 0);
    *shares_total = shares_total.checked_add(shares_for_user).unwrap();
}

fn burn_shares(shares_total: &mut u64, shares: u64) {
    *shares_total = shares_total.checked_sub(shares).unwrap();
}

fn del_token(token_total: &mut u64, tkn_for_user: u64) {
    *token_total = token_total.checked_sub(tkn_for_user).unwrap();
}

fn mul_div_floor(a: u64, b: u64, c: u64) -> u64 {
    (a as u128)
        .checked_mul(b as u128)
        .unwrap()
        .checked_div(c as u128)
        .unwrap()
        .try_into()
        .unwrap()
}
