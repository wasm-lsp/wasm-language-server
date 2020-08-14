//! Cargo xtask definitions for the wasm-language-server project.

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

fn main() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let app = clap::App::new("xtask").subcommands(vec![
        clap::SubCommand::with_name("check"),
        clap::SubCommand::with_name("clippy"),
        clap::SubCommand::with_name("doc"),
        clap::SubCommand::with_name("format"),
        clap::SubCommand::with_name("init").arg(
            clap::Arg::with_name("with-corpus")
                .long("with-corpus")
                .help("Initialize corpus submodules"),
        ),
        clap::SubCommand::with_name("install").arg(
            clap::Arg::with_name("rebuild-parsers")
                .long("rebuild-parsers")
                .help("Rebuild tree-sitter parsers if necessary"),
        ),
        clap::SubCommand::with_name("test"),
    ]);

    let matches = app.get_matches_safe()?;

    match matches.subcommand_name() {
        Some("check") => subcommand::cargo::check()?,
        Some("clippy") => subcommand::cargo::clippy()?,
        Some("doc") => subcommand::cargo::doc()?,
        Some("format") => subcommand::cargo::format()?,
        Some("init") => subcommand::init(&matches)?,
        Some("install") => subcommand::cargo::install(&matches)?,
        Some("test") => subcommand::cargo::test()?,
        _ => {},
    }

    Ok(())
}

mod metadata {
    use std::path::{Path, PathBuf};

    pub fn cargo() -> anyhow::Result<String> {
        // NOTE: we use the cargo wrapper rather than the binary reported through the "CARGO" environment
        // variable because we need to be able to invoke cargo with different toolchains (e.g., +nightly)
        Ok(String::from("cargo"))
    }

    pub fn project_root() -> PathBuf {
        Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .unwrap()
            .to_path_buf()
    }
}

mod subcommand {
    pub mod cargo {
        use crate::metadata;
        use std::{
            fs,
            path::PathBuf,
            process::{Command, Stdio},
        };

        // Run `cargo check` with custom options.
        pub fn check() -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(&[
                "check",
                "--all-features",
                "--all-targets",
                "--benches",
                "--bins",
                "--examples",
                "--tests",
                "--workspace",
            ]);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo clippy` with custom options.
        pub fn clippy() -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&[
                "clippy",
                "--all-features",
                "--all-targets",
                "--benches",
                "--bins",
                "--examples",
                "--tests",
                "--workspace",
                "--",
                "-D",
                "warnings",
            ]);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo doc` with custom options.
        pub fn doc() -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "doc", "--all-features", "--no-deps"]);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo format` with custom options.
        pub fn format() -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["+nightly", "fmt", "--all"]);
            cmd.status()?;
            Ok(())
        }

        // Run `cargo install` with custom options.
        pub fn install(matches: &clap::ArgMatches) -> anyhow::Result<()> {
            if let Some(matches) = matches.subcommand_matches("install") {
                if matches.is_present("rebuild-parsers") {
                    // Configure the project root path.
                    let root_path = metadata::project_root();
                    let root_path = root_path.to_str().unwrap();

                    // Configure the tree-sitter directory path.
                    let tree_sitter_path = [root_path, "vendor", "tree-sitter-wasm"].iter().collect::<PathBuf>();
                    let tree_sitter_path = tree_sitter_path.to_str().unwrap();

                    // Configure the tree-sitter cli binary path.
                    let tree_sitter_cli_path = [tree_sitter_path, "node_modules", ".bin", "tree-sitter"]
                        .iter()
                        .collect::<PathBuf>();
                    let tree_sitter_cli_path = tree_sitter_cli_path.to_str().unwrap();

                    // Check if the tree-sitter cli binary is available.
                    let mut cmd;
                    if cfg!(target_os = "windows") {
                        cmd = Command::new("cmd");
                        cmd.args(&["/C", format!("{} --help", tree_sitter_cli_path).as_ref()]);
                    } else {
                        cmd = Command::new("sh");
                        cmd.args(&["-c", format!("{} --help", tree_sitter_cli_path).as_ref()]);
                    };
                    cmd.stdout(Stdio::null());
                    cmd.stderr(Stdio::null());

                    // Run `npm ci` first if `tree-sitter` binary is not available.
                    if !cmd.status()?.success() {
                        log::info!("installing tree-sitter toolchain");
                        let mut cmd;
                        if cfg!(target_os = "windows") {
                            cmd = Command::new("cmd");
                            cmd.args(&["/C", "npm ci"]);
                        } else {
                            cmd = Command::new("sh");
                            cmd.args(&["-c", "npm ci"]);
                        }
                        cmd.current_dir(tree_sitter_path);
                        cmd.stdout(Stdio::null());
                        cmd.stderr(Stdio::null());
                        cmd.status()?;
                    }

                    // Iterate through the different grammar types.
                    for grammar in &["wast", "wat", "wit", "witx"] {
                        // Configure the grammar directory path.
                        let grammar_path = [tree_sitter_path, grammar].iter().collect::<PathBuf>();
                        let grammar_path = dunce::canonicalize(grammar_path)?;
                        let grammar_path = grammar_path.to_str().unwrap();

                        // Configure the grammar.js tree-sitter grammar definition path.
                        let js_path = [grammar_path, "grammar.js"].iter().collect::<PathBuf>();
                        let js_date = fs::metadata(js_path)?.modified()?;

                        // Configure the parser.c tree-sitter generated parser path.
                        let cc_path = [grammar_path, "src", "parser.c"].iter().collect::<PathBuf>();
                        let cc_date = fs::metadata(cc_path)?.modified()?;

                        // Check if the modification date on the grammar definition is newer than the generated parser.
                        // If it is, we want to regenerate the parser from the updated grammar.
                        if cc_date.duration_since(js_date).is_err() {
                            log::info!("regenerating parser: {}", grammar);
                            let commands = format!("cd {} && {} generate", grammar_path, tree_sitter_cli_path);
                            let mut cmd;
                            if cfg!(target_os = "windows") {
                                cmd = Command::new("cmd");
                                cmd.args(&["/C", commands.as_ref()]);
                            } else {
                                cmd = Command::new("sh");
                                cmd.args(&["-c", commands.as_ref()]);
                            }
                            cmd.status()?;
                        }
                    }
                }
            }

            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.args(&["install", "--path", "crates/server", "--offline"]);
            cmd.status()?;

            Ok(())
        }

        // Run `cargo test` with custom options.
        pub fn test() -> anyhow::Result<()> {
            let cargo = metadata::cargo()?;
            let mut cmd = Command::new(cargo);
            cmd.current_dir(metadata::project_root());
            cmd.env("RUSTFLAGS", "-Dwarnings");
            cmd.args(&[
                "test",
                "--all-features",
                "--all-targets",
                "--benches",
                "--bins",
                "--examples",
                "--tests",
                "--workspace",
            ]);
            cmd.status()?;
            Ok(())
        }
    }

    use crate::metadata;
    use std::{
        path::{Path, PathBuf},
        process::Command,
    };

    // Initialize submodules (e.g., for tree-sitter grammars and test suites)
    pub fn init(matches: &clap::ArgMatches) -> anyhow::Result<()> {
        if let Some(matches) = matches.subcommand_matches("init") {
            // initialize "vendor/tree-sitter-wasm" submodule
            let submodule = Path::new("vendor/tree-sitter-wasm").to_str().unwrap();
            let mut cmd = Command::new("git");
            cmd.current_dir(metadata::project_root());
            cmd.args(&["submodule", "update", "--init", "--depth", "1", "--", submodule]);
            cmd.status()?;

            if matches.is_present("with-corpus") {
                // initialize "vendor/corpus" submodule
                let submodule = Path::new("vendor/corpus").to_str().unwrap();
                let mut cmd = Command::new("git");
                cmd.current_dir(metadata::project_root());
                cmd.args(&["submodule", "update", "--init", "--depth", "1", "--", submodule]);
                cmd.status()?;

                // initialize "vendor/corpus/..." submodules
                let mut cmd = Command::new("git");
                let root = metadata::project_root();
                let root = root.to_str().unwrap();
                let path = [root, "vendor", "corpus"].iter().collect::<PathBuf>();
                cmd.current_dir(path);
                cmd.args(&["submodule", "update", "--init", "--depth", "1"]);
                cmd.status()?;
            }
        }

        Ok(())
    }
}