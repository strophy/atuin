use std::process::Command;
fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let proto_paths = [
        format!("{manifest_dir}/proto/history.proto"),
        format!("{manifest_dir}/proto/search.proto"),
        format!("{manifest_dir}/proto/control.proto"),
    ];
    let proto_include_dirs = [format!("{manifest_dir}/proto")];

    let file_descriptors = protox::compile(
        proto_paths.iter().map(String::as_str),
        proto_include_dirs.iter().map(String::as_str),
    ).unwrap();

    let output = Command::new("git").args(["rev-parse", "HEAD"]).output();

    let sha = match output {
        Ok(sha) => String::from_utf8(sha.stdout).unwrap(),
        Err(_) => String::from("NO_GIT"),
    };

    println!("cargo:rustc-env=GIT_HASH={sha}");
}
