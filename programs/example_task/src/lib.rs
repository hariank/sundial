use anchor_lang::prelude::*;
use std::convert::Into;

use solcron::Scheduled;

declare_id!("5aKgwdNbhhRfxTinWwEw8FKA81Mg1e79CJUjCMzt2EnY");

#[program]
pub mod example_task {
    use super::*;

    pub fn create(ctx: Context<Create>, authority: Pubkey) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.authority = authority;
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }

    #[state]
    pub struct ExampleScheduled;

//     impl<'info> Scheduled<'info, Increment<'info>> for ExampleScheduled {
//         fn run_scheduled(ctx: Context<Increment<'info>>) -> ProgramResult {
//             let counter = &mut ctx.accounts.counter;
//             counter.count += 1;
//             Ok(())
//         }
//     }

    impl<'info> Scheduled<'info, IncrementUnsafe<'info>> for ExampleScheduled {
        fn run_scheduled(ctx: Context<IncrementUnsafe<'info>>) -> ProgramResult {
            let counter = &mut ctx.accounts.counter;
            counter.count += 1;
            Ok(())
        }
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct IncrementUnsafe<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}
