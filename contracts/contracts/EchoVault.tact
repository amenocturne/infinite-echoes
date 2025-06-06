import "@stdlib/ownable";
import "./address_list.tact";
import "./messages.tact";

contract EchoVault(
  owner: Address,
  registry: Address,
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
