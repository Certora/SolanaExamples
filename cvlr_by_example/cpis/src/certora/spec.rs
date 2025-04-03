//! This module contains the specification for the vault application.

use crate::{get_account_balance, processor::*};
use cvlr::prelude::*;
use cvlr_solana::cvlr_deserialize_nondet_accounts;
use solana_program::account_info::{next_account_info, AccountInfo};

#[rule]
pub fn rule_transfer_transfers() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let from_account: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let to_account: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let system_program: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let transfer_amount: u64 = nondet();
    let token_instruction_data = &transfer_amount.to_le_bytes();

    let from_account_balance_pre = get_account_balance!(from_account);
    let to_account_balance_pre = get_account_balance!(to_account);

    process_transfer(&account_infos, token_instruction_data).unwrap();

    let from_account_balance_post = get_account_balance!(from_account);
    let to_account_balance_post = get_account_balance!(to_account);

    cvlr_assert!(*system_program.key == solana_program::system_program::id());
    cvlr_assert!(from_account_balance_post == from_account_balance_pre - transfer_amount);
    cvlr_assert!(to_account_balance_post == to_account_balance_pre + transfer_amount);
}
