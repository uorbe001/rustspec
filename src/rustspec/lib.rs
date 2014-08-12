#![crate_name="rustspec"]
#![crate_type="lib"]
#![feature(macro_rules)]
#![feature(phase)]

#[macro_export]
pub macro_rules! passes(
    ($test_name:ident $body:block) => (
        #[test]
        fn $test_name() {
            $body
        }
    );
)

#[macro_export]
pub macro_rules! fails(
    ($test_name:ident $body:block) => (
        #[test]
        #[should_fail]
        fn $test_name() {
            $body
        }
    );
)

#[macro_export]
pub macro_rules! ignores(
    ($test_name:ident $body:block) => (
        #[test]
        #[ignore]
        fn $test_name() {
            $body
        }
    );
)

#[macro_export]
pub macro_rules! when(
    ($context_name:ident { $(it $body:item)* $(test $tests:item)* }) => (
        mod $context_name {
            extern crate rustspec_assertions;
            #[allow(unused_imports)]
            use rustspec_assertions::{expect, eq, be_gt, be_ge, be_le, be_lt};

            $($body)*
            $($tests)*
        }
    );
)

#[macro_export]
pub macro_rules! scenario(
    ($context_name:ident { $(test $body:item)* }) => (
        mod $context_name {
            $($body)*
        }
    );
)

#[macro_export]
pub macro_rules! lets(
    ($var_name:ident: $kind:ty -> $value:expr) => (
        static $var_name: $kind = $value;
    );
)
