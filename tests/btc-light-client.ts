import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BtcLightClient } from "../target/types/btc_light_client";
import { BitcoinUtils } from "./utils";
import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";


describe("BTC Light Client Tests", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.BtcLightClient as Program<BtcLightClient>;

    //start at block #717694, two  blocks before retarget
    const genesisBlock = {
        height: 717694,
        hash: Buffer.from("edae5e1bd8a0e007e529fe33d099ebb7a82a06d6d63d0b000000000000000000", "hex"),
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
        await program.methods
            .initialize(
                new anchor.BN(genesisBlock.height),
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
            "3dcc4d5a",
            "hex"
        );

        const block717696 = Buffer.from(
            "00004020" + // version
            "9acaa5d26d392ace656c2428c991b0a3d3d773845a1300000000000000000000" +
            "aa8e225b1f3ea6c4b7afd5aa1cecf691a8beaa7fa1e579ce240e4a62b5ac8ecc" +
            "2141d961" + // time
            "8b8c0b17" + // bits
            "0d5c05bb", // nonce
            "hex"
        );

        const headers = [block717695, block717696];
        const blockHeight = genesisBlock.height + 1; // 717695
        // 准备 remaining accounts
        const remainingAccounts = [];

        // 为每个区块创建 block_hash 账户
        for (let i = 0; i < headers.length; i++) {
            const currentHeight = blockHeight + i;
            const [blockHashPda] = PublicKey.findProgramAddressSync(
                [Buffer.from("block_hash_entry"), new anchor.BN(currentHeight).toArrayLike(Buffer, 'le', 8)],
                program.programId
            );

            // 检查账户是否存在
            const accountInfo = await provider.connection.getAccountInfo(blockHashPda);
            if (!accountInfo) {
                // 创建 block_hash 账户
                await program.methods
                    .createBlockHashAccount(
                        new anchor.BN(currentHeight),
                        Array.from(new Uint8Array(32)) // 初始化为空的哈希
                    )
                    .accounts({
                    })
                    .rpc();
            }

            remainingAccounts.push({
                pubkey: blockHashPda,
                isWritable: true,
                isSigner: false
            });
        }

        //提交区块头
        await program.methods
            .submitBlockHeaders(
                new anchor.BN(blockHeight),
                Buffer.concat(headers)
            )
            .accounts({
            })
            .remainingAccounts(remainingAccounts)
            .rpc();

        // Verify the state after submission
        const stateAfterSubmission = await program.account.btcLightClientState.fetch(btcLightClientState);
        expect(stateAfterSubmission.latestBlockHeight.toString()).to.equal("717696");
        expect(stateAfterSubmission.latestBlockTime).to.equal(1641627937); // From block 717696

        // Verify block hash entries

        const [block717695HashPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("block_hash_entry"), new anchor.BN(717695).toArrayLike(Buffer, 'le', 8)],
            program.programId
        );

        const block717695Hash = await program.account.blockHashEntry.fetch(block717695HashPda);

        const expectedHash = "9acaa5d26d392ace656c2428c991b0a3d3d773845a1300000000000000000000";
        expect(Buffer.from(block717695Hash.hash).toString('hex')).to.equal(expectedHash);
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