use anchor_lang::prelude::*;

declare_id!("6qsJF44KYSjfA5jDbivqqfcsjk18hR6ZJPMdj3ZTAWvu");

pub const ESCROW_SEED: &[u8] = b"escrow";
pub const SIGNER_SEED: &[u8] = b"signer";
pub const REGISTRY_SEED: &[u8] = b"registry";

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.registry.admin = ctx.accounts.signer.key();

        Ok(())
    }

    pub fn add_whitelist(ctx: Context<AddWhitelist>, whitelist: Pubkey) -> Result<()> {
        ctx.accounts.registry.whitelist.push(whitelist);

        Ok(())
    }

    pub fn transfer_native(ctx: Context<TransferNative>, amount: u64) -> Result<()> {
        // check if the faucet program in whitelist
        let whitelist = &ctx.accounts.registry.whitelist;
        let faucet_program = ctx.accounts.faucet_program.key();
        require!(whitelist.contains(&faucet_program), VaultError::NotWhitelisted);

        ctx.accounts.from.sub_lamports(amount)?;
        ctx.accounts.to.add_lamports(amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: escrow account
    #[account(
        init,
        space = 8,
        payer = signer,
        seeds = [ESCROW_SEED],
        bump,
    )]
    pub escrow: AccountInfo<'info>,
    #[account(
        init,
        space = 8 + 32 + 4 + 32 * 3,
        payer = signer,
        seeds = [REGISTRY_SEED],
        bump,
    )]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddWhitelist<'info> {
    #[account(
        mut,
        seeds = [REGISTRY_SEED],
        bump,
    )]
    pub registry: Account<'info, Registry>,
    #[account(
        mut,
        constraint = signer.key() == registry.admin @VaultError::Unauthorized,
    )]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferNative<'info> {
    #[account(
        seeds = [REGISTRY_SEED],
        bump,
    )]
    pub registry: Account<'info, Registry>,
    /// CHECK: escrow account
    #[account(
        mut,
        seeds = [ESCROW_SEED],
        bump,
    )]
    pub from: AccountInfo<'info>,
    /// CHECK: escrow account
    #[account(
        mut,
        seeds = [ESCROW_SEED],
        bump,
        seeds::program = faucet_program,
    )]
    pub to: AccountInfo<'info>,
    #[account(
        seeds = [SIGNER_SEED],
        bump,
        seeds::program = faucet_program,
    )]
    pub signer: Signer<'info>,
    /// CHECK: faucet program
    #[account(executable)]
    pub faucet_program: AccountInfo<'info>,
}

#[account]
pub struct Registry {
    pub admin: Pubkey,
    pub whitelist: Vec<Pubkey>,
}

#[error_code]
pub enum VaultError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Not whitelisted")]
    NotWhitelisted,
}
