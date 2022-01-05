use anchor_spl::token::Token;
use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, TokenAccount, Mint},
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod stake {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>, 
        manager: Pubkey, 
        staker: Pubkey, 
        bump: u8,
    ) -> ProgramResult {
        let my_account = &mut ctx.accounts.my_account;
        my_account.manager = manager;
        my_account.staker = staker;
        my_account.bump = bump;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> ProgramResult {
        // only staker can call
        // only if not staked

        let my_account = &mut ctx.accounts.my_account;

        // transfer NFT to contract
        token::transfer( // small transfer then big transfer
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.staker_token_account.to_account_info(),
                    to: ctx.accounts.contract_token_account.to_account_info(),
                    authority: ctx.accounts.staker.to_account_info(),
                },
            ),
            1,
        )?;

        // set to staked
        my_account.is_staked = 1;
        
        Ok(())
    }

    pub fn release(ctx: Context<Release>) -> ProgramResult {
        // only manager can call
        // only if isStaked is true

        let my_account = &mut ctx.accounts.my_account;
        let acc_key = my_account.key();
        let seeds = &[acc_key.as_ref(), b"authority".as_ref(), &[my_account.bump]];
        

        // transfer NFT back
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info().clone(),
                token::Transfer {
                    from: ctx.accounts.contract_token_account.to_account_info(),
                    to: ctx.accounts.staker_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
                &[&seeds[..]]
            ),
            1,
        )?;

        // close account
        Ok(())
    } // no end for now?
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = user, 
        space = 8 + MyAccount::LEN
    )] // figure out space here. Is it needed?
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>, // can get the signer here. because payer update state.
    pub system_program: Program<'info, System>,
}
// do we need more about WHAT is staked?
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        mut, 
        has_one = staker, // make sure Stake.staker is Stake.myaccount.staker
        constraint = my_account.is_staked == 0 @ StakingError::AlreadyStaked, // make sure not already staked, add a msg
    )] 
    pub my_account: Account<'info, MyAccount>,

    #[account(
        seeds = [my_account.key().as_ref(), b"authority"], // what this
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    #[account(
        init, 
        seeds = [my_account.key().as_ref(), b"stake"], 
        bump, 
        token::mint = stake_mint, //which one? address? 
        token::authority = authority, // can store this way with this PDA, or use contract_token_account itself
        payer = staker
    )]
    pub contract_token_account: Account<'info, TokenAccount>,
    pub stake_mint: Box<Account<'info, Mint>>, // the point is just so we can create the right token account for the contract

    pub staker: Signer<'info>, // enforces contraint that authority account 
    #[account(mut)]
    pub staker_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    
}
// also close a buch of accounts
#[derive(Accounts)]
pub struct Release<'info> {
    #[account(
        mut, 
        has_one = manager,
        constraint = my_account.is_staked == 1,
        close = manager, // manager closes accounts to get sol back also
    )] //
    pub my_account: Account<'info, MyAccount>,

    #[account(
        seeds = [my_account.key().as_ref(), b"authority"], // what this
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    #[account(
        mut,
        close = manager,
        seeds = [my_account.key().as_ref(), b"stake"], 
        bump,
    )]
    pub contract_token_account: Account<'info, TokenAccount>,


    #[account(mut)]
    pub manager: Signer<'info>, // only manager can call
    #[account(mut)]
    pub staker_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,

}

#[account]
pub struct MyAccount {
    pub manager: Pubkey,
    pub staker: Pubkey,
    pub is_staked: u64, //bool? can also be id for exact token
    pub bump: u8,
}

impl MyAccount {
    pub const LEN: usize = 32 + 32 + 8 + 1;
}

#[error]
pub enum StakingError {
    #[msg("Already staked")]
    AlreadyStaked,
}

    
