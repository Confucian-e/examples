import * as anchor from "@coral-xyz/anchor";
import {
    LAMPORTS_PER_SOL,
    PublicKey,
} from "@solana/web3.js";
import * as assert from "assert";
import { Attacker } from "../target/types/attacker";
import type { Faucet } from "../target/types/faucet";
import type { Vault } from "../target/types/vault";

describe("Faucet Test", () => {
    // Set the provider to the local cluster
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const vaultProgram = anchor.workspace.Vault as anchor.Program<Vault>;
    const faucetProgram = anchor.workspace.Faucet as anchor.Program<Faucet>;
    const attackerProgram = anchor.workspace.Attacker as anchor.Program<Attacker>;

    const wallet = provider.wallet as anchor.Wallet;
    console.log("Wallet Pubkey:", wallet.publicKey);

    const vaultEscrow = getEscrowPda(vaultProgram.programId);
    const faucetEscrow = getEscrowPda(faucetProgram.programId);

    const [registry] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("registry"),
        ],
        vaultProgram.programId
    );

    const [faucetSigner] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("signer"),
        ],
        faucetProgram.programId
    );

    before(async () => {
        const latestBlockHash = await provider.connection.getLatestBlockhash();

        const amount = 10 * LAMPORTS_PER_SOL;
        // airdrop SOL to the wallet account
        const wallet_sig = await provider.connection.requestAirdrop(wallet.publicKey, amount);
        console.log("Airdrop SOL to the wallet", wallet_sig);
        await provider.connection.confirmTransaction({
            signature: wallet_sig,
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        });

        // airdrop SOL to vault's escrow account
        const vault_sig = await provider.connection.requestAirdrop(vaultEscrow, amount);
        console.log("Airdrop SOL to the vault escrow", vault_sig);
        await provider.connection.confirmTransaction({
            signature: vault_sig,
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        });
    });

    it("Initialize", async () => {
        const ix_vault = await vaultProgram.methods.initialize()
            .accounts({
                escrow: vaultEscrow,
                registry: registry,
                signer: wallet.publicKey,
            })
            .instruction();

        const ix_faucet = await faucetProgram.methods.initialize()
            .accounts({
                escrow: faucetEscrow,
                signerPda: faucetSigner,
                payer: wallet.publicKey,
            })
            .instruction();

        const tx = new anchor.web3.Transaction();
        tx.add(ix_vault, ix_faucet);
        await anchor.web3.sendAndConfirmTransaction(
            provider.connection,
            tx,
            [wallet.payer, wallet.payer],
        );
    });

    it("Add Whitelist", async () => {
        await vaultProgram.methods.addWhitelist(faucetProgram.programId)
            .accounts({
                registry: registry,
                signer: wallet.publicKey,
            })
            .rpc();
    });

    it("Request", async () => {
        const faucetEscrowBalanceBefore = await provider.connection.getBalance(faucetEscrow);

        const amount = new anchor.BN(1 * LAMPORTS_PER_SOL);
        await faucetProgram.methods.request(amount)
            .accounts({
                registry: registry,
                vaultEscrow: vaultEscrow,
                escrow: faucetEscrow,
                signerPda: faucetSigner,
                payer: wallet.publicKey,
                faucetProgram: faucetProgram.programId,
                vaultProgram: vaultProgram.programId,
            })
            .rpc();

        const faucetEscrowBalanceAfter = await provider.connection.getBalance(faucetEscrow);
        assert.equal(faucetEscrowBalanceAfter - faucetEscrowBalanceBefore, amount.toNumber());
    });

    it("Attack", async () => {
        const amount = new anchor.BN(1 * LAMPORTS_PER_SOL);

        await assert.rejects(
            attackerProgram.methods.request(amount)
                .accounts({
                    registry: registry,
                    vaultEscrow: vaultEscrow,
                    escrow: faucetEscrow,
                    signerPda: faucetSigner,
                    payer: wallet.publicKey,
                    faucetProgram: faucetProgram.programId,
                    vaultProgram: vaultProgram.programId,
                })
                .rpc(),
            /Cross-program invocation with unauthorized signer or writable account/
        );
    });
});

function getEscrowPda(programId: PublicKey) {
    const [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("escrow"),
        ],
        programId
    );
    return escrow;
}