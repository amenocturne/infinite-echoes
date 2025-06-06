// Type definitions for the application
export interface ContractInfo {
  feeParams: {
    deployValue: number;
    messageValue: number;
  } | null;
  securityParams: {
    minActionFee: number;
    coolDownSeconds: number;
  } | null;
  userVaultAddress: string | null;
  pieceCount: number | null;
  pieceAddresses: string[] | null;
  pieceData: { [address: string]: string | null } | null;
}

export interface Wallet {
  account: {
    address: string;
  };
}

// TonBridge interface for global window object
export interface TonBridge {
  getContractInfo: () => ContractInfo;
  isWalletConnected: () => boolean;
  getUserAddress: () => string | null;
  getUserVaultAddress: () => string | null;
  getPieceAddresses: () => string[] | null;
  getPieceData: () => { [address: string]: string | null }; // Changed this line
  refreshVaultAddress: () => Promise<string | null>;
  saveAudioGraph: (audioGraphData: string) => Promise<boolean>;
  loadAudioGraph: (nftAddress: string) => Promise<string | null>;
  createNewPiece: (pieceRawData: string, remixedFrom?: any) => Promise<boolean>;
}

// Extend Window interface
declare global {
  interface Window {
    tonBridge: TonBridge;
  }
}
