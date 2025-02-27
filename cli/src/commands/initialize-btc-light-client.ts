import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Connection, PublicKey } from "@solana/web3.js";
import { BtcLightClient } from "../../../target/types/btc_light_client";
import { loadWalletFromEnv, getRpcUrl } from "../utils/wallet";

export async function initializeBtcLightClient() {
    const connection = new Connection(getRpcUrl(), "confirmed");
    const wallet = new Wallet(loadWalletFromEnv());

    console.log("current wallet", wallet.publicKey.toString(),);

    // Create provider
    const provider = new anchor.AnchorProvider(connection, wallet, {});
    anchor.setProvider(provider);

    const program = anchor.workspace.BtcLightClient as Program<BtcLightClient>;

    console.log("program", program.programId.toString());

    // Genesis block parameters (testnet4)
    const genesisBlock = {
        height: 71872,
        // reverse the hash to little endian
        hash: Buffer.from(
            "0000000050f19ed6a245ca841d4fb0e069448dcd5254483645fe1947f7b80e86",
            "hex"
        ).reverse(),
        time: 1740652701,
        target: Buffer.from(
            "ffff0000000000000000000000000000000000000000000000000000",
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
                new anchor.BN(3)  // minConfirmations
            )
            .accounts({})
            .rpc();

        console.log("BTC Light Client initialized successfully");
    } catch (error) {
        console.error("Failed to initialize BTC Light Client:", error);
        throw error;
    }
} 