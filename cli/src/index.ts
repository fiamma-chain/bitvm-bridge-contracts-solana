import { Command } from "commander";
import { initializeBitvmBridge } from "./commands/initialize";

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

program.parse(process.argv);
