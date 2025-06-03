// Master Contract that manages EchoVault contracts
// Responsibilities:
// - Manages Echo Vaults (creation, mapping from user address to their EchoVault)

// Useful References:
// - https://github.com/tact-lang/contract-wallet/blob/main/sources/contract.tact

import "@stdlib/ownable";

contract EchoRegistry with Ownable{
  owner: Address;
  vaultCode: Cell;
  vaults: map<Address, Address>;

  init(vaultCode: Cell){
    self.vaultCode = vaultCode;
    self.owner = sender();
  }

  receive(msg: CreateVault) {
    let senderAddress = sender();
    require(!self.vaults.exists(senderAddress), "Vault already exists");

    let initMsg = InitVault { owner: senderAddress };

    let stateInit = StateInit{
      code: self.vaultCode,
      data: initMsg.toCell()
    };

    let vaultAddress = contractAddress(stateInit);

    deploy(DeployParameters {
      bounce: false,
      value: ton("0.1"),
      init: stateInit
    });

    self.vaults.set(senderAddress, vaultAddress);
  }

  get fun getVaultAddress(user: Address): Address? {
    return self.vaults.get(user);
  }

}

message CreateVault {}
message InitVault {
  owner: Address
}
