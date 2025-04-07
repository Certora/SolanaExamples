//! This module contains the specification for the vault application.

use crate::{get_account_balance, get_account_space, processor::*};
use cvlr::prelude::*;
use cvlr_solana::{
    cvlr_deserialize_nondet_accounts,
    token::{spl_mint_get_supply, spl_token_account_get_amount},
};
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

#[rule]
pub fn rule_transfer_token_transfers_different_wallets() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let token_program: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let from: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _mint: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let to: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _authority: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let amount: u64 = nondet();
    let decimals: u8 = nondet();
    let mut token_instruction_data = Vec::new();
    token_instruction_data.extend_from_slice(&amount.to_le_bytes());
    token_instruction_data.extend_from_slice(&decimals.to_le_bytes());

    cvlr_assume!(from.key != to.key);

    let from_wallet_amount_pre = spl_token_account_get_amount(from);
    let to_wallet_amount_pre = spl_token_account_get_amount(to);

    process_transfer_token(&account_infos, &token_instruction_data).unwrap();

    let from_wallet_amount_post = spl_token_account_get_amount(from);
    let to_wallet_amount_post = spl_token_account_get_amount(to);

    cvlr_assert!(*token_program.key == spl_token::id());
    cvlr_assert!(from_wallet_amount_post == from_wallet_amount_pre - amount);
    cvlr_assert!(to_wallet_amount_post == to_wallet_amount_pre + amount);
}

#[rule]
pub fn rule_transfer_token_transfers_same_wallet() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let token_program: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let from: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _mint: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let to: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _authority: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let amount: u64 = nondet();
    let decimals: u8 = nondet();
    let mut token_instruction_data = Vec::new();
    token_instruction_data.extend_from_slice(&amount.to_le_bytes());
    token_instruction_data.extend_from_slice(&decimals.to_le_bytes());

    let from_wallet_amount_pre = spl_token_account_get_amount(from);
    let to_wallet_amount_pre = spl_token_account_get_amount(to);

    // Assume from and to are the same account.
    cvlr_assume!(from.key == to.key);
    cvlr_assume!(from_wallet_amount_pre == to_wallet_amount_pre);

    process_transfer_token(&account_infos, &token_instruction_data).unwrap();

    let from_wallet_amount_post = spl_token_account_get_amount(from);
    let to_wallet_amount_post = spl_token_account_get_amount(to);

    cvlr_assert!(*token_program.key == spl_token::id());
    cvlr_assert!(from_wallet_amount_post == from_wallet_amount_pre);
    cvlr_assert!(to_wallet_amount_post == to_wallet_amount_pre);
}

#[rule]
pub fn rule_transfer_token_2022_transfers_different_wallets() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let token_program: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let from: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _mint: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let to: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _authority: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let amount: u64 = nondet();
    let decimals: u8 = nondet();
    let mut token_instruction_data = Vec::new();
    token_instruction_data.extend_from_slice(&amount.to_le_bytes());
    token_instruction_data.extend_from_slice(&decimals.to_le_bytes());

    cvlr_assume!(from.key != to.key);

    let from_wallet_amount_pre = spl_token_account_get_amount(from);
    let to_wallet_amount_pre = spl_token_account_get_amount(to);

    process_transfer_token_2022(&account_infos, &token_instruction_data).unwrap();

    let from_wallet_amount_post = spl_token_account_get_amount(from);
    let to_wallet_amount_post = spl_token_account_get_amount(to);

    cvlr_assert!(
        *token_program.key == spl_token_2022::id() || *token_program.key == spl_token::id()
    );
    cvlr_assert!(from_wallet_amount_post == from_wallet_amount_pre - amount);
    cvlr_assert!(to_wallet_amount_post == to_wallet_amount_pre + amount);
}

#[rule]
pub fn rule_transfer_token_2022_transfers_same_wallet() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let token_program: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let from: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _mint: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let to: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _authority: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let amount: u64 = nondet();
    let decimals: u8 = nondet();
    let mut token_instruction_data = Vec::new();
    token_instruction_data.extend_from_slice(&amount.to_le_bytes());
    token_instruction_data.extend_from_slice(&decimals.to_le_bytes());

    let from_wallet_amount_pre = spl_token_account_get_amount(from);
    let to_wallet_amount_pre = spl_token_account_get_amount(to);

    // Assume from and to are the same account.
    cvlr_assume!(from.key == to.key);
    cvlr_assume!(from_wallet_amount_pre == to_wallet_amount_pre);

    process_transfer_token_2022(&account_infos, &token_instruction_data).unwrap();

    let from_wallet_amount_post = spl_token_account_get_amount(from);
    let to_wallet_amount_post = spl_token_account_get_amount(to);

    cvlr_assert!(
        *token_program.key == spl_token_2022::id() || *token_program.key == spl_token::id()
    );
    cvlr_assert!(from_wallet_amount_post == from_wallet_amount_pre);
    cvlr_assert!(to_wallet_amount_post == to_wallet_amount_pre);
}

#[rule]
pub fn rule_mint_token_mints() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let token_program: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let mint: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let destination: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _mint_authority: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let amount: u64 = nondet();
    let decimals: u8 = nondet();
    let mut token_instruction_data = Vec::new();
    token_instruction_data.extend_from_slice(&amount.to_le_bytes());
    token_instruction_data.extend_from_slice(&decimals.to_le_bytes());

    let destination_wallet_amount_pre = spl_token_account_get_amount(destination);
    let mint_supply_pre = spl_mint_get_supply(mint);

    process_mint_token(&account_infos, &token_instruction_data).unwrap();

    let destination_wallet_amount_post = spl_token_account_get_amount(destination);
    let mint_supply_post = spl_mint_get_supply(mint);

    cvlr_assert!(*token_program.key == spl_token::id());
    cvlr_assert!(destination_wallet_amount_post == destination_wallet_amount_pre + amount);
    cvlr_assert!(mint_supply_post == mint_supply_pre + amount);
}

#[rule]
pub fn rule_mint_token_2022_mints() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let token_program: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let mint: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let destination: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _mint_authority: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let amount: u64 = nondet();
    let decimals: u8 = nondet();
    let mut token_instruction_data = Vec::new();
    token_instruction_data.extend_from_slice(&amount.to_le_bytes());
    token_instruction_data.extend_from_slice(&decimals.to_le_bytes());

    let destination_wallet_amount_pre = spl_token_account_get_amount(destination);
    let mint_supply_pre = spl_mint_get_supply(mint);

    process_mint_token_2022(&account_infos, &token_instruction_data).unwrap();

    let destination_wallet_amount_post = spl_token_account_get_amount(destination);
    let mint_supply_post = spl_mint_get_supply(mint);

    cvlr_assert!(
        *token_program.key == spl_token_2022::id() || *token_program.key == spl_token::id()
    );
    cvlr_assert!(destination_wallet_amount_post == destination_wallet_amount_pre + amount);
    cvlr_assert!(mint_supply_post == mint_supply_pre + amount);
}

#[rule]
pub fn rule_burn_token_burns() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let token_program: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let mint: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let source: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _mint_authority: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let amount: u64 = nondet();
    let decimals: u8 = nondet();
    let mut token_instruction_data = Vec::new();
    token_instruction_data.extend_from_slice(&amount.to_le_bytes());
    token_instruction_data.extend_from_slice(&decimals.to_le_bytes());

    let source_wallet_amount_pre = spl_token_account_get_amount(source);
    let mint_supply_pre = spl_mint_get_supply(mint);

    process_burn_token(&account_infos, &token_instruction_data).unwrap();

    let source_wallet_amount_post = spl_token_account_get_amount(source);
    let mint_supply_post = spl_mint_get_supply(mint);

    cvlr_assert!(*token_program.key == spl_token::id());
    cvlr_assert!(source_wallet_amount_post == source_wallet_amount_pre - amount);
    cvlr_assert!(mint_supply_post == mint_supply_pre - amount);
}

#[rule]
pub fn rule_burn_token_2022_burns() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();
    let token_program: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let mint: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let source: &AccountInfo = next_account_info(account_info_iter).unwrap();
    let _mint_authority: &AccountInfo = next_account_info(account_info_iter).unwrap();

    let amount: u64 = nondet();
    let decimals: u8 = nondet();
    let mut token_instruction_data = Vec::new();
    token_instruction_data.extend_from_slice(&amount.to_le_bytes());
    token_instruction_data.extend_from_slice(&decimals.to_le_bytes());

    let source_wallet_amount_pre = spl_token_account_get_amount(source);
    let mint_supply_pre = spl_mint_get_supply(mint);

    process_burn_token_2022(&account_infos, &token_instruction_data).unwrap();

    let source_wallet_amount_post = spl_token_account_get_amount(source);
    let mint_supply_post = spl_mint_get_supply(mint);

    cvlr_assert!(
        *token_program.key == spl_token_2022::id() || *token_program.key == spl_token::id()
    );
    cvlr_assert!(source_wallet_amount_post == source_wallet_amount_pre - amount);
    cvlr_assert!(mint_supply_post == mint_supply_pre - amount);
}
