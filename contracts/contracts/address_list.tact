const MAX_ADDRESSES: Int = 65536;

struct AddressList {
    addressesIncluded: map<Address, Bool>; // For existence checking
    addresses: map<Int as uint16, Address>;
    length: Int as uint16;
}

fun emptyAddressList(): AddressList {
  return AddressList{
    addressesIncluded: emptyMap(),
    addresses: emptyMap(),
    length: 0
  };
}

extends mutates fun push(self: AddressList, addr: Address): AddressList{
  require(self.length < MAX_ADDRESSES, "Can't add more addresses");
  self.addresses.set(self.length, addr);
  self.addressesIncluded.set(addr, true);
  self.length = self.length + 1;
  return self
}

extends fun contains(self: AddressList, addr: Address): Bool{
  return self.addressesIncluded.exists(addr)
}
