use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Mint, TokenAccount, Transfer, MintTo};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    // entrypoint,
    // entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::invoke,
    system_instruction,
};

declare_id!("4Y8PGKfY7q5hxDA17h6UHk5eACo6k9idy1chiHb7HKsp");

#[program]
mod example1 {
    use super::*;
    pub fn init(ctx :Context <Init>) -> ProgramResult {
        let plan = &mut ctx.accounts.plans;
        let mut x:[Plan;6];
        // let owner:Pubkey=;
        let seed = "LazyInvestor2";
        // let pubkey= Pubkey::;
        // let vault = Pubkey::create_with_seed(base, seed, owner)?;
        Ok(())
    }
    pub fn deposit(ctx: Context<DepositCTX>, amount: u64) -> ProgramResult {
        
        let user = &ctx.accounts.pool;
        let pool = &mut ctx.accounts.vault;
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(
                user.to_account_info().key,
                pool.to_account_info().key,
                amount,
            ),
            &[
                user.to_account_info(),
                pool.to_account_info(),
            ],
        )?;
        Ok(())
    }
    // pub fn withdraw() -> ProgramResult {
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer=user, space=264)]
    pub plans: Account<'info, Plan>,
    #[account(init, payer=user, space=264)]
    pub wallet_account_mapping: Account<'info, Mapping>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program: Program<'info, System>
}
#[account]
pub struct Plan {
    time : u64,
    percent : u8
}
#[account]
pub struct Mapping {
    userWalletPubKey : Pubkey,
    userAccountPubKey : Pubkey
}

#[derive(Accounts)]
pub struct DepositCTX<'info> {
    #[account(mut)]
    depositor: Account<'info, MyAccount>,
    // #[account(mut)]
    // refferrer: Account<'info, MyAccount>,
}
#[account]
pub struct MyAccount {
    deposits: Vec<Deposit>,
    checkpoint: u128,
    referrer: Pubkey,
    levels:[u8;3],
    bonus: u32,
    totalbonus:u32
}

#[account]
pub struct Deposit {
        plan:u8,
        percent:u128,
        amount:u128,
        profit:u128,
        start:u128,
        finish:u128
}

#[account]
pub struct DepositAccount {
    pub count: u64,
}

#[account]
pub struct Vault {
    pub bump: u8,
    pub payer: Pubkey,
    pub mint_token: Pubkey, 
    pub vault_token: Pubkey,
    pub vault_mint: Pubkey,
}




#[derive(Accounts)]
#[instruction(pool_name: String, payment: u64)]
pub struct PayPool<'info>{
    #[account(mut)]
        ///// CHECK: xyz

    pub pool: Account<'info, Pool>,
    /// CHECK: xyz
    #[account(mut)]
    pub vault: Signer<'info>,
}

#[account]
pub struct Pool {
    pub pool_owner: Pubkey,
    pub bump: u8,
    pub capacity: u16,
    pub name: String,
    pub payers: Vec<Pubkey>,
}