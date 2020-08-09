use cli_test_dir::*;

pub fn test_sample(input: &str, output: &str, bin: &str) {
    let testdir = TestDir::new(bin, "");
    let process_output = testdir
        .cmd()
        .output_with_stdin(input)
        .tee_output()
        .expect_success();
    assert_eq!(process_output.stdout_str(), output);
    assert!(process_output.stderr_str().is_empty());
}
