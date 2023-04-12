use anchor_lang::prelude::*;

declare_id!("6DefpFdPkTfKUzjZrxN2kcsFfFv37DKimxdzAePhvp1S");

#[program]
pub mod simple_storage {
    use super::*;

    pub fn store_details(
        ctx: Context<StoreDetails>,
        name: String,
        age: u8,
        address: String,
    ) -> Result<()> {
        require!(
            name.len() > 0 && address.len() > 0,
            StorageError::EmptyDetails
        );
        require!(
            name.len() <= UserDetails::MAX_NAME_LEN
                && address.len() <= UserDetails::MAX_ADDRESS_LEN,
            StorageError::MaxLengthExceeded
        );
        ctx.accounts.details.set_inner(UserDetails {
            name,
            age,
            address,
            identity: ctx.accounts.user.key(),
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StoreDetails<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub user: SystemAccount<'info>,
    #[account(init, payer = payer, space = UserDetails::LEN,
              seeds = [b"details", user.key().as_ref()], bump)]
    pub details: Account<'info, UserDetails>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserDetails {
    pub name: String,
    pub age: u8,
    pub address: String,
    pub identity: Pubkey,
}

impl UserDetails {
    pub const MAX_NAME_LEN: usize = 20;
    pub const MAX_ADDRESS_LEN: usize = 50;
    pub const LEN: usize = 8 + 4 + Self::MAX_NAME_LEN + 1 + 4 + Self::MAX_ADDRESS_LEN + 32;
}

#[error_code]
pub enum StorageError {
    #[msg("Name and address can not be empty strings")]
    EmptyDetails,
    #[msg("Name or address exceeds the maximum allowed length")]
    MaxLengthExceeded,
}
