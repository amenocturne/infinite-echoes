import "@stdlib/ownable";
import "./address_list.tact";
import "./upgradable_contract_meta.tact";
import "./EchoPiece.tact";
import "./EchoVault.tact";

struct FeeParams{
  deployValue: Int;
  messageValue: Int;
}

struct SecurityParams{
  minActionFee: Int;
  coolDownSeconds: Int;
}

contract EchoRegistry with Ownable{
  owner: Address;
  vaults: map<Address, Address>;
  feeParams: FeeParams;
  securityParams: SecurityParams;

  init() {
    self.feeParams = FeeParams{
      deployValue: ton("0.02"),
      messageValue: ton("0.005"),
    };
    self.securityParams = SecurityParams{
      minActionFee: ton("0.005"),
      coolDownSeconds: 30,
    };
    self.owner = sender();
  }

  receive() {}

  // --------------------------- Configuration --------------------------------


  receive(msg: UpdateFeeParams) {
    require(sender() == self.owner, "Only owner can upgrade code of pieces");
    self.feeParams = FeeParams{
      deployValue: msg.deployValue,
      messageValue: msg.messageValue,
    }
  }
  receive(msg: UpdateSecurityParams) {
    require(sender() == self.owner, "Only owner can upgrade code of pieces");
    self.securityParams = SecurityParams{
      minActionFee: msg.minActionFee,
      coolDownSeconds: msg.coolDownSeconds
    }
  }
  receive(msg: RemoveVault) {
    require(sender() == self.owner, "Only owner can remove vaults");
    self.vaults.del(msg.user);
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

    let stateInit = initOf EchoVault(
      user,
      myAddress(),
      emptyAddressList(),
      now() - self.securityParams.coolDownSeconds - 1,
      self.securityParams.coolDownSeconds
    );

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
    let stateInit = initOf EchoPiece(
      vault,
      remixedFrom,
      now(),
      data
    );

    let pieceAddress = contractAddress(stateInit);

    deploy(DeployParameters {
      bounce: false,
      value: self.feeParams.deployValue,
      init: stateInit
    });

    return pieceAddress
  }

  fun proofOfStakeCheck(value: Int){
    require(value >= self.securityParams.minActionFee, "Insufficient anti-spam fee");
  }

  // ------------------------------- Getters ----------------------------------

  get fun getVaultAddress(user: Address): Address? {
    return self.vaults.get(user);
  }

  get fun getFeeParams(): FeeParams {
    return self.feeParams;
  }

  get fun getSecurityParams(): SecurityParams {
    return self.securityParams;
  }
}
