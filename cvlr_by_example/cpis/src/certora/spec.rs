//! This module contains the specification for the vault application.

use crate::{get_account_balance, get_account_space, processor::*};
use cvlr::prelude::*;
use cvlr_solana::cvlr_deserialize_nondet_accounts;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    pubkey::Pubkey,
};

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

#[rule]
pub fn rule_create_account_creates() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let payer: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let new_account: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let owner: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let lamports: u64 = nondet();
    let space: u64 = nondet();
    let mut token_instruction_data = Vec::new();
    token_instruction_data.extend_from_slice(&lamports.to_le_bytes());
    token_instruction_data.extend_from_slice(&space.to_le_bytes());

    let payer_account_balance_pre = get_account_balance!(payer);

    process_create_account(&owner.key, &account_infos, &token_instruction_data).unwrap();

    let payer_account_balance_post = get_account_balance!(payer);
    let new_account_balance_post = get_account_balance!(new_account);
    let new_account_space = get_account_space!(new_account);

    cvlr_assert!(payer_account_balance_post == payer_account_balance_pre - lamports);
    cvlr_assert!(new_account_balance_post == lamports);
    cvlr_assert!(new_account_space == space as usize);
}
