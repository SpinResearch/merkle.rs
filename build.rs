#[cfg(feature = "serialization-protobuf")]
extern crate protoc_rust;

#[cfg(feature = "serialization-protobuf")]
fn assert_protobuf_version(version: &str) {
    use std::process::{Command, Stdio};
    let protoc = Command::new("protoc")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(&["--version"])
        .spawn()
        .unwrap();
    let version_output = protoc.wait_with_output().unwrap();
    assert!(version_output.status.success());
    assert_eq!(
        String::from_utf8(version_output.stdout).unwrap().trim(),
        version
    );
}

#[cfg(feature = "serialization-protobuf")]
fn build_protobuf(out_dir: &str, input: &[&str], includes: &[&str]) {
    use self::protoc_rust::{run, Args};
    run(Args {
        out_dir,
        input,
        includes,
        customize: Default::default(),
    }).expect("protoc");
}

#[cfg(feature = "serialization-protobuf")]
fn build_protobuf_schemata() {
    assert_protobuf_version("libprotoc 3.5.1");
    build_protobuf("src/proto", &["protobuf/proof.proto"], &[]);
}

fn main() {
    #[cfg(feature = "serialization-protobuf")]
    build_protobuf_schemata();
}
