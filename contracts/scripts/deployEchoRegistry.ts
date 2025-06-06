import { toNano } from "@ton/core";
import { EchoRegistry } from "../build/EchoRegistry/EchoRegistry_EchoRegistry";
import { NetworkProvider } from "@ton/blueprint";


export async function run(provider: NetworkProvider) {
  const registry = provider.open(
    await EchoRegistry.fromInit(),
  );

  await registry.send(
    provider.sender(),
    {
      value: toNano("0.05"),
    },
    null,
  );

  await provider.waitForDeploy(registry.address);

  console.log("EchoRegistry deployed to:", registry.address);
}
