@0xf945b45047097e91;

struct Package {
  name @0: Text;
  binName @1: Text;
  description @2: Text;
  note @3: Text;
  version @4: Text;
  downloadUrl @5: Text;
  size @6: Text;
  bsum @7: Text;
  buildDate @8: Text;
  srcUrl @9: Text;
  webUrl @10: Text;
  buildScript @11: Text;
  buildLog @12: Text;
  category @13: Text;
  extraBins @14: Text;
  icon @15: Text;
  desktop @16: Text;
}

struct Collection {
  key @0: Text;
  value @1: List(Package);
}

struct PackageList {
  packages @0: List(Collection);
}
