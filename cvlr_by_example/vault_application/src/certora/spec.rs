//! This module contains the specification for the vault application.

use crate::{assert_solvency, assume_solvency, fv_vault_from_acc_info, processor::*, state::Vault};
use cvlr::{mathint::NativeInt, prelude::*};
use cvlr_solana::cvlr_deserialize_nondet_accounts;
use solana_program::account_info::{next_account_info, AccountInfo};

/// Structure tracking the state for the formal verification (FV) of the vault.
/// Uses unbounded-size mathematical integers instead of integers with fixed
/// size.
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

impl FvVault {
    /// Verifies that a vault account remains solvent before and after a withdrawal
    /// operation.
    #[rule]
    pub fn rule_vault_solvency_withdraw() {
        let account_infos = cvlr_deserialize_nondet_accounts();
        let account_info_iter = &mut account_infos.iter();
        let vault_account: &AccountInfo = next_account_info(account_info_iter).unwrap();

        let fv_vault_pre: FvVault = fv_vault_from_acc_info!(vault_account);
        assume_solvency!(fv_vault_pre);

        let shares: u64 = nondet();
        let shares_instruction_data = &shares.to_le_bytes();
        process_withdraw(&account_infos, shares_instruction_data).unwrap();

        let fv_vault_post: FvVault = fv_vault_from_acc_info!(vault_account);
        assert_solvency!(fv_vault_post);
    }
}
