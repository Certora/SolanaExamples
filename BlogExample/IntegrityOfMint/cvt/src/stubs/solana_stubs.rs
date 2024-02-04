use {
    solana_program::{
        account_info::AccountInfo,
        pubkey::Pubkey
    }
};

extern "C" {
    #[allow(improper_ctypes)]
    fn mk_account_info_unchecked() -> AccountInfo<'static>;
    #[allow(improper_ctypes)]
    fn mk_pubkey_unchecked() -> Pubkey;
}

#[allow(non_snake_case)]
pub fn CVT_nondet_account_info_impl() -> AccountInfo<'static> {
    unsafe {
        return mk_account_info_unchecked();
    }
}

#[allow(non_snake_case)]
pub fn CVT_nondet_pubkey_impl() -> Pubkey {
    unsafe {
        return mk_pubkey_unchecked();
    }
}

