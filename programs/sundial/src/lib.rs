use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use std::convert::Into;

declare_id!("D8Eqftpam9F6nygDLVudKfiPZU4XNLnj8QmoeX4iYMVP");

#[interface]
pub trait Scheduled<'info, T: Accounts<'info>> {
    fn run_scheduled(ctx: Context<T>) -> ProgramResult;
}

#[program]
pub mod sundial {
    use super::*;

    pub fn register_task(
        ctx: Context<RegisterTask>,
        bump: u8,
        task_program: Pubkey,
        start_ts: i64,
        interval_ts: i64,
    ) -> ProgramResult {
        ctx.accounts.task_specification.task_program = task_program;
        // TODO: validate timestamps
        ctx.accounts.task_specification.start_ts = start_ts;
        ctx.accounts.task_specification.interval_ts = interval_ts;
        ctx.accounts.task_specification.bump = bump;
        Ok(())
    }

    // pub fn deregister_task(ctx: Context<DeregisterTask>) -> ProgramResult {
    //     Ok(())
    // }

    #[access_control(on_schedule(&ctx))]
    pub fn run_task<'info>(ctx: Context<'_, '_, '_, 'info, RunTask<'info>>) -> ProgramResult {
        let cpi_program = ctx.accounts.task_program.clone();
        let cpi_accounts = ctx.remaining_accounts.to_vec();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        scheduled::run_scheduled(cpi_ctx)?;

        let spec = &mut ctx.accounts.task_specification;
        spec.last_executed_ts = ctx.accounts.clock.unix_timestamp;

        Ok(())
    }
}

pub fn on_schedule<'info>(ctx: &Context<'_, '_, '_, 'info, RunTask<'info>>) -> Result<()> {
    let spec = &ctx.accounts.task_specification;
    let current_ts: i64 = ctx.accounts.clock.unix_timestamp;
    if current_ts < spec.start_ts {
        msg!(&format!(
            "current ts: \"{}\" start ts: \"{}\"",
            current_ts, spec.start_ts
        ));
        return Err(ErrorCode::RanEarly.into());
    }
    if spec.last_executed_ts != i64::default() {
        if current_ts < (spec.last_executed_ts + spec.interval_ts) {
            return Err(ErrorCode::RanOffSchedule.into());
        }
    }
    Ok(())
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct RegisterTask<'info> {
    task_program: AccountInfo<'info>,

    #[account(init, seeds = [task_program.key.as_ref()], bump = bump, payer = registrar)]
    task_specification: Account<'info, TaskSpecification>,

    #[account(signer)]
    registrar: AccountInfo<'info>,

    system_program: Program<'info, System>,
}

// All accounts not included here, i.e., the "remaining accounts" should be task args
#[derive(Accounts)]
pub struct RunTask<'info> {
    pub task_program: AccountInfo<'info>,

    #[account(mut, seeds = [task_program.key.as_ref()], bump = task_specification.bump, has_one = task_program)]
    pub task_specification: Account<'info, TaskSpecification>,

    // fee account (PDA?)
    pub clock: Sysvar<'info, Clock>,
}

#[account]
#[derive(Default)]
pub struct TaskSpecification {
    // Target program to execute
    task_program: Pubkey,

    // Schedule info
    start_ts: i64,
    interval_ts: i64,
    last_executed_ts: i64,

    // For PDA
    bump: u8,
    // TODO: resolver info
}

#[error]
pub enum ErrorCode {
    #[msg("Ran before start time in spec")]
    RanEarly,
    #[msg("Ran off schedule")]
    RanOffSchedule,
}
