use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;
use solana_safe_math::{SafeMath};
use solana_program::{
    account_info::{AccountInfo},
    // next_account_info,
    // entrypoint,
    // entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    // program::invoke,
    system_instruction,
};

declare_id!("4Y8PGKfY7q5hxDA17h6UHk5eACo6k9idy1chiHb7HKsp");

pub const INVEST_MIN_AMOUNT:u64 = 1; 
// Time step
// pub mut plans = vec![];
pub const PROJECT_FEE:u64 = 100;
pub const PERCENT_STEP:u64 = 5;
pub const PERCENTS_DIVIDER:u64 = 1000;
pub const REFERRAL_PERCENTS:[u64;3]=[50, 25, 5];

#[program]
mod example1 {
    use super::*;
    pub fn init(ctx :Context <Init>) -> ProgramResult {
        msg!("Initialize");
        // let plan = &mut ctx.accounts.plans;
        // let pool = &mut ctx.accounts.vault;
        Ok(())
    }
    pub fn deposit(ctx: Context<DepositCTX>, amount: u64, referrer: Pubkey) -> ProgramResult {
        let user = &mut ctx.accounts.depositor;
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
            let mut fee = amount.safe_mul(PROJECT_FEE)?;
            fee = fee.safe_div(PERCENTS_DIVIDER)?;

            //(transfer sol to commissioner wallet)
            
            // if (user.referrer == null) {
            // 	if (users[referrer].deposits.length > 0 && referrer != msg.sender) {
            		user.referrer = referrer;
            // }

            let upline=user.referrer;
            // for i in 1 .. 3 {
			// 	if (upline != address(0)) {
			// 		users[upline].levels[i] = users[upline].levels[i].safe_add(1);
			// 		upline = users[upline].referrer;
			// 	} else break;
			// }

            // if (user.referrer != address(0)) {
                // address upline = user.referrer;
                // for i in 1 .. 3 {
                    // if (upline != address(0)) {
                    //     let mut refamount = amount.safe_mul(REFERRAL_PERCENTS[i]);
                    //      refamount = refamount.sef_div(PERCENTS_DIVIDER);
                    //     users[upline].bonus = users[upline].bonus.safe_add(refamount);
                    //     users[upline].totalBonus = users[upline].totalBonus.add(refamount);
                    //     // emit RefBonus(upline, amount, i, refamount);
                    //     upline = users[upline].referrer;
                    // } else {break};
                // }
    
            // }
            // if (user.deposits.length() == 0) {
            //     user.checkpoint = block.timestamp;
            //     // emit Newbie(msg.sender);
            // }
            let plan = 2;
            let x = getResult(plan, amount);
                    Ok(())
    }
    
   
}
            
pub fn getResult(plan:u8, amount:u64) -> (u32, u32, u32) {
    let percent = 0; //getPercent(plan); safe_mul(percent); 
    
    let x: getdata = getdata{
        percent: 2,
        profit:4,
        finish:6
    };

    // if (plan < 3) {
        // let mut profit = deposit.safe_mul(percent);
        // profit = profit.safe_div(PERCENTS_DIVIDER);
        // profit=profit.safe_mul(plans[plan].time);
        // } else if (plan < 6) {
            // for i in 1 .. plans[plan].time {
                //  let mut adder = (deposit.safe_add(profit));
                // adder = adder.safe_mul(percent);
                // adder = adder.safe_div(PERCENTS_DIVIDER);
                //         // profit = profit.safe_add(adder);
                //     // }
                // }
                
                // let finish = block.timestamp.add(plans[plan].time.mul(TIME_STEP));
                return (x.percent, x.profit, x.finish);
}     
pub struct getdata {
    percent : u32,
    profit: u32,
    finish: u32 
     }
#[derive(Accounts)]
pub struct Init<'info> {
#[account(init, payer=user, space=264)]
pub users: Account<'info, Mapping>,
#[account(init, payer=user, space=264)]
pub vault: Account<'info, Vault>,
#[account(mut)]
pub user:Signer<'info>,
pub system_program: Program<'info, System>
}

#[account]
pub struct Plan {
    time : u64,
    percent : u16
}

#[derive()]
#[account]
pub struct Mapping {

    user : Vec<MyAccount>
}

#[derive(Accounts)]
pub struct DepositCTX<'info> {
    #[account(mut)]
    depositor: Account<'info, MyAccount>,
    #[account()]
    pub vault: Account<'info, Vault>,
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