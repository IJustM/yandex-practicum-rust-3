fn main() -> anyhow::Result<()> {
    // Пересобрать при изменении файла
    println!("cargo:rerun-if-changed=../proto/blog.proto");

    tonic_prost_build::configure()
        .build_server(true)
        .compile_protos(&["../proto/blog.proto"], &["../proto"])?;

    Ok(())
}
