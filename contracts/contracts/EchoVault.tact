// Acts as a "Wallet" stores all the EchoPieces created by the user. Only the
// creator of the wallet can manage it

import "@stdlib/ownable";
import "./address_list.tact";
import "./messages.tact";

contract EchoVault(
  owner: Address,
  registry: Address,
  version: Int as uint32,
  pieces: AddressList,
  lastActionTime: Int,
  coolDownSeconds: Int
) with Ownable{

  receive() {}

  receive(msg: AddPiece){
    require(sender() == self.registry, "Only registry can add new pieces");
    require(now() - self.lastActionTime >= self.coolDownSeconds, "Too many requests");
    self.lastActionTime = now();
    self.pieces.push(msg.pieceAddress);
  }

  get fun getPieces(): map<Int as uint16, Address> {
    return self.pieces.addresses;
  }

  get fun getPieceCount(): Int {
    return self.pieces.length;
  }

  get fun containsPiece(address: Address): Bool {
    return self.pieces.contains(address)
  }
}
