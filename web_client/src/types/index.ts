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
  pieceRemixData?: { [address: string]: string | null } | null;
}

export interface Wallet {
  account: {
    address: string;
  };
}

// TonBridge interface for global window object
export interface TonBridge {
  getContractInfo: () => ContractInfo;
  registryAddress: () => string | null;
  isWalletConnected: () => boolean;
  getUserAddress: () => string | null;
  getUserVaultAddress: () => string | null;
  getPieceAddresses: () => string[] | null;
  getPieceData: () => { [address: string]: string | null };
  getPieceRemixData: () => { [address: string]: string | null } | null;
  refreshVaultAddress: () => Promise<string | null>;
  saveAudioGraph: (audioGraphData: string) => Promise<boolean>;
  loadAudioGraph: (nftAddress: string) => Promise<string | null>;
  createNewPiece: (pieceRawData: string, remixedFrom?: any) => Promise<boolean>;
  setPendingPieceData: (pieceRawData: string | null, remixedFrom?: string | null) => void;
  getPendingPieceData: () => { pieceData: string | null; remixedFrom: string | null };
  clearPendingPieceData: () => void;
}

// Extend Window interface
declare global {
  interface Window {
    tonBridge: TonBridge;
  }
}
