#[macro_export]
macro_rules! get_account_balance {
    ($account:expr) => {{
        **$account.try_borrow_mut_lamports().unwrap()
    }};
}

#[macro_export]
macro_rules! get_account_space {
    ($account:expr) => {{
        $account.try_borrow_data().unwrap().len()
    }};
}
