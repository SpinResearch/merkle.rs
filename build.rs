#[cfg(feature = "serialization-protobuf")]
extern crate protoc_rust;

#[cfg(feature = "serialization-protobuf")]
fn has_right_protoc_version(version: &str) -> bool {
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

    let full_version = String::from_utf8(version_output.stdout).unwrap();
    full_version.trim() == format!("libprotoc {}", version.trim())
}

#[cfg(feature = "serialization-protobuf")]
fn build_protobuf(out_dir: &str, input: &[&str], includes: &[&str]) {
    protoc_rust::Codegen::new()
        .out_dir(out_dir)
        .inputs(input)
        .includes(includes)
        .run()
        .expect("Running protoc failed");
}

#[cfg(feature = "serialization-protobuf")]
fn build_protobuf_schemata() {
    use std::fs::File;
    use std::io::Read;
    let mut version_string = String::new();
    let mut version_pin =
        File::open("PROTOC_VERSION").expect("protoc version pin `PROTOC_VERSION` file is missing");

    version_pin
        .read_to_string(&mut version_string)
        .expect("cannot read protoc pin file");

    if !has_right_protoc_version(&version_string) {
        eprintln!(
            "Build failed because merkle.rs could not find protobuf version {}",
            version_string
        );

        std::process::exit(1);
    }

    build_protobuf("src/proto", &["protobuf/proof.proto"], &[]);
}

fn main() {
    #[cfg(feature = "serialization-protobuf")]
    build_protobuf_schemata();
}
