import { Address, toNano } from "@ton/core";
import { EchoRegistry } from "../build/EchoRegistry/EchoRegistry_EchoRegistry";
import { NetworkProvider } from "@ton/blueprint";
import { RemoveVault } from "../build/EchoRegistry/EchoRegistry_EchoRegistry";

export async function run(provider: NetworkProvider, args: string[]) {
  if (args.length < 1) {
    console.error("Usage: npx blueprint run removeVault <user-address>");
    process.exit(1);
  }

  // Parse the user address from command line arguments
  const userAddress = Address.parse(args[0]);
  
  // Open the existing registry contract
  const registry = provider.open(await EchoRegistry.fromAddress(provider.sender().address!));

  // Create the RemoveVault message
  const message: RemoveVault = {
    $$type: 'RemoveVault',
    user: userAddress
  };

  // Send the message to the registry
  await registry.send(
    provider.sender(),
    {
      value: toNano("0.05"),
    },
    message
  );

  console.log(`RemoveVault message sent for user: ${userAddress.toString()}`);
}
