import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BitvmBridgeContractsSolana } from "../target/types/bitvm_bridge_contracts_solana";

describe("bitvm-bridge-contracts-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BitvmBridgeContractsSolana as Program<BitvmBridgeContractsSolana>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
