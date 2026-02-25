fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../../proto/blog.proto");
    println!("cargo:rerun-if-changed=.env");

    tonic_prost_build::configure()
        .compile_well_known_types(true)
        .extern_path(".google.protobuf.Timestamp", "::pbjson_types::Timestamp")
        .extern_path(".google.protobuf.Empty", "::pbjson_types::Empty")
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["../../proto/blog.proto"], &["../../proto"])?;

    Ok(())
}
