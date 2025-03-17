fn main() {
    // tonic_build 是 tonic 框架提供的工具，用于处理 Protocol Buffers 文件
    tonic_build::configure()
        .type_attribute(
            ".",
            "#[derive(Hash, Eq, serde::Serialize, serde::Deserialize)]",
        )
        // 设置生成的 Rust 代码的输出目录
        // 这里指定生成的代码将放在 src/pb 目录下
        .out_dir("src/pb")
        // 编译 Protocol Buffers 文件
        // 第一个参数 &["./blockflow.proto"] 指定要编译的 .proto 文件路径
        // 第二个参数 &["./"] 指定在哪些目录下查找 .proto 文件
        // compile_protos() 是新版本推荐使用的方法，替代了旧的 compile()
        .compile_protos(&["proto/blockflow.proto"], &["./proto"])
        .expect("Failed to compile protobuf");

    // 自动格式化生成的代码
    std::process::Command::new("cargo")
        .args(["fmt", "--", "src/pb/blockflow.rs"])
        .status()
        .expect("Failed to format generated code");

    println!("cargo:rerun-if-changed=blockflow.proto"); // 如果 abi.proto 文件发生变化，重新运行构建脚本
    println!("cargo:rerun-if-changed=build.rs"); // 如果 build.rs
    // 文件发生变化，重新运行构建脚本
}
