import { Keypair } from "@solana/web3.js";
import * as bs58 from "bs58";
import dotenv from "dotenv";
import { resolve } from "path";

dotenv.config({ path: resolve(__dirname, "../../.env") });

export function loadWalletFromEnv(): Keypair {
    const privateKey = process.env.SOLANA_PRIVATE_KEY;
    if (!privateKey) {
        throw new Error("SOLANA_PRIVATE_KEY not found in .env file");
    }
    return Keypair.fromSecretKey(bs58.decode(privateKey));
}

export function getRpcUrl(): string {
    const rpcUrl = process.env.RPC_URL;
    if (!rpcUrl) {
        throw new Error("RPC_URL not found in .env file");
    }
    return rpcUrl;
}
