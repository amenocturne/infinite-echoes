import { Address } from '@ton/core';

export const TON_TESTNET_API = 'https://testnet.tonapi.io/v2/blockchain/accounts';
export const TON_API_TOKEN =
  'AE4JXL6HYJ3COLAAAAANTD6QYMQT2HB363XPK4SUJDTAAC5T6T653PQZAR6OFKXM7T7VPGY';
export const REGISTRY_ADDRESS =
  '0:bd87ec62c3b57e1d6601948bd94eaaec3011b1b52510248219c9d54104b82b5b';
export const MANIFEST_URL = 'https://infinite-echoes.app/tonconnect-manifest.json';

export const FRIENDLY_REGISTRY = toFriendlyAddr(REGISTRY_ADDRESS);

function toFriendlyAddr(full: string): string | null {
  const parts = full.split(':');
  if (parts.length !== 2) {
    return null;
  }

  const workchain = parseInt(parts[0]);
  const addressPart = parts[1];

  if (!addressPart) {
    return null;
  }

  const address = new Address(workchain, Buffer.from(addressPart, 'hex'));
  const friendlyAddress = address.toString({
    testOnly: true,
    bounceable: true,
  });
  return friendlyAddress;
}

export const PARTICLE_CONFIG = {
  COUNT: 100,
  MAX_COUNT: 200,
  INTERVAL_MS: 50,
  SIZES: ['small', 'medium', 'large'],
  MIN_DURATION: 8,
  MAX_DURATION: 23,
  MAX_DRIFT: 200,
};

export const TRANSACTION_CONFIG = {
  VALID_SECONDS: 360,
  DEFAULT_AMOUNT: '0.1',
};
