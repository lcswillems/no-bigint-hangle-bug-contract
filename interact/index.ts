import { Command } from "commander";
import { envChain } from "xsuite/interact";
import { World } from "xsuite/world";
import data from "./data.json";

const world = World.new({
  proxyUrl: envChain.publicProxyUrl(),
  chainId: envChain.id(),
  gasPrice: 1000000000,
});

const wallet = world.newWalletFromFile_unsafe("wallet.json", "1234");

const program = new Command();

program.command("deploy").action(async () => {
  const result = await wallet.deployContract({
    code: data.code,
    codeMetadata: [],
    gasLimit: 20_000_000,
  });
  console.log("Result:", result);
});

program.parse(process.argv);
