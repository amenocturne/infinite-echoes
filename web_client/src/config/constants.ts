import { Address } from "@ton/core";

// Application constants
export const TON_TESTNET_API =
  "https://testnet.tonapi.io/v2/blockchain/accounts";
export const TON_API_TOKEN =
  "AE4JXL6HYJ3COLAAAAANTD6QYMQT2HB363XPK4SUJDTAAC5T6T653PQZAR6OFKXM7T7VPGY";
export const REGISTRY_ADDRESS =
  "0:f1fa839dd70c72f2aa33dbad6a16804a97e0ede5abcc75d25981da66942b3d78";
export const MANIFEST_URL =
  "https://infinite-echoes.app/tonconnect-manifest.json";

export const FRIENDLY_REGISTRY = toFriendlyAddr(REGISTRY_ADDRESS);

function toFriendlyAddr(full: string): string | null {
  const parts = full.split(":");
  if (parts.length !== 2) {
    return null;
  }

  const workchain = parseInt(parts[0]);
  const addressPart = parts[1];

  if (!addressPart) {
    return null;
  }

  // Create Address object and convert to friendly format
  const address = new Address(workchain, Buffer.from(addressPart, "hex"));
  const friendlyAddress = address.toString({
    testOnly: true, // Using testOnly: true for testnet
    bounceable: true,
  });
  return friendlyAddress;
}


// Particle system constants
export const PARTICLE_CONFIG = {
  COUNT: 100,
  MAX_COUNT: 200,
  INTERVAL_MS: 50,
  SIZES: ["small", "medium", "large"],
  MIN_DURATION: 8,
  MAX_DURATION: 23,
  MAX_DRIFT: 200,
};

// Transaction constants
export const TRANSACTION_CONFIG = {
  VALID_SECONDS: 360,
  DEFAULT_AMOUNT: "0.1",
};
