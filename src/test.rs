use crate::common::*;

pub(crate) fn ok<Native: X + Eq + Debug>(native: Native, want: &[u8]) {
  let have = native.serialize_to_vec();
  assert_eq!(have, want);
  let view = Native::View::load(&have).unwrap();
  let round_tripped = view.to_native();
  assert_eq!(native, round_tripped);
}

pub(crate) fn err<V: View + Debug>(bytes: &[u8], want: Error) {
  let have = V::load(&bytes).unwrap_err();
  assert_eq!(have, want);
}
