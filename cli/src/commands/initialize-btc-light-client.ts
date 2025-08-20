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

    // Genesis block parameters (mainnet)
    const genesisBlock = {
        height: 83167,
        // reverse the hash to little endian
        hash: Buffer.from(
            "00000000d5c074d945840ac55a4b46c5c763b221efbc0eacaef6cdcf6bc1e68d",
            "hex"
        ).reverse(),
        time: 1747827720,
        target: Buffer.from(
            "ffff0000000000000000000000000000000000000000000000000000",
            "hex"
        ),
    };
    // Genesis block parameters (signet)
    // const genesisBlock = {
    //     height: 252937,
    //     // reverse the hash to little endian
    //     hash: Buffer.from(
    //         "0000000b83c39066658440edb23d947b6e1cc438ee07464de58af29bd32c88f6",
    //         "hex"
    //     ).reverse(),
    //     time: 1747832093,
    //     target: Buffer.from(
    //         "12bb1d0000000000000000000000000000000000000000000000000000",
    //         "hex"
    //     ),
    // };

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
                false,  // isTestnet
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