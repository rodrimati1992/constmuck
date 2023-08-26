mod main_tests {
    #[macro_use]
    mod test_utils;

    mod contiguous_tests;
    mod pod_tests;
    mod ui_tests;
    mod wrapper_tests;
    mod zeroable;
}

// for making sure that debug assertions are actually enabled <_<
#[test]
fn debug_flags() {
    if cfg!(feature = "__testdev") {
        assert!(cfg!(debug_assertions));
    }
    if cfg!(feature = "__testnodev") {
        assert!(!cfg!(debug_assertions));
    }
}
