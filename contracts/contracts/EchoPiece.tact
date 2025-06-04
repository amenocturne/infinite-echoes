// EchoPiece:
// - immutably stores piece information (which is provided as a binary data) in a linked-list of cells
// - Stores metadata about a piece (creator, creation time, remixed from )
// - Implements NFT TEP-62 standard

import "@stdlib/ownable";

contract EchoPiece(
  vault: Address,
  remixedFrom: Address?,
  created: Int as uint32,
  version: Int as uint32,
  data: Cell
) {
  receive() {}

}
