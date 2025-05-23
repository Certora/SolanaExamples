use solana_program::{
    account_info::{next_account_info, AccountInfo},
    instruction::{AccountMeta, Instruction},
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::{self, SystemInstruction},
    system_program,
};

pub fn process_transfer(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let account_info_iter = &mut accounts.iter();
    let from_account = next_account_info(account_info_iter)?;
    let to_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let transfer_amount = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );

    invoke_transfer_2(from_account, to_account, system_program, transfer_amount)?;

    Ok(())
}

#[cfg(not(feature = "mockcpis"))]
fn invoke_transfer<'a>(
    from_account: &AccountInfo<'a>,
    to_account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    transfer_amount: u64,
) -> Result<(), ProgramError> {
    if system_program.key != &system_program::ID {
        return Err(solana_program::program_error::ProgramError::InvalidArgument);
    }
    invoke(
        &system_instruction::transfer(from_account.key, to_account.key, transfer_amount),
        &[
            from_account.clone(),
            to_account.clone(),
            system_program.clone(),
        ],
    )?;
    Ok(())
}

#[cfg(not(feature = "mockcpis"))]
fn invoke_transfer_1<'a>(
    from_account: &AccountInfo<'a>,
    to_account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    transfer_amount: u64,
) -> Result<(), ProgramError> {
    use solana_program::{
        instruction::{AccountMeta, Instruction},
        system_instruction::SystemInstruction,
    };

    if system_program.key != &system_program::ID {
        return Err(solana_program::program_error::ProgramError::InvalidArgument);
    }
    let account_metas = vec![
        AccountMeta::new(*from_account.key, true),
        AccountMeta::new(*to_account.key, false),
    ];
    let inst = {
        Instruction::new_with_bincode(
            system_program::id(),
            &SystemInstruction::Transfer {
                lamports: transfer_amount,
            },
            account_metas,
        )
    };
    let inst_ref = &inst;
    let acc0 = from_account.clone();
    let acc1 = to_account.clone();
    let acc2 = system_program.clone();
    let accounts = [acc0, acc1, acc2];
    let accounts_ref = &accounts;
    invoke(inst_ref, accounts_ref)?;
    Ok(())
}

#[cfg(not(feature = "mockcpis"))]
fn invoke_transfer_2<'a>(
    from_account: &AccountInfo<'a>,
    to_account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    transfer_amount: u64,
) -> Result<(), ProgramError> {
    use cvlr::{cvlr_assume, nondet};
    use solana_program::{
        instruction::{AccountMeta, Instruction},
        system_instruction::SystemInstruction,
    };

    if system_program.key != &system_program::ID {
        return Err(solana_program::program_error::ProgramError::InvalidArgument);
    }
    let account_metas = vec![];
    let inst = {
        Instruction::new_with_bincode(
            spl_token::id(),
            // system_program::id(),
            &SystemInstruction::Transfer { lamports: 42 },
            account_metas,
        )
    };
    // cvlr_assume!(inst.program_id == system_program::id());
    // let inst = Instruction {
    //     program_id: system_program::id(),
    //     accounts: vec![],
    //     data: vec![],
    // };
    let inst_ref = &inst;
    // let acc0 = from_account.clone();
    // let acc1 = to_account.clone();
    // let acc2 = system_program.clone();
    let accounts = [];
    let accounts_ref = &accounts;
    if nondet::<bool>() {
        // cvlr_assume!(inst_ref.program_id == system_program::id());
        print_id(&spl_token::id());
    } else {
        invoke(inst_ref, accounts_ref)?;
    }
    Ok(())
}
#[inline(never)]
fn print_id(id: &Pubkey) {
    println!("ID: {}", id);
}

#[inline(never)]
fn get_inst(account_metas: Vec<AccountMeta>) -> Instruction {
    Instruction::new_with_bincode(
        system_program::id(),
        &SystemInstruction::Transfer { lamports: 42 },
        account_metas,
    )
}

#[cfg(feature = "mockcpis")]
fn invoke_transfer<'a>(
    from_account: &AccountInfo<'a>,
    to_account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    transfer_amount: u64,
) -> Result<(), ProgramError> {
    if system_program.key != &system_program::ID {
        return Err(solana_program::program_error::ProgramError::InvalidArgument);
    }
    **from_account.try_borrow_mut_lamports()? -= transfer_amount;
    **to_account.try_borrow_mut_lamports()? += transfer_amount;
    Ok(())
}

pub fn process_create_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let owner = program_id;
    let payer = &accounts[0];
    let new_account = &accounts[1];

    let lamports = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );
    let space = u64::from_le_bytes(
        instruction_data[8..16]
            .try_into()
            .expect("Invalid slice length"),
    );

    invoke_create_account(owner, payer, new_account, lamports, space)?;
    Ok(())
}

#[cfg(not(feature = "mockcpis"))]
fn invoke_create_account<'a>(
    owner: &Pubkey,
    payer: &AccountInfo<'a>,
    new_account: &AccountInfo<'a>,
    lamports: u64,
    space: u64,
) -> Result<(), ProgramError> {
    invoke(
        &system_instruction::create_account(payer.key, new_account.key, lamports, space, owner),
        &[payer.clone(), new_account.clone()],
    )?;
    Ok(())
}

#[cfg(feature = "mockcpis")]
fn invoke_create_account<'a>(
    _owner: &Pubkey,
    payer: &AccountInfo<'a>,
    new_account: &AccountInfo<'a>,
    lamports: u64,
    space: u64,
) -> Result<(), ProgramError> {
    **payer.try_borrow_mut_lamports()? -= lamports;
    **new_account.try_borrow_mut_lamports()? = lamports;
    cvlr::cvlr_assume!(new_account.try_borrow_mut_data()?.len() == space as usize);
    Ok(())
}

pub fn process_transfer_token(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let token_program = &accounts[0];
    let from = &accounts[1];
    let mint = &accounts[2];
    let to = &accounts[3];
    let authority = &accounts[4];
    let amount = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );
    let decimals = u8::from_le_bytes(
        instruction_data[8..9]
            .try_into()
            .expect("Invalid slice length"),
    );
    invoke_transfer_token(token_program, from, mint, to, authority, amount, decimals)?;
    Ok(())
}

#[cfg(not(feature = "mockcpis"))]
fn invoke_transfer_token<'a>(
    token_program: &AccountInfo<'a>,
    from: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> Result<(), ProgramError> {
    let instruction = my_transfer_checked(
        token_program.key,
        from.key,
        mint.key,
        to.key,
        authority.key,
        &[],
        amount,
        decimals,
    )?;
    // let account_infos = [from.clone(), mint.clone(), to.clone(), authority.clone()];
    invoke(&instruction, &[])?;
    Ok(())
}

#[inline(never)]
#[cfg_attr(feature = "certora", cvlr::early_panic)]
pub fn my_transfer_checked(
    token_program_id: &Pubkey,
    source_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    destination_pubkey: &Pubkey,
    authority_pubkey: &Pubkey,
    signer_pubkeys: &[&Pubkey],
    amount: u64,
    decimals: u8,
) -> Result<Instruction, ProgramError> {
    spl_token::check_program_account(token_program_id)?;
    let data = spl_token::instruction::TokenInstruction::TransferChecked { amount: 666, decimals }.pack();

    // let accounts = vec![];
    let mut accounts = Vec::with_capacity(4 + signer_pubkeys.len());
    accounts.push(AccountMeta::new(*source_pubkey, false));
    // accounts.push(AccountMeta::new_readonly(*mint_pubkey, false));
    // accounts.push(AccountMeta::new(*destination_pubkey, false));
    // accounts.push(AccountMeta::new_readonly(
    //     *authority_pubkey,
    //     signer_pubkeys.is_empty(),
    // ));
    // for signer_pubkey in signer_pubkeys.iter() {
    //     accounts.push(AccountMeta::new_readonly(**signer_pubkey, true));
    // }

    Ok(Instruction {
        program_id: *token_program_id,
        accounts,
        data,
    })
}

#[cfg(feature = "mockcpis")]
fn invoke_transfer_token<'a>(
    token_program: &AccountInfo<'a>,
    from: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> Result<(), ProgramError> {
    cvlr_solana::token::spl_token_transfer(from, to, authority, amount).unwrap();
    let _instruction = spl_token::instruction::transfer_checked(
        token_program.key,
        from.key,
        mint.key,
        to.key,
        authority.key,
        &[],
        amount,
        decimals,
    )?;
    Ok(())
}

pub fn process_transfer_token_2022(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let token_program = &accounts[0];
    let from = &accounts[1];
    let mint = &accounts[2];
    let to = &accounts[3];
    let authority = &accounts[4];
    let amount = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );
    let decimals = u8::from_le_bytes(
        instruction_data[8..9]
            .try_into()
            .expect("Invalid slice length"),
    );
    invoke_transfer_token_2022(token_program, from, mint, to, authority, amount, decimals)?;
    Ok(())
}

#[cfg(not(feature = "mockcpis"))]
fn invoke_transfer_token_2022<'a>(
    token_program: &AccountInfo<'a>,
    from: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> Result<(), ProgramError> {
    let instruction = spl_token_2022::instruction::transfer_checked(
        token_program.key,
        from.key,
        mint.key,
        to.key,
        authority.key,
        &[],
        amount,
        decimals,
    )?;
    invoke(
        &instruction,
        &[from.clone(), mint.clone(), to.clone(), authority.clone()],
    )?;
    Ok(())
}

#[cfg(feature = "mockcpis")]
fn invoke_transfer_token_2022<'a>(
    token_program: &AccountInfo<'a>,
    from: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> Result<(), ProgramError> {
    cvlr_solana::token::spl_token_transfer(from, to, authority, amount).unwrap();
    let _instruction = spl_token_2022::instruction::transfer_checked(
        token_program.key,
        from.key,
        mint.key,
        to.key,
        authority.key,
        &[],
        amount,
        decimals,
    )?;
    Ok(())
}

pub fn process_mint_token(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let token_program = &accounts[0];
    let mint = &accounts[1];
    let destination = &accounts[2];
    let mint_authority = &accounts[3];

    let amount = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );
    let decimals = u8::from_le_bytes(
        instruction_data[8..9]
            .try_into()
            .expect("Invalid slice length"),
    );

    invoke_mint_token(
        token_program,
        mint,
        destination,
        mint_authority,
        amount,
        decimals,
    )?;
    Ok(())
}

#[cfg(not(feature = "mockcpis"))]
fn invoke_mint_token<'a>(
    token_program: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    mint_authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> Result<(), ProgramError> {
    let instruction = spl_token::instruction::mint_to_checked(
        token_program.key,
        mint.key,
        destination.key,
        mint_authority.key,
        &[],
        amount,
        decimals,
    )?;
    invoke(
        &instruction,
        &[mint.clone(), destination.clone(), mint_authority.clone()],
    )?;
    Ok(())
}

#[cfg(feature = "mockcpis")]
fn invoke_mint_token<'a>(
    token_program: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    mint_authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> Result<(), ProgramError> {
    cvlr_solana::token::spl_mint_to(mint, destination, mint_authority, amount).unwrap();
    let _instruction = spl_token::instruction::mint_to_checked(
        token_program.key,
        mint.key,
        destination.key,
        mint_authority.key,
        &[],
        amount,
        decimals,
    )?;
    Ok(())
}

pub fn process_mint_token_2022(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let token_program = &accounts[0];
    let mint = &accounts[1];
    let destination = &accounts[2];
    let mint_authority = &accounts[3];

    let amount = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );
    let decimals = u8::from_le_bytes(
        instruction_data[8..9]
            .try_into()
            .expect("Invalid slice length"),
    );

    invoke_mint_token_2022(
        token_program,
        mint,
        destination,
        mint_authority,
        amount,
        decimals,
    )?;
    Ok(())
}

#[cfg(not(feature = "mockcpis"))]
fn invoke_mint_token_2022<'a>(
    token_program: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    mint_authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> Result<(), ProgramError> {
    let instruction = spl_token_2022::instruction::mint_to_checked(
        token_program.key,
        mint.key,
        destination.key,
        mint_authority.key,
        &[],
        amount,
        decimals,
    )?;
    invoke(
        &instruction,
        &[mint.clone(), destination.clone(), mint_authority.clone()],
    )?;
    Ok(())
}

#[cfg(feature = "mockcpis")]
fn invoke_mint_token_2022<'a>(
    token_program: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    destination: &AccountInfo<'a>,
    mint_authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> Result<(), ProgramError> {
    cvlr_solana::token::spl_mint_to(mint, destination, mint_authority, amount).unwrap();
    let _instruction = spl_token_2022::instruction::mint_to_checked(
        token_program.key,
        mint.key,
        destination.key,
        mint_authority.key,
        &[],
        amount,
        decimals,
    )?;
    Ok(())
}

pub fn process_burn_token(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let token_program = &accounts[0];
    let mint = &accounts[1];
    let source = &accounts[2];
    let authority = &accounts[3];

    let amount = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );
    let decimals = u8::from_le_bytes(
        instruction_data[8..9]
            .try_into()
            .expect("Invalid slice length"),
    );

    invoke_burn_token(token_program, mint, source, authority, amount, decimals)?;
    Ok(())
}

#[cfg(not(feature = "mockcpis"))]
fn invoke_burn_token<'a>(
    token_program: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    source: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> Result<(), ProgramError> {
    let instruction = spl_token::instruction::burn_checked(
        token_program.key,
        source.key,
        mint.key,
        authority.key,
        &[],
        amount,
        decimals,
    )?;
    invoke(
        &instruction,
        &[source.clone(), mint.clone(), authority.clone()],
    )?;
    Ok(())
}

#[cfg(feature = "mockcpis")]
fn invoke_burn_token<'a>(
    token_program: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    source: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> Result<(), ProgramError> {
    cvlr_solana::token::spl_burn(mint, source, authority, amount).unwrap();
    let _instruction = spl_token::instruction::burn_checked(
        token_program.key,
        source.key,
        mint.key,
        authority.key,
        &[],
        amount,
        decimals,
    )?;
    Ok(())
}

pub fn process_burn_token_2022(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let token_program = &accounts[0];
    let mint = &accounts[1];
    let destination = &accounts[2];
    let mint_authority = &accounts[3];

    let amount = u64::from_le_bytes(
        instruction_data[..8]
            .try_into()
            .expect("Invalid slice length"),
    );
    let decimals = u8::from_le_bytes(
        instruction_data[8..9]
            .try_into()
            .expect("Invalid slice length"),
    );

    invoke_burn_token_2022(
        token_program,
        mint,
        destination,
        mint_authority,
        amount,
        decimals,
    )?;
    Ok(())
}

#[cfg(not(feature = "mockcpis"))]
fn invoke_burn_token_2022<'a>(
    token_program: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    source: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> Result<(), ProgramError> {
    let instruction = spl_token_2022::instruction::burn_checked(
        token_program.key,
        source.key,
        mint.key,
        authority.key,
        &[],
        amount,
        decimals,
    )?;
    invoke(
        &instruction,
        &[source.clone(), mint.clone(), authority.clone()],
    )?;
    Ok(())
}

#[cfg(feature = "mockcpis")]
fn invoke_burn_token_2022<'a>(
    token_program: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    source: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    amount: u64,
    decimals: u8,
) -> Result<(), ProgramError> {
    cvlr_solana::token::spl_burn(mint, source, authority, amount).unwrap();
    let _instruction = spl_token_2022::instruction::burn_checked(
        token_program.key,
        source.key,
        mint.key,
        authority.key,
        &[],
        amount,
        decimals,
    )?;
    Ok(())
}
