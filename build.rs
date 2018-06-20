#[cfg(feature = "serialization-protobuf")]
extern crate protoc_rust;

#[cfg(feature = "serialization-protobuf")]
fn assert_protobuf_version(version: String) {
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
        version.trim()
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
    use std::fs::File;
    use std::io::Read;
    let mut version_string = String::new();
    let mut version_pin =
        File::open("PROTOC_VERSION").expect("protoc version pin `PROTOC_VERSION` file is missing");
    version_pin.read_to_string(&mut version_string).expect("cannot read protoc pin file");
    assert_protobuf_version(version_string);
    build_protobuf("src/proto", &["protobuf/proof.proto"], &[]);
}

fn main() {
    #[cfg(feature = "serialization-protobuf")]
    build_protobuf_schemata();
}
