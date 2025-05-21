import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { BtcLightClient } from "../../../target/types/btc_light_client";
import { Connection, PublicKey } from "@solana/web3.js";
import { JsonRpcClient } from "../utils/json-rpc";
import { getRpcUrl, loadSubmitterWalletFromEnv } from "../utils/wallet";

let isRunning = false;
let intervalId: NodeJS.Timeout;

const MAX_REORG = 50; // Maximum number of blocks to check for reorg

export async function submitHeaders(options: { daemon?: boolean } = {}) {
    if (isRunning) {
        console.log("Submitter is already running");
        return;
    }

    const run = async () => {
        try {
            isRunning = true;
            await submitHeadersOnce();
        } catch (error) {
            console.error("Error submitting headers:", error);
        } finally {
            isRunning = false;
        }
    };

    if (options.daemon) {
        // execute every minute
        intervalId = setInterval(run, 60 * 1000);
        console.log("Started submitter in daemon mode");

        // handle exit signal
        process.on('SIGINT', () => {
            console.log("Stopping submitter...");
            clearInterval(intervalId);
            process.exit(0);
        });

        // execute immediately
        await run();
    } else {
        // single execution mode
        await run();
    }
}

async function submitHeadersOnce() {
    const connection = new Connection(getRpcUrl());
    const wallet = new Wallet(loadSubmitterWalletFromEnv());

    console.log("current wallet", wallet.publicKey.toString());

    // Create provider
    const provider = new anchor.AnchorProvider(connection, wallet, {});
    anchor.setProvider(provider);

    const program = anchor.workspace.BtcLightClient as Program<BtcLightClient>;

    // get config from env
    const { env } = process;
    const bitcoinRpcUrl = env.BITCOIN_RPC_URL || "http://127.0.0.1:18443";
    const bitcoinRpcAuth = env.BITCOIN_RPC_AUTH || "test:1234";
    const maxBlocks = Number(env.MAX_BLOCKS_PER_BATCH || "10");

    // get current state
    const [statePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("btc_light_client")],
        program.programId
    );

    const state = await program.account.btcLightClientState.fetch(statePda);
    const mirrorLatestHeight = state.latestBlockHeight.toNumber();
    console.log("Current light client height:", mirrorLatestHeight);

    // connect to bitcoin node
    const rpc = new JsonRpcClient({ url: bitcoinRpcUrl, auth: bitcoinRpcAuth });
    const btcTipHeight = await getBtcBlockCount(rpc);
    console.log("Bitcoin tip height:", btcTipHeight);

    if (btcTipHeight <= mirrorLatestHeight) {
        console.log("No new blocks");
        return;
    }

    // Find common ancestor in case of reorg
    let commonHeight = mirrorLatestHeight;

    for (let height = mirrorLatestHeight; height > mirrorLatestHeight - MAX_REORG; height--) {
        const btcHash = await getBtcBlockHash(rpc, height);
        // convert to little endian, because the block hash is stored in little endian in the light client
        const btcHashLE = Buffer.from(btcHash, 'hex').reverse().toString('hex');

        // Get stored hash from light client
        const [blockHashPda] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("block_hash_entry"),
                new anchor.BN(height).toArrayLike(Buffer, "le", 8),
            ],
            program.programId
        );

        try {
            const blockHashAccount = await program.account.blockHashEntry.fetch(blockHashPda);
            const storedHash = Buffer.from(blockHashAccount.hash).toString('hex');

            if (btcHashLE === storedHash) {
                console.log(`Found common hash at height ${height}: ${btcHash} (LE: ${btcHashLE})`);
                commonHeight = height;
                break;
            }
        } catch (error) {
            // Account does not exist, which means we've found our starting point
            console.log(`No block hash account found at height ${height}, using as starting point`);
            commonHeight = height;
            break;
        }

        if (height === mirrorLatestHeight - MAX_REORG) {
            throw new Error(`No common hash found within ${MAX_REORG} blocks. Catastrophic reorg?`);
        }
    }

    // get and submit block headers
    await submitBlockHeaders(
        program,
        provider,
        rpc,
        commonHeight,  // found common ancestor
        btcTipHeight,
        maxBlocks
    );
}

async function getBtcBlockCount(rpc: JsonRpcClient): Promise<number> {
    const res = await rpc.req("getblockcount", []);
    if (res.error) throw new Error("Failed to get block count: " + JSON.stringify(res));
    return res.result as number;
}

async function submitBlockHeaders(
    program: Program<BtcLightClient>,
    provider: AnchorProvider,
    rpc: JsonRpcClient,
    currentHeight: number,
    tipHeight: number,
    maxBlocks: number
) {
    const targetHeight = Math.min(tipHeight, currentHeight + maxBlocks);
    const headers: Buffer[] = [];

    // If there is a reorg, we need to start from the common height
    for (let height = currentHeight + 1; height <= targetHeight; height++) {
        const hash = await getBtcBlockHash(rpc, height);
        const header = await getBtcBlockHeader(rpc, hash);
        headers.push(Buffer.from(header, 'hex'));
        console.log(`Fetched header at height ${height}: ${hash}`);
    }

    // create block hash accounts
    const remainingAccounts = await Promise.all(
        headers.map((_, i) =>
            createBlockHashAccountIfNeeded(program, provider, currentHeight + i + 1)
        )
    );

    // wait for create block hash accounts to be processed
    console.log("Waiting for create block hash accounts to be processed...");
    await new Promise(resolve => setTimeout(resolve, 5000));

    console.log(`Submitting ${headers.length} headers from height ${currentHeight + 1}`);

    await program.methods
        .submitBlockHeaders(new anchor.BN(currentHeight + 1), Buffer.concat(headers))
        .accounts({})
        .remainingAccounts(remainingAccounts)
        .preInstructions([
            anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
                units: 500_000,
            }),
        ])
        .rpc();

    console.log("Headers submitted successfully");
}

async function getBtcBlockHash(rpc: JsonRpcClient, height: number): Promise<string> {
    const res = await rpc.req("getblockhash", [height]);
    if (res.error) throw new Error("Failed to get block hash: " + JSON.stringify(res));
    return res.result as string;
}

async function getBtcBlockHeader(rpc: JsonRpcClient, hash: string): Promise<string> {
    const res = await rpc.req("getblockheader", [hash, false]);
    if (res.error) throw new Error("Failed to get block header: " + JSON.stringify(res));
    return res.result as string;
}

async function createBlockHashAccountIfNeeded(
    program: Program<BtcLightClient>,
    provider: AnchorProvider,
    height: number
): Promise<anchor.web3.AccountMeta> {
    const [blockHashPda] = PublicKey.findProgramAddressSync(
        [
            Buffer.from("block_hash_entry"),
            new anchor.BN(height).toArrayLike(Buffer, "le", 8),
        ],
        program.programId
    );

    const accountInfo = await provider.connection.getAccountInfo(blockHashPda);
    if (!accountInfo) {
        await program.methods
            .createBlockHashAccount(
                new anchor.BN(height),
                Array.from(new Uint8Array(32))
            )
            .accounts({})
            .rpc();
    }

    return {
        pubkey: blockHashPda,
        isWritable: true,
        isSigner: false,
    };
} 