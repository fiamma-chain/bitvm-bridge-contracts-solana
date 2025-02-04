import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BtcLightClient } from "../target/types/btc_light_client";
import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";
import { createBlockHashAccount, createPeriodTargetAccount } from "./utils";

describe("BTC Light Client Tests", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.BtcLightClient as Program<BtcLightClient>;

    //start at block #717694, two  blocks before retarget
    const genesisBlock = {
        height: 717694,
        hash: Buffer.from("0000000000000000000b3dd6d6062aa8b7eb99d033fe29e507e0a0d81b5eaeed", "hex"),
        time: 1641627092,
        target: Buffer.from("0000000000000000000B98AB0000000000000000000000000000000000000000", "hex"),
    };

    let btcLightClientState: PublicKey;

    before(async () => {
        // Get state PDA
        const [statePda] = PublicKey.findProgramAddressSync(
            [Buffer.from("btc_light_client")],
            program.programId
        );
        btcLightClientState = statePda;
    });

    it("Initialize state", async () => {
        const initialPeriod = Math.floor(genesisBlock.height / 2016);
        await program.methods
            .initialize(
                new anchor.BN(genesisBlock.height),
                new anchor.BN(initialPeriod),
                Array.from(genesisBlock.hash),
                genesisBlock.time,
                Array.from(genesisBlock.target),
                false,
            )
            .accounts({})
            .rpc();

        // Verify state
        const state = await program.account.btcLightClientState.fetch(btcLightClientState);
        expect(state.latestBlockHeight.toString()).to.equal(genesisBlock.height.toString());
        expect(state.latestBlockTime).to.equal(genesisBlock.time);
        expect(state.isTestnet).to.be.false;
    });

    it("Submit block headers", async () => {
        // Block #717695 header data,  
        // all bitcoin header values are little-endian:
        const block717695 = Buffer.from(
            "04002020" +          // 4 bytes (version)
            "edae5e1bd8a0e007e529fe33d099ebb7a82a06d6d63d0b000000000000000000" +  // 32 bytes (prev hash)
            "f8aec519bcd878c9713dc8153a72fd62e3667c5ade70d8d0415584b8528d79ca" +  // 32 bytes (merkle root)
            "0b40d961" +          // 4 bytes (time)
            "ab980b17" +          // 4 bytes (bits)
            "3dcc4d5a", "hex");   // 4 bytes (nonce)

        const block717696 = Buffer.from(
            "00004020" + // version
            "9acaa5d26d392ace656c2428c991b0a3d3d773845a1300000000000000000000" + // parent hash
            "aa8e225b1f3ea6c4b7afd5aa1cecf691a8beaa7fa1e579ce240e4a62b5ac8ecc" + // merkle root
            "2141d961" + // time
            "8b8c0b17" + // bits
            "0d5c05bb", // nonce
            "hex"
        );

        const headers = [block717695, block717696];
        const blockHeight = genesisBlock.height + 1; // 717695

        // 准备 remaining accounts
        const remainingAccounts = [];

        // 1. 添加前一个区块哈希账户 (717694)
        const [prevBlockHashPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("block_hash"), new anchor.BN(blockHeight - 1).toArrayLike(Buffer, 'le', 8)],
            program.programId
        );
        remainingAccounts.push({
            pubkey: prevBlockHashPda,
            isWritable: false,
            isSigner: false
        });

        // 2. Add current block hash accounts (717695, 717696)
        for (let i = 0; i < headers.length; i++) {
            const [blockHashPda] = PublicKey.findProgramAddressSync(
                [Buffer.from("block_hash"), new anchor.BN(blockHeight + i).toArrayLike(Buffer, 'le', 8)],
                program.programId
            );
            remainingAccounts.push({
                pubkey: blockHashPda,
                isWritable: true,
                isSigner: false
            });
        }

        // 3. Add difficulty target account
        const currentPeriod = Math.floor(blockHeight / 2016);
        const [periodTargetPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("period_target"), new anchor.BN(currentPeriod).toArrayLike(Buffer, 'le', 8)],
            program.programId
        );
        remainingAccounts.push({
            pubkey: periodTargetPda,
            isWritable: false,
            isSigner: false
        });

        await program.methods
            .submitBlockHeaders(
                new anchor.BN(blockHeight),
                Buffer.concat(headers)
            )
            .accounts({
                submitter: provider.wallet.publicKey,
            })
            .remainingAccounts(remainingAccounts)
            .rpc();

        // Verify the state after submission
        const state = await program.account.btcLightClientState.fetch(btcLightClientState);
        expect(state.latestBlockHeight.toString()).to.equal("717696");
        expect(state.latestBlockTime).to.equal(1641627937); // From block 717696
    });

    // it("Verify transaction", async () => {
    //     const blockHeight = genesisBlock.height + 1;
    //     const txProof = {
    //         blockHeader: Buffer.from("..."), // Block header data
    //         txId: Buffer.from("..."), // Transaction ID
    //         txIndex: 1,
    //         merkleProof: [], // Merkle proof
    //         rawTx: Buffer.from("..."), // Raw transaction data
    //         outputIndex: 0,
    //         expectedAmount: new anchor.BN(100000), // 1000 satoshis
    //         expectedScriptHash: Buffer.from("..."), // Script hash
    //     };

    //     // Get block hash PDA
    //     const [blockHashPda] = PublicKey.findProgramAddressSync(
    //         [Buffer.from("block_hash"), new anchor.BN(blockHeight).toBuffer('le', 8)],
    //         program.programId
    //     );

    //     await program.methods
    //         .verifyTransaction(new anchor.BN(blockHeight), txProof)
    //         .accounts({
    //             state: btcLightClientState,
    //             blockHash: blockHashPda,
    //         })
    //         .rpc();
    // });
}); 