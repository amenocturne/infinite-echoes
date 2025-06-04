struct AddressList {
    addresses: map<Int as uint16, Address>;
    length: Int as uint16;
}

fun emptyAddressList(): AddressList {
  return AddressList{
    addresses: emptyMap(),
    length: 0
  };
}

extends mutates fun push(self: AddressList, addr: Address): AddressList{
  self.addresses.set(self.length, addr);
  self.length = self.length + 1;
  return self
}
