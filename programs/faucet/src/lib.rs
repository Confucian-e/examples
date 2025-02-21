use anchor_lang::prelude::*;
use program::Faucet;
use vault::cpi::{accounts::TransferNative, transfer_native};
use vault::{program::Vault, ESCROW_SEED, SIGNER_SEED};
use vault::{Registry, REGISTRY_SEED};

declare_id!("HPZ99LcPpWiVqKmNKkUQPwyiytkpYg5tbMY4Sv3x9cvP");

#[program]
pub mod faucet {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn request(ctx: Context<Request>, amount: u64) -> Result<()> {
        let bump = ctx.bumps.signer_pda;
        let seeds: &[&[&[u8]]] = &[&[SIGNER_SEED, &[bump]]];

        transfer_native(
            CpiContext::new_with_signer(
                ctx.accounts.vault_program.to_account_info(),
                TransferNative {
                    registry: ctx.accounts.registry.to_account_info(),
                    from: ctx.accounts.vault_escrow.to_account_info(),
                    to: ctx.accounts.escrow.to_account_info(),
                    signer: ctx.accounts.signer_pda.to_account_info(),
                    faucet_program: ctx.accounts.faucet_program.to_account_info(),
                },
                seeds,
            ),
            amount,
        )
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: escrow account
    #[account(
        init,
        space = 8,
        seeds = [ESCROW_SEED],
        bump,
        payer = payer,
    )]
    pub escrow: AccountInfo<'info>,
    /// CHECK: signer account
    #[account(
        init,
        space = 8,
        seeds = [SIGNER_SEED],
        bump,
        payer = payer,
    )]
    pub signer_pda: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Request<'info> {
    #[account(
        seeds = [REGISTRY_SEED],
        bump,
        seeds::program = vault_program,
    )]
    pub registry: Account<'info, Registry>,
    /// CHECK: escrow account
    #[account(
        mut,
        seeds = [ESCROW_SEED],
        bump,
        seeds::program = vault_program,
    )]
    pub vault_escrow: AccountInfo<'info>,
    /// CHECK: escrow account
    #[account(
        mut,
        seeds = [ESCROW_SEED],
        bump,
    )]
    pub escrow: AccountInfo<'info>,
    /// CHECK: signer account
    #[account(
        seeds = [SIGNER_SEED],
        bump,
    )]
    pub signer_pda: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub faucet_program: Program<'info, Faucet>,
    pub vault_program: Program<'info, Vault>,
}
