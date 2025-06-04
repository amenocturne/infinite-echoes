// Master Contract that manages EchoVault contracts
// Responsibilities:
// - Manages Echo Vaults (creation, mapping from user address to their EchoVault)

// Useful References:
// - https://github.com/tact-lang/contract-wallet/blob/main/sources/contract.tact

import "@stdlib/ownable";
import "./address_list.tact";
import "./upgradable_contract_meta.tact";
import "./EchoPiece.tact";
import "./EchoVault.tact";

struct FeeParams{
  deployValue: Int;
  messageValue: Int;
  minActionFee: Int;
}

contract EchoRegistry with Ownable{
  owner: Address;
  pieceMeta: UpgradableContractMeta;
  vaultMeta: UpgradableContractMeta;
  vaults: map<Address, Address>;
  feeParams: FeeParams;

  init(vaultCode: Cell, pieceCode: Cell) {
    self.vaultMeta = initUpgradableContractmeta(vaultCode);
    self.pieceMeta = initUpgradableContractmeta(pieceCode);
    self.feeParams = FeeParams{
      deployValue: ton("0.02"),
      messageValue: ton("0.005"),
      minActionFee: ton("0.005"),
    };
    self.owner = sender();
  }

  receive() {}

  // --------------------------- Configuration --------------------------------

  receive(msg: UpgradeVaultCode) {
    require(sender() == self.owner, "Only owner can upgrade code of vaults");
    self.vaultMeta.upgradeCode(msg.code);
  }

  receive(msg: UpgradePieceCode) {
    require(sender() == self.owner, "Only owner can upgrade code of pieces");
    self.pieceMeta.upgradeCode(msg.code);
  }

  receive(msg: UpdateFeeParams) {
    require(sender() == self.owner, "Only owner can upgrade code of pieces");
    self.feeParams = FeeParams{
      deployValue: msg.deployValue,
      messageValue: msg.messageValue,
      minActionFee: msg.minActionFee
    }
  }

  // ------------------ Piece And Vault Creation ------------------------------

  receive(msg: CreatePiece) {
    self.proofOfStakeCheck(context().value);
    let senderAddress = sender();
    let vaultAddress = self.getOrCreateVault(senderAddress);
    let pieceAddress = self.createPiece(vaultAddress, msg.pieceData, msg.remixedFrom);
    send(SendParameters{
      to: vaultAddress,
      value: self.feeParams.messageValue,
      bounce: false,
      body: AddPiece {pieceAddress: pieceAddress}.toCell()
    });
    cashback(sender());
  }


  inline fun getOrCreateVault(user: Address): Address{
    let maybeVaultAddr = self.vaults.get(user);
    if (maybeVaultAddr != null){
      return maybeVaultAddr!!
    }

    let stateInit = StateInit{
      code: self.vaultMeta.code,
      data: initOf EchoVault(user, myAddress(), self.vaultMeta.version, emptyAddressList()).toCell()
    };

    let vaultAddress = contractAddress(stateInit);

    deploy(DeployParameters {
      bounce: false,
      value: self.feeParams.deployValue,
      init: stateInit
    });

    self.vaults.set(user, vaultAddress);

    return vaultAddress
  }

  inline fun createPiece(vault: Address, data: Cell, remixedFrom: Address?): Address{
    let stateInit = StateInit{
      code: self.pieceMeta.code,
      data: initOf EchoPiece(vault, remixedFrom, now(), self.pieceMeta.version, data).toCell()
    };

    let pieceAddress = contractAddress(stateInit);

    deploy(DeployParameters {
      bounce: false,
      value: self.feeParams.deployValue,
      init: stateInit
    });

    return pieceAddress
  }

  fun proofOfStakeCheck(value: Int){
    require(value >= self.feeParams.minActionFee, "Insufficient anti-spam fee");
  }

  // ------------------------------- Getters ----------------------------------

  get fun getVaultAddress(user: Address): Address? {
    return self.vaults.get(user);
  }

  get fun getPieceVersion(): Int {
    return self.pieceMeta.version;
  }

  get fun getVaultVersion(): Int {
    return self.pieceMeta.version;
  }

  get fun getFeeParams(user: Address): FeeParams {
    return self.feeParams;
  }
}
