import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BtcLightClient } from "../target/types/btc_light_client";
import { expect } from "chai";
import { describe, it, before } from "node:test";
import { PublicKey } from "@solana/web3.js";
import { BankrunProvider } from "anchor-bankrun";
import { startAnchor } from "solana-bankrun";

const IDL = require("../target/idl/btc_light_client.json");
const PROGRAM_ID = new PublicKey(IDL.address);

describe("BTC TX Verify Tests", async () => {
  const context = await startAnchor(
    "",
    [{ name: "btc_light_client", programId: PROGRAM_ID }],
    []
  );
  const provider = new BankrunProvider(context);

  const program = new anchor.Program<BtcLightClient>(IDL, provider);

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

  let btcLightClientState: PublicKey;

  const [statePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("btc_light_client")],
    program.programId
  );
  btcLightClientState = statePda;

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
    Buffer.from(genesisBlock.target).toString("hex")
  );
  expect(state.isTestnet).to.be.false;

  it("Verify tx", async () => {
    // signet Block 230627 block header
    const blockHeight = 230627;
    const blockHeader = Buffer.from(
      "0000002005cd6ba37338a8c37e431180aa2d2175c39d9a6bdf3954653cb0630420000000a553f5c27318e0d7afbcf242942a0fd099683a6e6670f4a68dc8ab9ddfd7761a1ccf8467ad46011edb6c4401",
      "hex"
    );
    const txId = Buffer.from(
      "c6c911614166de26173be7c90ba37a0a26c44c3dac9bb69f84ef5b35d7525026",
      "hex"
    );

    const rawTx = Buffer.from(
      "020000000326a07fe8edcaf04e6e77508064437abf02ee0d22864849af908a3fbaaa5a992a0000000000ffffffff0ac2fb6b10869caec02d6d509fde9b2d0af3f808985ebfb623c085a0110d02840000000000ffffffffd58d5ca8fe86fd28bbb4530fa7133e2d68d263e43c453fefead6c25b42dc69ff0000000000ffffffff03a08601000000000022002085f1940c71a1e1a852db646fa0f79cf1e5defc9e4bda671ad4cf9000ada74b41881300000000000022512052d19a46c1a8cd90001a816420448b612d9c13bdb50d02d716d411deb94dc930e208000000000000225120e1382c1cb56e91bc45683199f550261b4a2da8a6db7454f3e236a4e3dfba890c00000000",
      "hex"
    );

    const txIndex = 378;
    const outputIndex = 0;
    const expectedAmount = 100000;
    const expectedScriptHash = Buffer.from(
      "85f1940c71a1e1a852db646fa0f79cf1e5defc9e4bda671ad4cf9000ada74b41",
      "hex"
    );
    const txMerkleProof = [
      Buffer.from(
        "d1263d3e754e1167d9a68b7c4ca98b245b696ecc18badea92e49c55c0729bd1a",
        "hex"
      ),
      Buffer.from(
        "20aeeae156d22ffb9a128ede8555de27c3c6bd9f47647e3de7bd4b332f8d5086",
        "hex"
      ),
      Buffer.from(
        "67e6ea2dd621b45e7aa1ff1d20977851b4694c25db628bc1359072e83fe0a2af",
        "hex"
      ),
      Buffer.from(
        "83579ca714156e1ad2832cbc4b0c1ca2d599e2cc46f6d31e967c844415c6767a",
        "hex"
      ),
      Buffer.from(
        "17fc9275ca3b6d678b77ab3677a7da37c65997893f7c10db67b5552c628f7d7f",
        "hex"
      ),
      Buffer.from(
        "50c7a085616cca1e17a766a1c581eede0139f5b0c03a8e37dfe3c2d8c2798e0a",
        "hex"
      ),
      Buffer.from(
        "239c7c1ea91e3868721ef686ca893a40b1d532d637a3fefd53df1de9aba847ab",
        "hex"
      ),
      Buffer.from(
        "0a117857964bc8182f3ddc1cdda9c71b3a081989d0aa3404be924ea3c1507671",
        "hex"
      ),
      Buffer.from(
        "23fae37f988d3dcc5cf4ed21139d9e9c3b35d68b2c1cbfaf21b67fb606cd0954",
        "hex"
      ),
      Buffer.from(
        "7a4491c685b8ae32f9ae1266a193ac335ca8f722c47af58d4e2c8283892ed091",
        "hex"
      ),
    ];
    await program.methods
      .verifyTransaction(new anchor.BN(blockHeight), {
        blockHeader: blockHeader,
        txId: Array.from(txId),
        txIndex: txIndex,
        merkleProof: txMerkleProof.map((proof) => Array.from(proof)),
        rawTx: rawTx,
        outputIndex: outputIndex,
        expectedAmount: new anchor.BN(expectedAmount),
        expectedScriptHash: Array.from(expectedScriptHash),
      })
      .accounts({})
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
          units: 1_000_000,
        }),
      ])
      .rpc();

    const [btcTxStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("tx_verified_state"), txId],
      program.programId
    );

    const txState = await program.account.txVerifiedState.fetch(btcTxStatePda);
    expect(txState.isVerified).to.be.true;
  });
});
