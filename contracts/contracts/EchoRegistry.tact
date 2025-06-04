// Master Contract that manages EchoVault contracts
// Responsibilities:
// - Manages Echo Vaults (creation, mapping from user address to their EchoVault)

// Useful References:
// - https://github.com/tact-lang/contract-wallet/blob/main/sources/contract.tact

import "@stdlib/ownable";
import "./constants.tact";
import "./address_list.tact";
import "./upgradable_contract_meta.tact";
import "./EchoPiece.tact";
import "./EchoVault.tact";

contract EchoRegistry with Ownable{
  owner: Address;
  pieceMeta: UpgradableContractMeta;
  vaultMeta: UpgradableContractMeta;
  vaults: map<Address, Address>;

  init(vaultCode: Cell, pieceCode: Cell){
    self.vaultMeta = initUpgradableContractmeta(vaultCode);
    self.pieceMeta = initUpgradableContractmeta(pieceCode);
    self.owner = sender();
  }

  receive() {}

  // --------------------------- Code Upgrades --------------------------------

  receive(msg: UpgradeVaultCode) {
    require(sender() == self.owner, "Only owner can upgrade code of vaults");
    self.vaultMeta.upgradeCode(msg.code);
  }

  receive(msg: UpgradePieceCode) {
    require(sender() == self.owner, "Only owner can upgrade code of pieces");
    self.pieceMeta.upgradeCode(msg.code);
  }

  // ------------------ Piece And Vault Creation ------------------------------

  receive(msg: CreatePiece) {
    let senderAddress = sender();
    let vaultAddress = self.getOrCreateVault(senderAddress);
    let pieceAddress = self.createPiece(vaultAddress, msg.pieceData, msg.remixedFrom);
    send(SendParameters{
      to: vaultAddress,
      value: MESSAGE_VALUE,
      bounce: false,
      body: AddPiece {pieceAddress: pieceAddress}.toCell()
    })
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
      value: DEPLOYMENT_VALUE,
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
      value: DEPLOYMENT_VALUE,
      init: stateInit
    });

    return pieceAddress
  }

  // ------------------------------- Getters ----------------------------------

  get fun getVaultAddress(user: Address): Address? {
    return self.vaults.get(user);
  }

  get fun getPieceVersion(user: Address): Int {
    return self.pieceMeta.version;
  }

  get fun getVaultVersion(user: Address): Int {
    return self.pieceMeta.version;
  }
}
