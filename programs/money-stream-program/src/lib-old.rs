use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Token, Transfer};

declare_id!("2m2pMvmHSkHnvc8VS492WeoxMqnyou3mdPCQxvocDXPx");

#[program]
pub mod money_stream_program {
    use super::*;

    pub fn create(ctx: Context<Create>, authority: Pubkey, benefiter: Pubkey, limit: u64, step:u64) -> ProgramResult {
        let sesion_account = &mut ctx.accounts.session_account;
        sesion_account.authority = authority;
        sesion_account.benefiter = benefiter;
        sesion_account.limit = limit;
        sesion_account.step = step;
        sesion_account.amount = step;
        Ok(())
    }

    pub fn tick(ctx: Context<Tick>) -> ProgramResult {
        let session_account = &mut ctx.accounts.session_account;
        session_account.amount += session_account.step;
        Ok(())
    }


    pub fn balance(ctx: Context<Balance>,authority: Pubkey) -> Result<()> {
        let session_account = &ctx.accounts.session_account;
        if session_account.benefiter == authority {
            Ok(())
        } else {
            return Err(ErrorCode::Unauthorized.into());
        }
    }

    pub fn withdraw(ctx: Context<Withdraw> ) -> Result<()> {
        //let session_account = &mut ctx.accounts.session_account;
        // if session_account.benefiter != authority {
        //    return Err(ErrorCode::Unauthorized.into());
        // }
        // session_account.amount = 0;      
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 120)]
    pub session_account: Account<'info, SessionAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Tick<'info> {
    #[account(mut, has_one = authority)]
    pub session_account: Account<'info, SessionAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Balance<'info> {
    #[account(has_one = authority)]
    pub session_account: Account<'info, SessionAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub session_account: Account<'info, SessionAccount>,
    pub authority: Signer<'info>,
}

#[account]
pub struct SessionAccount {
    pub authority: Pubkey,
    pub benefiter: Pubkey,
    pub limit: u64,
    pub step: u64,
    pub amount: u64
}

#[error]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}