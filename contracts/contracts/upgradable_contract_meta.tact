struct UpgradableContractMeta{
  version: Int as uint32;
  code: Cell
}

extends mutates fun upgradeCode(self: UpgradableContractMeta, code: Cell): UpgradableContractMeta {
  self.version = self.version + 1;
  self.code = code;
  return self;
}

fun initUpgradableContractmeta(code: Cell): UpgradableContractMeta{
  return UpgradableContractMeta{
    version: 0,
    code: code
  };
}
