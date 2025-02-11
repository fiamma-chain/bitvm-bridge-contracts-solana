import { Command } from "commander";
import { initializeBitvmBridge } from "./commands/initialize-bridge";
import { initializeBtcLightClient } from "./commands/initialize-btc-light-client";
import { submitHeaders } from "./commands/submit-headers";

const program = new Command();

program
  .command("initialize")
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

program.parse(process.argv);
