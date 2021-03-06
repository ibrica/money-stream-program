//! Money stream program, paying over time (transfer tokens from one account to another)
//! Following escrow approach from
//! https://hackmd.io/@ironaddicteddog/anchor_example_escrow and
//! https://github.com/project-serum/anchor/tree/master/tests/escrow
//! 
//!
//! User (Initializer I) starts a token stream with Taker(T):
//! - SPL tokens will be sent to a vault from I
//! - Initializer will send ticks over time and increment steps
//! - T can check the balance of the stream
//! - I can cancel stream, vault is closed and tokens flow to the T, rest amount is returned to T
//! - T can withdraw tokens, stream is ended in same way like with cancel


use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

declare_id!("5xCzmy6UvLQJX7GBckN4q5EaC4ypsLhAhM9jhdfKXBR1");

#[program]
pub mod money_stream_program {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"streaming";

    pub fn initialize_stream(
        ctx: Context<InitializeStream>,
        _vault_account_bump: u8,
        limit: u64,
        step: u64,
        rate: u64
    ) -> ProgramResult {
        if limit < step { // Not enough money
            return Err(ErrorCode::LimitLow.into());
        }
        ctx.accounts.stream_account.initializer_key = *ctx.accounts.initializer.key;
        ctx.accounts
            .stream_account
            .initializer_token_account = *ctx
            .accounts
            .initializer_token_account
            .to_account_info()
            .key;

        ctx.accounts.stream_account.limit = limit;
        ctx.accounts.stream_account.step = step;
        ctx.accounts.stream_account.rate = rate;

        let (vault_authority, _vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);

        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            ctx.accounts.stream_account.limit,
        )?;

        Ok(())
    }

    pub fn cancel_stream(ctx: Context<CancelStream>) -> ProgramResult {
        let (_vault_authority, vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let authority_seeds = &[&ESCROW_PDA_SEED[..], &[vault_authority_bump]];

        // Calculate amount for taker (rate * step)
        let amount = ctx.accounts.stream_account.rate * ctx.accounts.stream_account.step;
        // Return rest to the initializer
        let rest = ctx.accounts.stream_account.limit - amount;
        
        // Send to taker
        token::transfer(
            ctx.accounts
                .into_transfer_to_taker_context()
                .with_signer(&[&authority_seeds[..]]),
            amount,
        )?;


        // Send back
        token::transfer(
            ctx.accounts
                .into_transfer_to_initializer_context()
                .with_signer(&[&authority_seeds[..]]),
            rest,
        )?;

        token::close_account(
            ctx.accounts
                .into_close_context()
                .with_signer(&[&authority_seeds[..]]),
        )?;

        Ok(())
    }

    // Basicaly the same as cancel just from takers side, leaving it apart for now, maybe will change the flow
    pub fn withdraw(ctx: Context<Withdraw>) -> ProgramResult {
        // Transferring from initializer to taker
        let (_vault_authority, vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let authority_seeds = &[&ESCROW_PDA_SEED[..], &[vault_authority_bump]];

       // Calculate amount for taker (rate * step)
       let amount = ctx.accounts.stream_account.rate * ctx.accounts.stream_account.step;
       // Return rest to the initializer
       let rest = ctx.accounts.stream_account.limit - amount;
       
       // Send to taker
       token::transfer(
           ctx.accounts
               .into_transfer_to_taker_context()
               .with_signer(&[&authority_seeds[..]]),
           amount,
       )?;


       // Send back
       token::transfer(
           ctx.accounts
               .into_transfer_to_initializer_context()
               .with_signer(&[&authority_seeds[..]]),
           rest,
       )?;

       token::close_account(
           ctx.accounts
               .into_close_context()
               .with_signer(&[&authority_seeds[..]]),
       )?;

       Ok(())
    }

    // Another moment passed, increment step (more money to taker)
    pub fn tick(ctx: Context<Tick>) -> ProgramResult {
        ctx.accounts.stream_account.step = ctx.accounts.stream_account.step + 1;
        Ok(())
    }

    // Check the balance from taker side
    pub fn balance(_ctx: Context<Balance>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(vault_account_bump: u8, initializer_amount: u64)]
pub struct InitializeStream<'info> {
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [b"token-seed".as_ref()],
        bump = vault_account_bump,
        payer = initializer,
        token::mint = mint,
        token::authority = initializer,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(
        mut,
    )]
    pub initializer_token_account: Account<'info, TokenAccount>,
    #[account(zero)]
    pub stream_account: ProgramAccount<'info, StreamAccount>,
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Balance<'info> {
    #[account(signer)]
    pub taker: AccountInfo<'info>,
    #[account(mut)]
    pub initializer: AccountInfo<'info>,
    #[account(
        constraint = stream_account.initializer_key == *initializer.key
    )]
    pub stream_account: ProgramAccount<'info, StreamAccount>,
}

#[derive(Accounts)]
pub struct Tick<'info> {
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    #[account(
        mut,
        constraint = stream_account.initializer_key == *initializer.key
    )]
    pub stream_account: ProgramAccount<'info, StreamAccount>,
}

#[derive(Accounts)]
pub struct CancelStream<'info> {
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    pub vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub taker_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = stream_account.initializer_key == *initializer.key,
        constraint = stream_account.initializer_token_account == *initializer_token_account.to_account_info().key,
        close = initializer
    )]
    pub stream_account: ProgramAccount<'info, StreamAccount>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(signer)]
    pub taker: AccountInfo<'info>,
    #[account(mut)]
    pub initializer: AccountInfo<'info>,
    #[account(mut)]
    pub taker_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub initializer_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = (stream_account.step * stream_account.rate)  <= stream_account.limit,
        constraint = stream_account.initializer_token_account == *initializer_token_account.to_account_info().key,
        constraint = stream_account.initializer_key == *initializer.key,
        close = initializer
    )]
    pub stream_account: ProgramAccount<'info, StreamAccount>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    pub vault_authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[account]
pub struct StreamAccount {
    pub initializer_key: Pubkey,
    pub initializer_token_account: Pubkey,
    pub limit: u64,
    pub step: u64,
    pub rate: u64
}

impl<'info> InitializeStream<'info> {
    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .initializer_token_account
                .to_account_info()
                .clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.initializer.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.initializer.clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

impl<'info> CancelStream<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self
                .initializer_token_account
                .to_account_info()
                .clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_transfer_to_taker_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self.taker_token_account.to_account_info().clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.vault_account.to_account_info().clone(),
            destination: self.initializer.clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

impl<'info> Withdraw<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self
                .initializer_token_account
                .to_account_info()
                .clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_transfer_to_taker_context(
        &self
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self.taker_token_account.to_account_info().clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

    fn into_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.vault_account.to_account_info().clone(),
            destination: self.initializer.clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

#[error]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Limit lower then one step.")]
    LimitLow,
}

