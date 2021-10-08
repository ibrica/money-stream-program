use anchor_lang::prelude::*;

declare_id!("2m2pMvmHSkHnvc8VS492WeoxMqnyou3mdPCQxvocDXPx");

#[program]
pub mod money_stream_program {
    use super::*;
    pub fn save_funds(ctx: Context<Save>, amount: u64) -> ProgramResult {
        let save_account = &mut ctx.accounts.save_account;
        save_account.amount = amount;
        Ok(())
    }
    
    pub fn transfer_funds(ctx: Context<Transfer>) -> ProgramResult {
        let base_account = &mut ctx.accounts.save_account;
        base_account.amount = 0;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Save<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub save_account: Account<'info, SaveAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Transaction instructions
#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, has_one = authority)]
    pub save_account: Account<'info, SaveAccount>,
    pub authority: Signer<'info>,
}

// An account that goes inside a transaction instruction
#[account]
pub struct SaveAccount {
    pub amount: u64,
    pub authority: Pubkey
}

#[error]
pub enum ErrorCode {
    #[msg(&quot;You are not authorized to perform this action.&quot;)]
    Unauthorized,
}
