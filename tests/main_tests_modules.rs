mod main_tests {
    #[macro_use]
    mod test_utils;

    mod contiguous_tests;
    mod copying_zeroable;
    mod marker_type_construction;
    mod pod_tests;
    mod transmute_and_wrapper;
    mod type_size_tests;
    mod ui_tests;
}
