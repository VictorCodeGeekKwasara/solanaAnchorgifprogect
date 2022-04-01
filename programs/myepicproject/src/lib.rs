use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
    use super::*;
    
    pub fn start_stuff_off(ctx: Context<StartstuffOff>) -> Result<()> {
        // Get a reference to the account.
        let base_account = &mut ctx.accounts.base_account ;

        // Initializs total_gifs
        base_account.total_gifs = 0 ;
        Ok(())
    }
}

// Attach certain variables to the startstuffoff context.

#[derive(Accounts)]
pub struct StartstuffOff<'info> {
    #[account(init,payer = user , space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Tell Solan what we want to store on this account.

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}
