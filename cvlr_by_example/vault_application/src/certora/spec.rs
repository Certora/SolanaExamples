//! This module contains the specification for the vault application.

use crate::{assert_solvency, assume_solvency, processor::*, state::Vault};
use cvlr::{mathint::NativeInt, prelude::*};
use cvlr_solana::cvlr_deserialize_nondet_accounts;
use solana_program::account_info::{next_account_info, AccountInfo};

/// Structure tracking the state for the formal verification (FV) of the vault.
struct FvVault {
    shares_total: NativeInt,
    token_total: NativeInt,
}

impl From<&Vault> for FvVault {
    fn from(vault: &Vault) -> FvVault {
        let shares_total: u64 = vault.shares_total.into();
        let token_total: u64 = vault.token_total.into();
        FvVault {
            shares_total: shares_total.into(),
            token_total: token_total.into(),
        }
    }
}

impl<'a> From<&AccountInfo<'a>> for FvVault {
    fn from(acc_info: &AccountInfo) -> FvVault {
        let mut data = acc_info.data.borrow_mut();
        let vault: &Vault = bytemuck::from_bytes_mut(&mut data[..]);
        FvVault::from(vault)
    }
}

/// Verifies that a vault account remains solvent before and after a withdrawal
/// operation.
#[rule]
pub fn rule_vault_solvency_withdraw() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let vault_account: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let fv_vault_pre: FvVault = vault_account.into();
    assume_solvency!(fv_vault_pre);

    let shares: u64 = nondet();
    let shares_instruction_data = &shares.to_le_bytes();
    process_withdraw(&account_infos, shares_instruction_data).unwrap();

    let fv_vault_post: FvVault = vault_account.into();
    assert_solvency!(fv_vault_post);
}

/// Verifies that a vault account remains solvent before and after a deposit
/// operation.
#[rule]
pub fn rule_vault_solvency_deposit() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let vault_account: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let fv_vault_pre: FvVault = vault_account.into();
    assume_solvency!(fv_vault_pre);

    let token: u64 = nondet();
    let token_instruction_data = &token.to_le_bytes();
    process_deposit(&account_infos, token_instruction_data).unwrap();

    let fv_vault_post: FvVault = vault_account.into();
    assert_solvency!(fv_vault_post);
}

/// Verifies that a vault account remains solvent before and after a reward
/// operation.
#[rule]
pub fn rule_vault_solvency_reward() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let vault_account: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let fv_vault_pre: FvVault = vault_account.into();
    assume_solvency!(fv_vault_pre);

    let token: u64 = nondet();
    let token_instruction_data = &token.to_le_bytes();
    process_reward(&account_infos, token_instruction_data).unwrap();

    let fv_vault_post: FvVault = vault_account.into();
    assert_solvency!(fv_vault_post);
}

/// Verifies that a vault account remains solvent before and after a slash
/// operation.
/// This rule is expected to fail.
#[rule]
pub fn rule_vault_solvency_slash() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let vault_account: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let fv_vault_pre: FvVault = vault_account.into();
    assume_solvency!(fv_vault_pre);

    let token: u64 = nondet();
    let token_instruction_data = &token.to_le_bytes();
    process_slash(&account_infos, token_instruction_data).unwrap();

    let fv_vault_post: FvVault = vault_account.into();
    assert_solvency!(fv_vault_post);
}
