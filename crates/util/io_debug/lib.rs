//! # 文字列から入力/標準出力+読み込みに出力のセット

use io_util::*;
use prelude::*;

#[snippet(name = "io-debug", doc_hidden)]
// #[rustfmt::skip]
pub use io_debug_impl::{Assertion, FValueAssertion, IODebug, NoAssertion, StaticAssertion};
#[snippet(name = "io-debug", doc_hidden)]
// #[rustfmt::skip]
mod io_debug_impl {
    use super::{stdout, BufWriter, Display, ReaderFromStr, ReaderTrait, Write, WriterTrait};

    pub struct IODebug<A> {
        pub reader: ReaderFromStr,
        pub test_reader: ReaderFromStr,
        pub buf: String,
        enable_stdout: bool,
        assert: A,
    }

    impl<A: Assertion> WriterTrait for IODebug<A> {
        fn out<S: Display>(&mut self, s: S) {
            self.buf.push_str(&s.to_string());
        }
        fn flush(&mut self) {
            if self.enable_stdout {
                let stdout = stdout();
                let mut writer = BufWriter::new(stdout.lock());
                write!(writer, "{}", self.buf).expect("Failed to write.");
                let _ = writer.flush();
            }
            self.test_reader.push(&self.buf);
            self.buf.clear();
            self.assert.assert(&mut self.test_reader, &mut self.reader)
        }
    }

    impl<F> ReaderTrait for IODebug<F> {
        fn next(&mut self) -> Option<String> {
            self.reader.next()
        }
    }

    impl<A> IODebug<A> {
        pub fn new(initial_input: &str, enable_stdout: bool, assert: A) -> Self {
            Self {
                reader: ReaderFromStr::new(initial_input),
                test_reader: ReaderFromStr::new(""),
                buf: String::new(),
                enable_stdout,
                assert,
            }
        }
    }
    impl IODebug<StaticAssertion> {
        pub fn static_assert(input: &str, expect: &str) -> Self {
            IODebug::new(
                input,
                false,
                StaticAssertion {
                    expect: ReaderFromStr::new(expect),
                },
            )
        }
    }
    impl IODebug<FValueAssertion> {
        pub fn fvalue_assert(input: &str, expect: &str) -> Self {
            IODebug::new(
                input,
                false,
                FValueAssertion {
                    expect: ReaderFromStr::new(expect),
                },
            )
        }
    }

    pub trait Assertion {
        fn assert(&mut self, output: &mut ReaderFromStr, re_input: &mut ReaderFromStr);
    }

    pub struct NoAssertion;
    impl Assertion for NoAssertion {
        fn assert(&mut self, _: &mut ReaderFromStr, _: &mut ReaderFromStr) {}
    }

    pub struct StaticAssertion {
        expect: ReaderFromStr,
    }
    impl Assertion for StaticAssertion {
        fn assert(&mut self, output: &mut ReaderFromStr, _: &mut ReaderFromStr) {
            while let Some(a) = output.next() {
                assert_eq!(Some(a), self.expect.next())
            }
            assert_eq!(None, self.expect.next())
        }
    }
    pub struct FValueAssertion {
        expect: ReaderFromStr,
    }
    impl Assertion for FValueAssertion {
        fn assert(&mut self, output: &mut ReaderFromStr, _: &mut ReaderFromStr) {
            use float_value::FValue;
            use std::str::FromStr;
            while let Some(a) = output.next() {
                if let Some(b) = self.expect.next() {
                    assert_eq!(FValue::from_str(&a), FValue::from_str(&b));
                } else {
                    panic!("expect exit but actual {}", a);
                }
            }
            assert_eq!(None, self.expect.next())
        }
    }
}

#[snippet(name = "io-debug", doc_hidden)]
pub use custom_assertion_impl::ClosureAssertion;
#[snippet(name = "io-debug")]
mod custom_assertion_impl {
    use super::{Assertion, ReaderFromStr, ReaderTrait, WriterTrait};
    pub struct ClosureAssertion {
        /// add status for assertions
        pub buf: i64,
    }
    impl Assertion for ClosureAssertion {
        fn assert(&mut self, output: &mut ReaderFromStr, re_input: &mut ReaderFromStr) {
            self.buf += output.v::<i64>();
            re_input.out(self.buf);
        }
    }
}

#[test]
fn test() {
    use string_util::*;
    let mut io = IODebug::new("", false, NoAssertion);
    io.out(123);
    io.out(456.line());
    io.out(&[1, 2, 3, 4, 5].join(" ").line());
    io.out(13.bits(5));
    io.flush();
    for &expect in &["123456", "1", "2", "3", "4", "5", "10110"] {
        assert_eq!(Some(expect.to_string()), io.test_reader.next());
    }
    assert!(io.next().is_none());
}

#[test]
fn interactive_test() {
    let mut io = IODebug::new("100", false, ClosureAssertion { buf: 100 });
    assert_eq!(100, io.v());
    io.out(1000);
    io.flush();
    assert_eq!(io.buf, "");
    assert_eq!(1100, io.v());
}
