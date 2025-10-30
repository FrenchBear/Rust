// rfind tests
//
// 2025-10-30   PV      First version of the tests

#[cfg(test)]
mod tests {
    use assert_cmd::{Command, cargo};
    use predicates::prelude::*;
    use std::collections::HashSet;

    #[test]
    fn test_find_rs_files_in_src() {
        #[allow(deprecated)]
        let mut cmd = Command::new(cargo::cargo_bin("rfind"));
        let assert = cmd.arg("src\\*.rs").arg("-print").assert();

        assert.success().stdout(predicate::function(|output: &str| {
            let mut expected_files: HashSet<&str> = [
                "src\\actions.rs",
                "src\\command_to_run.rs",
                "src\\fa_streams.rs",
                "src\\filters.rs",
                "src\\main.rs",
                "src\\options.rs",
                "src\\tests.rs",
            ]
            .iter()
            .cloned()
            .collect();

            for line in output.lines() {
                if !expected_files.remove(line) {
                    // Found a line that was not expected, or a duplicate
                    return false;
                }
            }

            // If we found all expected files, the set should be empty
            expected_files.is_empty()
        }));
    }

    #[test]
    fn test_exec() {
        #[allow(deprecated)]
        let mut cmd = Command::new(cargo::cargo_bin("rfind"));
        let assert = cmd.arg("src\\*.rs").arg("-exec").arg("cmd").arg("/c").arg("echo").assert();

        assert.success().stdout(predicate::function(|output: &str| {
            let mut expected_files: HashSet<&str> = [
                "src\\actions.rs",
                "src\\command_to_run.rs",
                "src\\fa_streams.rs",
                "src\\filters.rs",
                "src\\main.rs",
                "src\\options.rs",
                "src\\tests.rs",
            ]
            .iter()
            .cloned()
            .collect();

            for line in output.lines() {
                if !expected_files.remove(line) {
                    // Found a line that was not expected, or a duplicate
                    return false;
                }
            }

            // If we found all expected files, the set should be empty
            expected_files.is_empty()
        }));
    }

    #[test]
    fn test_yaml() {
        #[allow(deprecated)]
        let mut cmd = Command::new(cargo::cargo_bin("rfind"));
        let assert = cmd.arg("src\\fa*.rs").arg("-yaml").assert();

        assert.success().stdout(predicate::function(|output: &str| {
            let mut expected_files: HashSet<&str> = [
                r"- typ: file",
                r"",
                r"  old: 'src\fa_streams.rs'",
                r"  new: 'src\fa_streams.rs'",
            ]
            .iter()
            .cloned()
            .collect();

            for line in output.lines() {
                if !expected_files.remove(line) {
                    // Found a line that was not expected, or a duplicate
                    return false;
                }
            }

            // If we found all expected files, the set should be empty
            expected_files.is_empty()
        }));
    }
}
