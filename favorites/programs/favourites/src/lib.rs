use anchor_lang::prelude::*;

declare_id!("6CWFZERMLpKbZJpXYJbrsoW5Bqs8MzTd7cL4ggxUKuz3");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favourites {
    use super::*;

    pub fn set_favourites(
        ctx: Context<SetFavourites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        let user_public_key = ctx.accounts.user.key(); // Initialize the user's public key

        msg!("Greetings from {}", ctx.program_id);
        msg!(
            "User {}'s favourite number is {}, favourite color is {}, and their hobbies are {:?}",
            user_public_key,
            number,
            color,
            hobbies
        );

        // Check the color length manually
        if color.len() > 50 {
            return Err(ErrorCode::ColorTooLong.into());
        }

        // Manually limit the number of hobbies
        if hobbies.len() > 5 {
            return Err(ErrorCode::TooManyHobbies.into());
        }

        let favourites: &mut Account<Favourites> = &mut ctx.accounts.favourites;
        favourites.number = number;
        favourites.color = color;
        favourites.hobbies = hobbies;

        Ok(())
    }
}

#[account]
pub struct Favourites {
    pub number: u64,
    pub color: String,
    pub hobbies: Vec<String>,
}

// Define custom error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Color length exceeds 50 characters.")]
    ColorTooLong,

    #[msg("Too many hobbies.")]
    TooManyHobbies,
}

#[derive(Accounts)]
pub struct SetFavourites<'info> {
    #[account(
        init_if_needed, 
        payer = user, 
        space = ANCHOR_DISCRIMINATOR_SIZE + Favourites::space(),
        seeds = [b"favourites", user.key().as_ref()],
        bump
    )]
    pub favourites: Account<'info, Favourites>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl Favourites {
    pub const fn space() -> usize {
        let color_size = 50; // Max length for the color string
        let hobbies_size = 5 * 50; // Max 5 hobbies, each max length 50
        let vec_length_size = 4; // Size of the vector length (u32)

        // Return total space: discriminator + number + color + hobbies length + hobbies data
        8 + // Anchor discriminator
        8 + // u64 for number
        color_size + // color string
        vec_length_size + // length of hobbies vector (u32 for vector size)
        hobbies_size // hobbies vector (max 5 hobbies of 50 chars each)
    }
}
