import { Cell, toNano } from "@ton/core";
import { EchoRegistry } from "../build/EchoRegistry/EchoRegistry_EchoRegistry";
import { EchoVault } from "../build/EchoVault/EchoVault_EchoVault";
import { EchoPiece } from "../build/EchoPiece/EchoPiece_EchoPiece";
import { NetworkProvider } from "@ton/blueprint";
// import * as fs from 'fs';

// async function readJsonField(filePath: string, field: string): Promise<any> {
//   const data = await fs.readFile(filePath, 'utf8');
//   const jsonData = JSON.parse(data);
//   return jsonData[field];
// }

export async function run(provider: NetworkProvider) {
  const echoVaultHex = require("../build/EchoVault.compiled.json")["hex"];
  const echoVaultCode = Cell.fromHex(echoVaultHex);

  const echoPieceHex = require("../build/EchoPiece.compiled.json")["hex"];
  const echoPieceCode = Cell.fromHex(echoPieceHex);

  const registry = provider.open(
    await EchoRegistry.fromInit(echoVaultCode, echoPieceCode),
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
