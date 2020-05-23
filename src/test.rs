use crate::common::*;

pub(crate) fn ok<T: View + PartialEq + Debug>(buffer: &[u8], want: T) {
  let have = T::load(buffer).unwrap();
  assert_eq!(have, &want);

  assert_eq!(have.total_size(), buffer.len());

  let mut stored = vec![0; buffer.len()];

  want.store(&mut stored).unwrap();

  assert_eq!(stored, buffer);
}

pub(crate) fn err<T: View + Debug>(buffer: &[u8], want: Error) {
  let have = T::load(buffer).unwrap_err();
  assert_eq!(have, want);
}
