use crate::cli;
use color_eyre::eyre::Result;
use duct::cmd;

/// Create the release binaries for all the supported architectures.
pub fn release(args: &cli::ReleaseArgs) -> Result<()> {
    println!("Building release binaries");
    if !args.no_apple_x86_64 {
        println!("Building for x86_64");
        cmd!(
            "cargo",
            "build",
            "--bin",
            &args.binary,
            "--target",
            "x86_64-apple-darwin",
            "--release"
        )
        .run()?;
    }

    if !args.no_apple_silicon {
        println!("Building for Apple Silicon");
        cmd!(
            "cargo",
            "build",
            "--bin",
            &args.binary,
            "--target",
            "aarch64-apple-darwin",
            "--release"
        )
        .run()?;
    }

    if !args.no_linux_aarch64 {
        // println!("Building for x86_64 Linux");
        // cmd!(
        //     "cargo",
        //     "build",
        //     "--bin",
        //     &args.binary,
        //     "--target",
        //     "aarch64-unknown-linux-gnu"
        // )
        // .run()?;
        println!("[WIP] Building for AAarch64 Linux");
    }

    Ok(())
}

pub fn install(args: &cli::InstallArgs) -> Result<()> {
    release(&cli::ReleaseArgs {
        binary: args.name.clone(),
        no_apple_x86_64: true,
        no_apple_silicon: false,
        no_linux_aarch64: true,
    })?;

    let target_path = "target/aarch64-apple-darwin/release/".to_string() + &args.name;

    cmd!("cp", &target_path, &args.path).run()?;
    cmd!("chmod", "+x", &args.path).run()?;

    Ok(())
}
