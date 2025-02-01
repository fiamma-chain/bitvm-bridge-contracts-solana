import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BtcLightClient } from "../target/types/btc_light_client";
import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";

describe("BTC Light Client Tests", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.BtcLightClient as Program<BtcLightClient>;

    // Test data
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

    // it("Submit block headers", async () => {
    //     const headers = [
    //         // ... block header data
    //     ];
    //     const blockHeight = genesisBlock.height + 1;

    //     const remainingAccounts = [];

    //     // 1. Add previous block hash account
    //     const [prevBlockHashPda] = PublicKey.findProgramAddressSync(
    //         [Buffer.from("block_hash"), new anchor.BN(blockHeight - 1).toBuffer('le', 8)],
    //         program.programId
    //     );
    //     remainingAccounts.push({
    //         pubkey: prevBlockHashPda,
    //         isWritable: false,
    //         isSigner: false
    //     });

    //     // 2. Add current block hash accounts
    //     for (let i = 0; i < headers.length; i++) {
    //         const [blockHashPda] = PublicKey.findProgramAddressSync(
    //             [Buffer.from("block_hash"), new anchor.BN(blockHeight + i).toBuffer('le', 8)],
    //             program.programId
    //         );
    //         remainingAccounts.push({
    //             pubkey: blockHashPda,
    //             isWritable: true,
    //             isSigner: false
    //         });
    //     }

    //     // 3. Add difficulty target account
    //     const currentPeriod = Math.floor(blockHeight / 2016);
    //     const [currentPeriodTargetPda] = PublicKey.findProgramAddressSync(
    //         [Buffer.from("period_target"), new anchor.BN(currentPeriod).toBuffer('le', 8)],
    //         program.programId
    //     );
    //     remainingAccounts.push({
    //         pubkey: currentPeriodTargetPda,
    //         isWritable: false,
    //         isSigner: false
    //     });

    //     await program.methods
    //         .submitBlockHeaders(new anchor.BN(blockHeight), headers)
    //         .accounts({
    //             state: btcLightClientState,
    //             submitter: provider.wallet.publicKey,
    //             systemProgram: anchor.web3.SystemProgram.programId,
    //         })
    //         .remainingAccounts(remainingAccounts)
    //         .rpc();
    // });

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