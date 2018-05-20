extern crate protoc_rust;

fn build_protobuf<'a>(out_dir: &'a str, input: &'a [&'a str], includes: &'a [&'a str]) {
    use self::protoc_rust::{run, Args};
    run(Args {
        out_dir,
        input,
        includes,
    }).expect("protoc");
}

fn main() {
    #[cfg(feature = "serialization-protobuf")]
    build_protobuf("src/proto", &["protobuf/proof.proto"], &[]);
}
