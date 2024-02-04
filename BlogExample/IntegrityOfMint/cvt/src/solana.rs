/* Stubs for solana_program */

use {
    solana_program::{
        account_info::AccountInfo,
        pubkey::Pubkey
    },
    crate::{
        nondet::{
            Nondet, nondet_impl,
        },
        stubs:: { solana_stubs }
    }
};

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_account_info() -> AccountInfo<'static> {
    solana_stubs::CVT_nondet_account_info_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_pubkey() -> Pubkey {
    solana_stubs::CVT_nondet_pubkey_impl()
}

nondet_impl! {Pubkey, CVT_nondet_pubkey(), "Nondet for Pubkey" }
nondet_impl! {AccountInfo<'static>, CVT_nondet_account_info(), "Nondet for AccountInfo" }

// We redefine this macro to avoid including error code conversion/formatting
#[macro_export]
macro_rules! require_keys_eq {
    ($value1: expr, $value2: expr, $error_code:expr $(,)?) => {
        cvt::CVT_assume($value1 == $value2);
    };
    ($value1: expr, $value2: expr $(,)?) => {
        cvt::CVT_assume($value1 == $value2);
    };
}