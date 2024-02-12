pub trait ResultFinalizer: Sized {
    fn ok<E>(self) -> Result<Self, E>;
    fn err<T>(self) -> Result<T, Self>;
}

impl<S> ResultFinalizer for S {
    fn ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }

    fn err<T>(self) -> Result<T, Self> {
        Err(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Foo {
        bar: usize,
        baz: bool,
    }

    fn return_ok() -> Result<Foo, ()> {
        Foo {
            bar: 123,
            baz: true,
        }
        .ok()
    }

    fn return_err() -> Result<(), Foo> {
        Foo {
            bar: 321,
            baz: false,
        }
        .err()
    }

    #[test]
    fn test_result_finalizer() {
        let foo = return_ok().unwrap();
        assert!(foo.baz);

        let foo = return_err().unwrap_err();
        assert_eq!(foo.bar, 321);
    }
}
