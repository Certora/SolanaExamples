use cvlr::prelude::*;
use cvlr_solana::cvlr_deserialize_nondet_accounts;
use solana_program::account_info::{next_account_info, AccountInfo};

use crate::{assert_solvency, assume_solvency, processor::*, state::Vault};

/// Verifies that a vault account remains solvent before and after a withdrawal
/// operation.
#[rule]
pub fn rule_vault_solvency_withdraw() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let vault_account: &AccountInfo = next_account_info(account_info_iter).unwrap();

    assume_solvency!(vault_account);

    let shares: u8 = nondet();
    process_withdraw(&account_infos, &[shares as u8]).unwrap();

    assert_solvency!(vault_account);
}
