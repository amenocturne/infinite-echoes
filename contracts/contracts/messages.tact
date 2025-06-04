// ------------------------------ Registry ------------------------------------
message UpgradeVaultCode{
  code: Cell
}

message UpgradePieceCode{
  code: Cell
}

message CreatePiece{
  pieceData: Cell;
  remixedFrom: Address?
}

// ------------------------------ Vault ------------------------------------

message AddPiece{
  pieceAddress: Address
}
