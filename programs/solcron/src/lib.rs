use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::UnixTimestamp;
use std::convert::Into;

declare_id!("A4f4ZDWEPLXnY8jGhTY5JfZZG6CTRBi5ikgtpQ2pbv8g");

#[interface]
pub trait Scheduled<'info, T: Accounts<'info>> {
    fn run_scheduled(ctx: Context<T>) -> ProgramResult;
}

#[program]
pub mod solcron {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
    //     Ok(())
    // }

    // pub fn register_task(ctx: Context<Initialize>) -> ProgramResult {
    //     Ok(())
    // }

    // pub fn deregister_task(ctx: Context<Initialize>) -> ProgramResult {
    //     Ok(())
    // }

    pub fn run_task(ctx: Context<RunTask>) -> ProgramResult {
        let cpi_program = ctx.accounts.task_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, Empty {});

        // with args
        // let cpi_accounts = ctx.remaining_accounts.to_vec()[1..].to_vec();
        // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // TODO: check timing

        scheduled::run_scheduled(cpi_ctx)?;

        Ok(())
    }
}

// #[derive(Accounts)]
// pub struct Initialize<'info> {}

#[derive(Accounts)]
pub struct Empty {}

#[derive(Accounts)]
pub struct RegisterTask {}

// All accounts not included here, i.e., the "remaining accounts" should be task args
#[derive(Accounts)]
pub struct RunTask<'info> {
    pub task_program: AccountInfo<'info>,
    
    // fee account (PDA?)
    // signers to transfer from account
}

// #[account]
// pub struct TaskRegistry {
//     // All registered tasks
//     accounts: Vec<Task>,
// }

// #[account]
// pub struct Task {
//     // Target program to execute against
//     program_id: Pubkey,

//     // Schedule info
//     min_interval: UnixTimestamp,
//     last_executed: UnixTimestamp,
// }