#[cfg(test)]
pub mod utils {
    pub struct TestContext<F: Fn()> {
        pub teardown_fn: F,
    }

    impl<F: Fn()> TestContext<F> {
        pub fn new(setup_fn: impl Fn(), teardown_fn: F) -> Self {
            setup_fn();

            Self { teardown_fn }
        }
    }

    impl<F: Fn()> Drop for TestContext<F> {
        fn drop(&mut self) {
            (self.teardown_fn)();
        }
    }
}
