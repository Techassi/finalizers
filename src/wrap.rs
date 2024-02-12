/// Wraps any type in another type.
///
/// ### Basic Example
///
/// ```
/// use finalizers::Wrap;
///
/// let str = "foo";
/// let str = str.wrap(String::from);
/// ```
///
/// ### Wrapping Match Expressions
///
/// ```
/// use std::sync::Arc;
/// use finalizers::Wrap;
///
/// # pub enum Mode {
/// #     Foo,
/// #     Bar,
/// #     Baz,
/// # }
/// let mode = Mode::Foo;
///
/// let arc = match mode {
///     Mode::Foo => String::from("foo"),
///     Mode::Bar => String::from("bar"),
///     Mode::Baz => String::from("baz"),
/// }.wrap(Arc::new);
/// ```
pub trait Wrap<S, T>: Sized {
    fn wrap(self, f: impl Fn(S) -> T) -> T;
}

impl<S, T> Wrap<S, T> for S {
    fn wrap(self, f: impl Fn(S) -> T) -> T {
        f(self)
    }
}
