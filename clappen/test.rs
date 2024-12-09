#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[allow(dead_code)]
#[cfg(test)]
mod tests {
    /// Macros used for nested struct definition : []
    /// Struct with prefix '', default_prefix: ''
    pub struct MyStruct {}
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::it_works"]
    #[doc(hidden)]
    pub const it_works: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::it_works"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "clappen/tests/full_default_prefix.rs",
            start_line: 53usize,
            start_col: 8usize,
            end_line: 53usize,
            end_col: 16usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(it_works()),
        ),
    };
    fn it_works() {
        /// Macros used for nested struct definition : []
        /// Struct with prefix '', default_prefix: 'my_prefix'
        pub struct ServerOptions {
            /// Address to connect to.
            ///
            my_prefix_url: String,
            /// Do you need to say hello?.
            ///
            my_prefix_say_hello: Option<bool>,
            /// A nested struct without a prefix.
            ///
            my_prefix_nested_default: MyStruct,
        }
        impl ServerOptions {
            fn a_function(&self) -> String {
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "url: {0}, say_hello: {1:?}",
                            self.url,
                            self.say_hello,
                        ),
                    );
                    res
                })
            }
            fn another_function(&self) {}
        }
        impl ServerOptions {
            fn a_third_function_in_second_impl_block(&self) {}
        }
        /// Macros used for nested struct definition : []
        /// Struct with prefix 'test', default_prefix: 'my_prefix'
        pub struct MyPrefixTestServerOptions {
            /// Address to connect to.
            ///
            my_prefix_test_url: String,
            /// Do you need to say hello?.
            ///
            my_prefix_test_say_hello: Option<bool>,
            /// A nested struct without a prefix.
            ///
            my_prefix_test_nested_default: MyStruct,
        }
        /// Fields with prefix: [my_prefix_url,my_prefix_say_hello,my_prefix_nested_default]
        impl MyPrefixTestServerOptions {
            fn a_function(&self) -> String {
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "url: {0}, say_hello: {1:?}",
                            self.url,
                            self.say_hello,
                        ),
                    );
                    res
                })
            }
            fn another_function(&self) {}
        }
        /// Fields with prefix: [my_prefix_url,my_prefix_say_hello,my_prefix_nested_default]
        impl MyPrefixTestServerOptions {
            fn a_third_function_in_second_impl_block(&self) {}
        }
        let a = ServerOptions {
            my_prefix_url: "url a".into(),
            my_prefix_say_hello: Some(false),
            my_prefix_nested_default: MyStruct {},
        };
        match (&a.a_function(), &"url: url a, say_hello: Some(false)") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        a.another_function();
        a.a_third_function_in_second_impl_block();
        let b = MyPrefixTestServerOptions {
            my_prefix_test_url: "url b".into(),
            my_prefix_test_say_hello: Some(true),
            my_prefix_test_nested_default: MyStruct {},
        };
        match (&b.a_function(), &"url: url b, say_hello: Some(true)") {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        b.another_function();
        b.a_third_function_in_second_impl_block();
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&it_works])
}
