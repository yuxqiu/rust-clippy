//@aux-build:option_helpers.rs
#![warn(clippy::option_filter_is_some)]

#[macro_use]
extern crate option_helpers;

#[rustfmt::skip]
fn option_methods() {
    let opt = Some(1);

    // Check for `option.filter(_).is_some()` use.
    // Single line case.
    let _ = opt.filter(|x| *x > 1)
        // Should lint even though this call is on a separate line.
        .is_some();
    // Multi-line cases.
    let _ = opt.filter(|x| {
        *x > 1
    }
    ).is_some();

    let y = 2;
    let _ = opt.filter(|_| y > 1).is_some();

    let _ = opt.filter(|x| x.eq(&5)).is_some();

    let _ = opt.filter(|&x| x > 1).is_some();

    // won't fix because the argument for the closure is type annotated
    let _ = opt.filter(|&x: &i32| x > 1).is_some();

    // won't fix because
    // - the closure is bounded to a variable
    // - the argument for the closure is type annotated
    let predicate = |x: &i32| *x > 1;
    let _ = opt.filter(predicate).is_some();

    fn check_num(num: &i32) -> bool {
        *num > 1
    }

    // won't fix because `check_num` is not a closure
    let _ = opt.filter(check_num).is_some();

    // won't fix because of macro
    let _ = opt_filter!(opt, |x| *x == 1).is_some();
}
