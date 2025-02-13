import { Keypair } from "@solana/web3.js";
import * as bs58 from "bs58";
import dotenv from "dotenv";
import { resolve } from "path";

dotenv.config({ path: resolve(__dirname, "../../../.env") });

export function loadWalletFromEnv(): Keypair {
    const privateKey = process.env.SOLANA_PRIVATE_KEY;
    if (!privateKey) throw new Error("SOLANA_PRIVATE_KEY not found in .env");
    return Keypair.fromSecretKey(bs58.decode(privateKey));
}

export function loadSubmitterWalletFromEnv(): Keypair {
    const privateKey = process.env.SUBMITTER_PRIVATE_KEY;
    if (!privateKey) throw new Error("SUBMITTER_PRIVATE_KEY not found in .env");
    return Keypair.fromSecretKey(bs58.decode(privateKey));
}

export function getRpcUrl(): string {
    const rpcUrl = process.env.SOLANA_RPC_URL;
    if (!rpcUrl) throw new Error("SOLANA_RPC_URL not found in .env");
    return rpcUrl;
} 