use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::UnixTimestamp;
use anchor_lang::solana_program::instruction::Instruction;
use std::convert::Into;

declare_id!("A4f4ZDWEPLXnY8jGhTY5JfZZG6CTRBi5ikgtpQ2pbv8g");

#[interface]
pub trait Scheduled<'info, T: Accounts<'info>> {
    fn run_scheduled(ctx: Context<T>) -> ProgramResult;
}

#[program]
pub mod sol_cron {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct RegisterTask {}

#[derive(Accounts)]
pub struct RunTask {}

#[account]
pub struct Task {
    // Target program to execute against.
    program_id: Pubkey,

    // Accounts required for the task.
    accounts: Vec<TaskAccount>,

    // Instruction data for the task.
    data: Vec<u8>,

    min_interval: UnixTimestamp,

    last_executed: UnixTimestamp,
}

impl From<&Task> for Instruction {
    fn from(tx: &Task) -> Instruction {
        Instruction {
            program_id: tx.program_id,
            accounts: tx.accounts.clone().into_iter().map(Into::into).collect(),
            data: tx.data.clone(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TaskAccount {
    pubkey: Pubkey,
    is_signer: bool,
    is_writable: bool,
}

impl From<TaskAccount> for AccountMeta {
    fn from(account: TaskAccount) -> AccountMeta {
        match account.is_writable {
            false => AccountMeta::new_readonly(account.pubkey, account.is_signer),
            true => AccountMeta::new(account.pubkey, account.is_signer),
        }
    }
}
