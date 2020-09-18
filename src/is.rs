// all credit to: https://github.com/clintonmead/is_type
/// This trait is implemented for types which are the same as Self::Type.
///
/// It is useful in where clauses, since equality constraints like the following
/// are not yet supported:
///
/// ```compile_fail
/// fn foo<T, S>(t: T) -> S
/// where
///   T = S,
/// {
///   t.into()
/// }
/// ```
///
/// The above can be written as follows using the `Is` trait:
///
/// ```
/// use x::Is;
///
/// fn foo<T, S>(t: T) -> S
/// where
///   T: Is<Type = S>,
/// {
///   t.identity()
/// }
/// ```
pub trait Is {
  type Type: ?Sized;

  fn identity(self) -> Self::Type;
}

impl<T> Is for T {
  type Type = T;

  fn identity(self) -> Self::Type {
    self
  }
}
