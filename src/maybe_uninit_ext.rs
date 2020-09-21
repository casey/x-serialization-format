use crate::common::*;

pub(crate) trait MaybeUninitExt<T> {
  fn cast<U>(&self) -> &MaybeUninit<U>;
}

impl<T> MaybeUninitExt<T> for MaybeUninit<T> {
  fn cast<U>(&self) -> &MaybeUninit<U> {
    assert_eq!(mem::align_of::<U>(), 1);
    assert!(mem::size_of::<T>() >= mem::size_of::<U>());

    let pointer: *const MaybeUninit<T> = self;
    let pointer = pointer as *const MaybeUninit<U>;

    // Safe because:
    // - pointer is not null becuase it's derived from a reference
    // - MaybeUinint<U> has no alignment requirments
    // - MaybeUinint<U> is the same size or smaller than MaybeUninit<T>
    unsafe { &*pointer }
  }
}
