import "@stdlib/ownable";

contract EchoPiece(
  vault: Address,
  remixedFrom: Address?,
  created: Int as uint32,
  data: Cell
) {
  receive() {}

  get fun getVault(): Address {
    return self.vault
  }

  get fun getData(): Cell {
    return self.data
  }

  get fun getRemixedFrom(): Address? {
    return self.remixedFrom
  }

  get fun getCreated(): Int {
    return self.created
  }
}
