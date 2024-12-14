use rs_car_sync::car_read_all;

enum TestResult {
    Error(&'static str),
    Success,
}

macro_rules! car_read_all_test {
    ($name:ident, $file:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let result = std::panic::catch_unwind(|| {
                let mut file = std::fs::File::open($file).unwrap();
                car_read_all(&mut file, true)
            });

            match result {
                Ok(Ok(_)) => match $expected {
                    TestResult::Success => {} // Ok
                    TestResult::Error(err) => {
                        panic!("expected error but got success: {:?}", err)
                    }
                },
                Ok(Err(err)) => match $expected {
                    TestResult::Success => {
                        panic!("expected success but got error: {:?}", err)
                    }
                    TestResult::Error(expected_err) => {
                        assert_eq!(err.to_string(), expected_err)
                    }
                },
                Err(panic_error) => match $expected {
                    TestResult::Success => panic!("expected panic but got success"),
                    TestResult::Error(expected_err) => {
                        panic!(
                            "expected error but got panic: {:?} \n {:?}",
                            panic_error, expected_err
                        )
                    }
                },
            };
        }
    };
}

car_read_all_test!(
    go_car_fixture_sample_corrupt_pragma,
    "tests/go_car_fixtures/sample-corrupt-pragma.car",
    TestResult::Error(
        "IoError(Error { kind: UnexpectedEof, message: \"failed to fill whole buffer\" })"
    )
);
car_read_all_test!(
    go_car_fixture_sample_rootless_v42,
    "tests/go_car_fixtures/sample-rootless-v42.car",
    TestResult::Error("UnsupportedCarVersion { version: 42 }")
);
car_read_all_test!(
    go_car_fixture_sample_rw_bs_v2,
    "tests/go_car_fixtures/sample-rw-bs-v2.car",
    TestResult::Success
);
car_read_all_test!(
    go_car_fixture_sample_unixfs_v2,
    "tests/go_car_fixtures/sample-unixfs-v2.car",
    TestResult::Success
);
car_read_all_test!(
    go_car_fixture_sample_v1,
    "tests/go_car_fixtures/sample-v1.car",
    TestResult::Success
);
car_read_all_test!(
    go_car_fixture_sample_v1_noidentity,
    "tests/go_car_fixtures/sample-v1-noidentity.car",
    TestResult::Success
);
car_read_all_test!(
    go_car_fixture_sample_v1_tailing_corrupt_section,
    "tests/go_car_fixtures/sample-v1-tailing-corrupt-section.car",
    TestResult::Error(
        "IoError(Error { kind: UnexpectedEof, message: \"failed to fill whole buffer\" })"
    )
);
car_read_all_test!(
    go_car_fixture_sample_v1_with_zero_len_section,
    "tests/go_car_fixtures/sample-v1-with-zero-len-section.car",
    TestResult::Error("InvalidBlockHeader(\"zero length\")")
);
car_read_all_test!(
    go_car_fixture_sample_v1_with_zero_len_section2,
    "tests/go_car_fixtures/sample-v1-with-zero-len-section2.car",
    TestResult::Error("InvalidBlockHeader(\"zero length\")")
);
car_read_all_test!(
    go_car_fixture_sample_v2_corrupt_data_and_index,
    "tests/go_car_fixtures/sample-v2-corrupt-data-and-index.car",
    TestResult::Error("InvalidCarV1Header(\"padding len too big 18446744073709550203\")")
);
car_read_all_test!(
    go_car_fixture_sample_v2_indexless,
    "tests/go_car_fixtures/sample-v2-indexless.car",
    TestResult::Success
);
car_read_all_test!(
    go_car_fixture_sample_wrapped_v2_is_okay,
    "tests/go_car_fixtures/sample-wrapped-v2.car",
    TestResult::Success
);
car_read_all_test!(
    custom_fixtures_config_size_1,
    "tests/custom_fixtures/config.toml.size-1.normal.car",
    TestResult::Success
);
car_read_all_test!(
    custom_fixtures_config_size_32,
    "tests/custom_fixtures/config.toml.size-32.normal.car",
    TestResult::Success
);
