// Acts as a "Wallet" stores all the EchoPieces created by the user. Only the
// creator of the wallet can manage it

import "@stdlib/ownable";
import "./address_list.tact";
import "./messages.tact";

contract EchoVault(
  owner: Address,
  registry: Address,
  version: Int as uint32,
  pieces: AddressList
) with Ownable{

  receive() {}

  receive(msg: AddPiece){
    require(sender() == self.registry, "Only registry can add new pieces");
    self.pieces.push(msg.pieceAddress);
  }
}
