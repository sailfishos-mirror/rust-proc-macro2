#![allow(clippy::ignore_without_reason)]

#[test]
#[ignore]
fn make_sure_no_proc_macro() {
    assert!(
        !cfg!(feature = "proc-macro"),
        "still compiled with proc_macro?"
    );
}
