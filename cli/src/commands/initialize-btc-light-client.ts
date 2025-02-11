import { Program, Wallet } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Connection, PublicKey } from "@solana/web3.js";
import { BtcLightClient } from "../../../target/types/btc_light_client";
import { loadWalletFromEnv, getRpcUrl } from "../utils/wallet";

export async function initializeBtcLightClient() {
    const connection = new Connection(getRpcUrl());
    const wallet = new Wallet(loadWalletFromEnv());

    console.log("current wallet", wallet.publicKey.toString());

    // Create provider
    const provider = new anchor.AnchorProvider(connection, wallet, {});
    anchor.setProvider(provider);

    const program = anchor.workspace.BtcLightClient as Program<BtcLightClient>;

    // Genesis block parameters (testnet)
    const genesisBlock = {
        height: 230627,
        hash: Buffer.from(
            "35c40037f72c8a014c431212ad9d7452682243e5fa5de4bc4548550ac2000000",
            "hex"
        ),
        time: 1736757020,
        target: Buffer.from(
            "0003400100000000000000000000000000000000000000000000000000000000",
            "hex"
        ),
    };

    console.log("Initializing BTC Light Client with genesis block:", {
        height: genesisBlock.height,
        hash: genesisBlock.hash.toString('hex'),
        time: genesisBlock.time,
        target: genesisBlock.target.toString('hex')
    });

    try {
        await program.methods
            .initialize(
                new anchor.BN(genesisBlock.height),
                Array.from(genesisBlock.hash),
                genesisBlock.time,
                Array.from(genesisBlock.target),
                true,  // isTestnet
                new anchor.BN(1)  // minConfirmations
            )
            .accounts({})
            .rpc();

        console.log("BTC Light Client initialized successfully");
    } catch (error) {
        console.error("Failed to initialize BTC Light Client:", error);
        throw error;
    }
} 