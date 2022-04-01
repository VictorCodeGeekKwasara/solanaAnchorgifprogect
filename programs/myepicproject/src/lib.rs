use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
    use super::*;    
    pub fn start_stuff_off(ctx: Context<StartstuffOff>) -> Result<()> {
        // Get a reference to the account.
        let base_account = &mut ctx.accounts.base_account ;

        // Initialize total_gifs
        base_account.total_gifs = 0 ;
        Ok(())
    }

    // the function now accepts a gif_link param from the user. We also reference the user fro the Context
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String)-> Result<()> {
        // Get reference to the account  and  increament total_gifs.

        let base_account = &mut ctx.accounts.base_account ;
        let user = &mut ctx.accounts.user ;

        // Build the struct.
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key ,
        };

        // Add it to the gif_list vector.
        base_account.gif_list.push(item);
        base_account.total_gifs += 1 ;

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
// Specify what data you want in the Addgif Context.
// Getting a handle on the flow of things :)? 
// add a signer who calls the AddGif method tothe struct so tha we can save it

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug,Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

// Tell Solana what we want to store on this account.

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}
