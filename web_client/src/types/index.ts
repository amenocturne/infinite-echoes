// Type definitions for the application
export interface ContractInfo {
  pieceVersion: number | null;
  vaultVersion: number | null;
  feeParams: {
    deployValue: string;
    messageValue: string;
  } | null;
  securityParams: {
    minActionFee: string;
    coolDownSeconds: number;
  } | null;
}

export interface Wallet {
  account: {
    address: string;
  };
}
