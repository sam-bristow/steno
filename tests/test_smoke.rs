/*!
 * Smoke tests for steno.  These aren't close to exhaustive, but tests that it's
 * not completely broken.
 */

use expectorate::assert_contents;
use std::env::current_exe;
use std::path::PathBuf;
use subprocess::Exec;
use subprocess::Redirection;

fn example_bin() -> PathBuf {
    /*
     * This is unfortunate, but it's the best way I know to run one of the
     * examples out of our project.
     */
    let mut my_path = current_exe().expect("failed to find test program");
    my_path.pop();
    assert_eq!(my_path.file_name().unwrap(), "deps");
    my_path.pop();
    my_path.push("examples");
    my_path.push("demo-provision");
    my_path
}

fn run_example(test_name: &str, config_fn: impl Fn(Exec) -> Exec) -> String {
    let config = config_fn(Exec::cmd(example_bin()).stdout(Redirection::Pipe));
    let cmdline = config.to_cmdline_lossy();
    eprintln!("test \"{}\": run: {}", test_name, cmdline);
    config.capture().expect("failed to execute command").stdout_str()
}

#[test]
fn no_args() {
    assert_contents(
        "tests/test_smoke_no_args.out",
        &run_example("no_args", |exec| exec.stderr(Redirection::Merge)),
    );
}

#[test]
fn cmd_info() {
    assert_contents(
        "tests/test_smoke_info.out",
        &run_example("info", |exec| {
            exec.stderr(Redirection::Merge).arg("info")
        }),
    );
}

#[test]
fn cmd_dot() {
    assert_contents(
        "tests/test_smoke_dot.out",
        &run_example("dot", |exec| exec.stderr(Redirection::Merge).arg("dot")),
    );
}

#[test]
fn cmd_run_basic() {
    assert_contents(
        "tests/test_smoke_run_basic.out",
        &run_example("run_basic", |exec| exec.arg("run")),
    );
}

#[test]
fn cmd_run_error() {
    assert_contents(
        "tests/test_smoke_run_error.out",
        &run_example("run_error", |exec| {
            exec.arg("run").arg("--inject-error=instance_boot")
        }),
    );
}
