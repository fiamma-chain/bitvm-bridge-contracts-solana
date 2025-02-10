import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BtcLightClient } from "../target/types/btc_light_client";
import { expect } from "chai";
import { PublicKey } from "@solana/web3.js";

describe("BTC Light Client Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BtcLightClient as Program<BtcLightClient>;

  //start at block #717694, two  blocks before retarget
  const genesisBlock = {
    height: 717694,
    hash: Buffer.from(
      "edae5e1bd8a0e007e529fe33d099ebb7a82a06d6d63d0b000000000000000000",
      "hex"
    ),
    time: 1641627092,
    target: Buffer.from(
      "0000000000000000000B98AB0000000000000000000000000000000000000000",
      "hex"
    ),
  };

  // Block #717695 header data,
  // all bitcoin header values are little-endian:
  const block717695 = Buffer.from(
    "04002020" + // 4 bytes (version)
      "edae5e1bd8a0e007e529fe33d099ebb7a82a06d6d63d0b000000000000000000" + // 32 bytes (prev hash)
      "f8aec519bcd878c9713dc8153a72fd62e3667c5ade70d8d0415584b8528d79ca" + // 32 bytes (merkle root)
      "0b40d961" + // 4 bytes (time)
      "ab980b17" + // 4 bytes (bits)
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
        new anchor.BN(0)
      )
      .accounts({})
      .rpc();

    // Verify state
    const state = await program.account.btcLightClientState.fetch(
      btcLightClientState
    );
    expect(state.latestBlockHeight.toString()).to.equal(
      genesisBlock.height.toString()
    );
    expect(state.latestBlockTime).to.equal(genesisBlock.time);
    expect(Buffer.from(state.latestBlockHash).toString("hex")).to.equal(
      Buffer.from(genesisBlock.hash).toString("hex")
    );
    expect(Buffer.from(state.latestPeriodTarget).toString("hex")).to.equal(
      genesisBlock.target.toString("hex")
    );
    expect(state.isTestnet).to.be.false;
  });

  it("Submit block headers", async () => {
    const headers = [block717695, block717696];
    const blockHeight = genesisBlock.height + 1; // 717695

    // create block hash accounts for each block
    let remainingAccounts = await Promise.all(
      headers.map((_, i) =>
        createBlockHashAccountIfNeeded(program, provider, blockHeight + i)
      )
    );

    // submit block headers
    await program.methods
      .submitBlockHeaders(new anchor.BN(blockHeight), Buffer.concat(headers))
      .accounts({})
      .remainingAccounts(remainingAccounts)
      .rpc();

    // Verify the state after submission
    const stateAfterSubmission =
      await program.account.btcLightClientState.fetch(btcLightClientState);
    expect(stateAfterSubmission.latestBlockHeight.toString()).to.equal(
      "717696"
    );
    expect(stateAfterSubmission.latestBlockTime).to.equal(1641627937); // From block 717696

    // Verify block hash entries

    const [block717695HashPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("block_hash_entry"),
        new anchor.BN(717695).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    const block717695Hash = await program.account.blockHashEntry.fetch(
      block717695HashPda
    );

    const expectedHash =
      "9acaa5d26d392ace656c2428c991b0a3d3d773845a1300000000000000000000";
    expect(Buffer.from(block717695Hash.hash).toString("hex")).to.equal(
      expectedHash
    );
  });

  // Test submit block headers with empty headers
  it("Submit block headers with empty headers", async () => {
    const blockHeight = genesisBlock.height + 1;
    const headers = [];
    try {
      await program.methods
        .submitBlockHeaders(new anchor.BN(blockHeight), Buffer.concat(headers))
        .accounts({})
        .rpc();
      expect.fail("should have thrown error");
    } catch (err) {
      expect(err.toString()).to.include("No headers provided.");
    }
  });

  // Test submit block headers with invalid headers
  it("Submit block headers with invalid headers", async () => {
    const blockHeight = genesisBlock.height + 1;
    const headers = [Buffer.from("invalid header")];
    try {
      await program.methods
        .submitBlockHeaders(new anchor.BN(blockHeight), Buffer.concat(headers))
        .accounts({})
        .rpc();
      expect.fail("should have thrown error");
    } catch (err) {
      expect(err.toString()).to.include("Invalid block header");
    }
  });
  // Test submit block headers with wrong parent block hash
  it("Submit block headers with wrong parent block hash", async () => {
    const blockHeight = genesisBlock.height + 3;

    const wrongPrevHashBlock = Buffer.from(
      "00004020" + // version
        "9acaa5d26d392ace656c2428c991b0a3d3d773845a1300000000000000000000" +
        "aa8e225b1f3ea6c4b7afd5aa1cecf691a8beaa7fa1e579ce240e4a62b5ac8ecc" +
        "2141d961" + // time
        "8b8c0b17" + // bits
        "0d5c05bb", // nonce
      "hex"
    );
    const headers = [wrongPrevHashBlock];
    let remainingAccounts = await Promise.all(
      headers.map((_, i) =>
        createBlockHashAccountIfNeeded(program, provider, blockHeight + i)
      )
    );

    try {
      await program.methods
        .submitBlockHeaders(new anchor.BN(blockHeight), Buffer.concat(headers))
        .accounts({})
        .remainingAccounts(remainingAccounts)
        .rpc();
      expect.fail("should have thrown error");
    } catch (err) {
      expect(err.toString()).to.include("Invalid previous block hash");
    }
  });

  // Test submit block headers hash too easy

  it("Submit block headers hash too easy", async () => {
    const blockHeight = genesisBlock.height + 3;
    const hashTooEasyBlock = Buffer.from(
      "04002020" + // 4 bytes (version)
        "bf559a5b0479c2a73627af40cef1835d44de7b32dd3503000000000000000000" + // 32 bytes (prev hash)
        "f8aec519bcd878c9713dc8153a72fd62e3667c5ade70d8d0415584b8528d79ca" + // 32 bytes (merkle root)
        "0b40d961" + // 4 bytes (time)
        "ab980b17" + // 4 bytes (bits)
        "41b360c0",
      "hex"
    );
    const headers = [hashTooEasyBlock];
    let remainingAccounts = await Promise.all(
      headers.map((_, i) =>
        createBlockHashAccountIfNeeded(program, provider, blockHeight + i)
      )
    );

    try {
      await program.methods
        .submitBlockHeaders(new anchor.BN(blockHeight), Buffer.concat(headers))
        .accounts({})
        .remainingAccounts(remainingAccounts)
        .rpc();
      expect.fail("should have thrown error");
    } catch (err) {
      expect(err.toString()).to.include("Invalid proof of work");
    }
  });
});

async function createBlockHashAccountIfNeeded(
  program: Program<BtcLightClient>,
  provider: anchor.AnchorProvider,
  currentHeight: number
): Promise<anchor.web3.AccountMeta> {
  const [blockHashPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("block_hash_entry"),
      new anchor.BN(currentHeight).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  // Check if account exists
  const accountInfo = await provider.connection.getAccountInfo(blockHashPda);
  if (!accountInfo) {
    // Create block_hash account
    const tx = await program.methods
      .createBlockHashAccount(
        new anchor.BN(currentHeight),
        Array.from(new Uint8Array(32)) // Initialize with empty hash
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
