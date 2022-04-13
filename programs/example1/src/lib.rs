use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;
use solana_safe_math::{SafeMath};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    // account_info::{AccountInfo},
    // next_account_info,
    pubkey::Pubkey,
    entrypoint,
    // entrypoint::ProgramResult,
    msg,
    // entrypoint::ProgramResult,
    // pubkey::Pubkey,
    // program::invoke,
    program_error::ProgramError,
    system_instruction,
};
use borsh::{BorshDeserialize, BorshSerialize};

// declare_id!("4Y8PGKfY7q5hxDA17h6UHk5eACo6k9idy1chiHb7HKsp");



entrypoint!(process_instruction);

    // use super::*;
   
    pub fn process_instruction(
        program_id: &Pubkey, // Public key of the account the hello world program was loaded into
        accounts: &[AccountInfo], // The account to say hello to
        _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
    ) -> ProgramResult {
        // let user = &mut ctx.accounts.depositor;
        // // let pool = &mut ctx.accounts.vault;
        // let referrer = &mut ctx.accounts.referrer;
        let INVEST_MIN_AMOUNT:u64 = 1; 
        // Time step
        // pub mut plans = vec![];
        let PROJECT_FEE:u64 = 100;
        let PERCENT_STEP:u64 = 5;
        let PERCENTS_DIVIDER:u64 = 1000;
        let REFERRAL_PERCENTS:[u64;3]=[50, 25, 5];
        
        let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let user = next_account_info(accounts_iter)?;
    let referrer = next_account_info(accounts_iter)?;
    let data = next_account_info(accounts_iter)?;
    
        msg!("deposit function >>>>>>>>>>>>>>>>>>>>>>>>>>>>");

        // let amount : u64 = From::from(_instruction_data[0]);
        // let plan :u64 = From::from(_instruction_data[1]);

        let amount:u64 =10;
        let plan : u64 =2;
        // // anchor_lang::solana_program::program::invoke(
        //     &system_instruction::transfer(
        //         user.to_account_info().key,
        //         pool.to_account_info().key,
        //         amount,
        //     ),
        //     &[
        //         user.to_account_info(),
        //         pool.to_account_info(),
        //         ],
        //     )?;

        let mut userdata = MyAccount::try_from_slice(&data.data.borrow())?;
    // greeting_account.counter += 1;
    // greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
            let mut fee = amount.safe_mul(PROJECT_FEE)?;
            fee = fee.safe_div(PERCENTS_DIVIDER)?;

//    let abc= "5HBM1p3iLhFPoURaXVmGcCgrAyp4Mk1jG9xN6JkwfAEt";

    let mut refaccount = MyAccount {
        deposits: [0].to_vec(),
        plan : [0].to_vec(),
        percent: [20].to_vec(),
        amount : [0].to_vec(),
        profit: [0].to_vec(),
        start: [0].to_vec(),
        finish: [0].to_vec(),
        checkpoint: [1234].to_vec(),
        referrer: referrer.key(),
        levels:[0,0,0],
        bonus: 100,
        totalbonus:1000,
    };
    let mut ref2account = MyAccount {
        deposits: [0].to_vec(),
        plan : [0].to_vec(),
        percent: [20].to_vec(),
        amount : [0].to_vec(),
        profit: [0].to_vec(),
        start: [0].to_vec(),
        finish: [0].to_vec(),
        checkpoint: [1234].to_vec(),
        referrer: referrer.key(),
        levels:[0,0,0],
        bonus: 100,
        totalbonus:1000,
        };

        let mut ref3account = MyAccount{
        deposits: [0].to_vec(),
        plan : [0].to_vec(),
        percent: [20].to_vec(),
        amount : [0].to_vec(),
        profit: [0].to_vec(),
        start: [0].to_vec(),
        finish: [0].to_vec(),
        checkpoint: [1234].to_vec(),
        referrer: referrer.key(),
        levels:[0,0,0],
        bonus: 100,
        totalbonus:1000,
        };

            //(transfer sol to commissioner wallet)
            
            // if (user.referrer == 0) {
            // 	if (refaccount.deposits.len() > 0) {
            userdata.referrer = referrer.key();
            // }
            
          //  actual code for updating uplink levels
            // let upline=user.referrer;
            // for i in 1 .. 3 {
			// 	if (upline != 0) {
			// 		users[upline].levels[i] = users[upline].levels[i].safe_add(1);
			// 		upline = users[upline].referrer;
			// 	} else {break};
			// }

            let upline=userdata.referrer;
			refaccount.levels[0] = add(refaccount.levels[0], 1);
			ref2account.levels[1] = add(ref2account.levels[1], 1);
            ref3account.levels[2] = add(ref3account.levels[2], 1);

            msg!("refaccount.level {:?}>>>>>>>>>>", refaccount.levels[0]);
            msg!("ref2account.level {:?}>>>>>>>>>>", ref2account.levels[1]);
            msg!("ref2account.level {:?}>>>>>>>>>>", ref3account.levels[2]);
            
           // actual code for updating bonuses 
            // if (user.referrer != address(0)) {
            //     address upline = user.referrer;
            //     for i in 1 .. 3 {
            //         if (upline != address(0)) {
            //             let mut refamount = amount.safe_mul(REFERRAL_PERCENTS[i]);
            //              refamount = refamount.sef_div(PERCENTS_DIVIDER);
            //             users[upline].bonus = users[upline].bonus.safe_add(refamount);
            //             users[upline].totalBonus = users[upline].totalBonus.add(refamount);
            //             // emit RefBonus(upline, amount, i, refamount);
            //             upline = users[upline].referrer;
            //         } else {break};
            //     }

                        let mut refamount = mul(amount, REFERRAL_PERCENTS[0]);
                         refamount = div(refamount, PERCENTS_DIVIDER);
                        refaccount.bonus = refaccount.bonus + refamount;
                        refaccount.totalbonus = add(refaccount.totalbonus, refamount);
                        msg!("refaccount.totalbonus {}>>>>>>>>>>>>>>>>>>>>>>>>>",refaccount.totalbonus);

                        let mut refamount = mul(amount, REFERRAL_PERCENTS[1]);
                         refamount = div(refamount, PERCENTS_DIVIDER);
                        ref2account.bonus = ref2account.bonus + refamount;
                        ref2account.totalbonus = add(ref2account.totalbonus, refamount);
                        msg!("ref2account.totalbonus {}>>>>>>>>>>>>>>>>>>>>>>>>>",ref2account.totalbonus);

                        let mut refamount = mul(amount, REFERRAL_PERCENTS[2]);
                        refamount = div(refamount, PERCENTS_DIVIDER);
                        ref3account.bonus = ref3account.bonus + refamount;
                        ref3account.totalbonus = add(ref3account.totalbonus, refamount);
                        msg!("ref3account.totalbonus {}>>>>>>>>>>>>>>>>>>>>>>>>>",ref3account.totalbonus);
                        
            // if (user.deposits.len() == 0) {
            //     user.checkpoint = 1234;
            //     // emit Newbie(msg.sender);
            // }

          //  user.deposits.push(Deposit(plan, percent, amount, profit, block.timestamp, finish));
            userdata.deposits.push(amount);
            userdata.plan.push(plan);
            userdata.profit.push(10);
            userdata.checkpoint.push(0);
            msg!("users.deposits {:?}>>>>>>>>>>>>>>", userdata.deposits);
            msg!("users.plan {:?}>>>>>>>>>>>>>>", userdata.plan);
            msg!("users.profit {:?}>>>>>>>>>>>>>>", userdata.profit);
            msg!("users.checkpoint {:?}>>>>>>>>>>>>>>", userdata.checkpoint);
           // user.finish.push(finish);

            let plan = 2;
            // let x = getResult(plan, amount);
                    Ok(())
    }

    
    // pub fn withdraw(ctx: Context<WithdrawCTX>)-> ProgramResult {
    //     msg!("Withdraw function >>>>>>>>>>>>>>>>>>>>");
    //     Ok(())
    // }
   

pub fn add(a:u64, b:u64) -> u64 {
    return a+b;
}

pub fn sub(a:u64, b:u64) -> u64 {
    return if a>b {a-b} else {b-a};
}

pub fn mul(a:u64, b:u64) -> u64 {
    if a == 0 || b== 0 {return 0} else {return a*b};
}
    
pub fn div(a:u64, b:u64) -> u64 {
        if b == 0 {return 0} else {return a/b};
}
            
pub fn getResult(plan:u64, amount:u64) -> (u64, u64, u64) {
    let percent = 0; //getPercent(plan); safe_mul(percent); 
    //let plans = [];
    let mut x: getdata = getdata{
        percent: 0,
        profit:0,
        finish:0
    };

    if (plan < 3) {
        x.profit = amount * percent;
        x.profit = x.profit / 1000;
        x.profit=x.profit * 3;
        } else if (plan < 6) {
            for i in 1 .. 3 {
                 let mut adder = amount + x.profit;
                adder = adder * percent;
                adder = adder * 1000;
                x.profit = x.profit + adder;
                    }
                }
                x.finish = 1234;
                return (x.percent, x.profit, x.finish);
}     
pub struct getdata {
    percent : u64,
    profit: u64,
    finish: u64 
     }

#[derive(BorshSerialize, BorshDeserialize, Debug)]

     pub struct data {
          amount: u64
        }

// #[account]
// pub struct Plan {
//     time : u64,
//     percent : u16
// }

// // #[derive()]
// #[account]
// pub struct Referer {

//     user : Pubkey
// }

// #[derive(Accounts)]
// pub struct DepositCTX<'info> {
//     #[account(mut)]
//     depositor: Account<'info, MyAccount>,
//     #[account(mut)]
// pub referrer:Account<'info, Referer>,
//     // #[account()]
//     // pub vault: Account<'info, Vault>,
// }
// #[derive(Accounts)]
// pub struct WithdrawCTX<'info> {
//     #[account(mut)]
//     withdrawer: Account<'info, MyAccount>,
//     // #[account()]
//     // pub vault: Account<'info, Vault>,
// }
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MyAccount {
    deposits: Vec<u64>,
    plan:Vec<u64>,
    percent:Vec<u64>,
    amount:Vec<u64>,
    profit:Vec<u64>,
    start:Vec<u64>,
    finish:Vec<u64>,
    checkpoint: Vec<u64>,
    referrer: Pubkey,
    levels:[u64;3],
    bonus: u64,
    totalbonus:u64
} 

// #[account]
// pub struct Deposit {
//         plan:u64,
//         percent:u64,
//         amount:u64,
//         profit:u64,
//         start:u64,
//         finish:u64
// }

// #[account]
// pub struct DepositAccount {
//     pub count: u64,
// }

// #[account]
// pub struct Vault {
//     pub bump: u64,
//     pub payer: Pubkey,
//     pub mint_token: Pubkey, 
//     pub vault_token: Pubkey,
//     pub vault_mint: Pubkey,
// }

// #[derive(Accounts)]
// #[instruction(pool_name: String, payment: u64)]
// pub struct PayPool<'info>{
//     #[account(mut)]
//         ///// CHECK: xyz

//     pub pool: Account<'info, Pool>,
//     /// CHECK: xyz
//     #[account(mut)]
//     pub vault: Signer<'info>,
// }

// #[account]
// pub struct Pool {
//     pub pool_owner: Pubkey,
//     pub bump: u8,
//     pub capacity: u16,
//     pub name: String,
//     pub payers: Vec<Pubkey>,
// }