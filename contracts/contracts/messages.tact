// ------------------------------ Registry ------------------------------------

// Configuration
message UpgradeVaultCode{
  code: Cell
}

message UpgradePieceCode{
  code: Cell
}

message UpdateFeeParams{
  deployValue: Int;
  messageValue: Int;
}
message UpdateSecurityParams{
  minActionFee: Int;
  coolDownSeconds: Int;
}

// User interactions
message CreatePiece{
  pieceData: Cell;
  remixedFrom: Address?
}

// ------------------------------ Vault ------------------------------------

message AddPiece{
  pieceAddress: Address
}
