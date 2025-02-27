import { Command } from "commander";
import { initializeBitvmBridge } from "./commands/initialize-bridge";
import { initializeBtcLightClient } from "./commands/initialize-btc-light-client";
import { submitHeaders } from "./commands/submit-headers";
import { updateSkipTxVerification } from "./commands/update-skip-tx-verification";

const program = new Command();

program
  .command("initialize-bridge")
  .description("Initialize BitVM Bridge program")
  .action(async () => {
    try {
      await initializeBitvmBridge();
    } catch (error) {
      console.error("Error:", error);
      process.exit(1);
    }
  });

program
  .command("initialize-btc-light-client")
  .description("Initialize BTC Light Client program")
  .action(async () => {
    try {
      await initializeBtcLightClient();
    } catch (error) {
      console.error("Error:", error);
      process.exit(1);
    }
  });

program
  .command("submit-headers")
  .description("Submit Bitcoin block headers")
  .option("-d, --daemon", "Run in daemon mode")
  .action(async (options) => {
    try {
      await submitHeaders(options);
    } catch (error) {
      console.error("Error:", error);
      if (!options.daemon) {
        process.exit(1);
      }
    }
  });

program
  .command("update-skip-tx-verification")
  .description("Update skip tx verification")
  .action(async () => {
    await updateSkipTxVerification();
  });

program.parse(process.argv);
