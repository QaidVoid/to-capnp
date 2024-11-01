fn main() {
    ::capnpc::CompilerCommand::new()
        .output_path("src/")
        .file("schema.capnp")
        .run()
        .expect("capnp compiles");
}
