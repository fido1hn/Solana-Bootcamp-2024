use anchor_lang::prelude::*;

declare_id!("7ghGdpBu5LJP8V6HpddAjtjue8SEa9qG6T7BqaHjKxeY");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites_program {
    use super::*;

    pub fn set_favorites(ctx: Context<SetFavorites>,
            number: u64,
            color: String,
            hobbies: Vec<String>,
        ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let user_public_key = ctx.accounts.user.key();
        msg!("User {}'s favorite number is {}, favorite color is {}, and favorites hobbies are {:?}", user_public_key, number, color, hobbies);
        ctx.accounts.favorites.set_inner(Favorites {
            number, 
            color, 
            hobbies
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, 
        payer = user, 
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,
    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,
    #[max_len(50)]
    pub color: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}
