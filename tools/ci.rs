// Copyright 2021 Robin Freyler
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Runs CI scripts locally on the users computer.
//!
//! Developers should run this script at least once before pushing to the
//! Runwell repository.
//!
//! # Usage
//!
//! In order to successfully run this script the user needs to have installed
//! the following programs on their machines:
//!
//! - git
//! - rustup
//! - cargo
//!
//! The scripts will eventually install some other Rust or Cargo components:
//!
//! - rustfmt via rustup
//! - cargo-clippy via rustup
//!
//! # Credits
//!
//! This quality controlling Rust script was heavily inspired by the one
//! used and provided in `stdio-utils` repository authored by Consolero:
//!
//! <https://github.com/consolero/stdio-utils-rs/blob/dev/0.1/tools/src/bin/quality-control.rs>
//!
//! Developers should run this script before pushing their pull requests to
//! the main repository to make sure that the quality of the pull request's
//! edits are in the realm of what the project accepts.

use std::{env::set_var, process::Command};

fn main() {
    git(["--version"]);
    rustup(["--version"]);
    cargo(["--version"]);

    // Add required rustup components.
    rustup(["+nightly", "component", "add", "rustfmt"]);
    rustup(["+nightly", "component", "add", "clippy"]);

    // Check if the entire workspace can be compiled under different configurations.
    cargo(["--locked", "check", "--workspace"]);
    cargo(["--locked", "check", "--workspace", "--no-default-features"]);
    cargo([
        "--locked",
        "check",
        "--workspace",
        "--no-default-features",
        "--features=alloc",
    ]);
    cargo([
        "--locked",
        "check",
        "--workspace",
        "--no-default-features",
        "--features=derive",
    ]);
    cargo([
        "--locked",
        "check",
        "--workspace",
        "--no-default-features",
        "--features=alloc,derive",
    ]);
    cargo(["--locked", "check", "--workspace", "--all-features"]);

    // Check formatting of the entire workspace.
    cargo(["--locked", "fmt", "--all", "--", "--check"]);

    // Check test suite of the entire workspace under different configurations.
    cargo(["--locked", "test", "--workspace"]);
    cargo(["--locked", "test", "--workspace", "--no-default-features"]);
    cargo([
        "--locked",
        "test",
        "--workspace",
        "--no-default-features",
        "--features=alloc",
    ]);
    cargo([
        "--locked",
        "test",
        "--workspace",
        "--no-default-features",
        "--features=derive",
    ]);
    cargo([
        "--locked",
        "test",
        "--workspace",
        "--no-default-features",
        "--features=alloc,derive",
    ]);
    cargo(["--locked", "test", "--workspace", "--all-features"]);

    // Lint the entire workspace under different configurations.
    cargo([
        "+nightly",
        "--locked",
        "clippy",
        "--workspace",
        "--",
        "-Dwarnings",
    ]);
    cargo([
        "+nightly",
        "--locked",
        "clippy",
        "--workspace",
        "--no-default-features",
        "--",
        "-Dwarnings",
    ]);
    cargo([
        "+nightly",
        "--locked",
        "clippy",
        "--workspace",
        "--no-default-features",
        "--features=alloc",
        "--",
        "-Dwarnings",
    ]);
    cargo([
        "+nightly",
        "--locked",
        "clippy",
        "--workspace",
        "--no-default-features",
        "--features=derive",
        "--",
        "-Dwarnings",
    ]);
    cargo([
        "+nightly",
        "--locked",
        "clippy",
        "--workspace",
        "--no-default-features",
        "--features=alloc,derive",
        "--",
        "-Dwarnings",
    ]);
    cargo([
        "+nightly",
        "--locked",
        "clippy",
        "--workspace",
        "--all-features",
        "--",
        "-Dwarnings",
    ]);

    // Test if documentation of the entire workspace builds properly.
    // This is especially useful to detect broken intra doc links.
    //
    // We have to set `RUSTDOCFLAGS` to `-Dwarnings` so that the tool
    // errors instead of just warns.
    set_var("RUSTDOCFLAGS", "-Dwarnings");
    cargo([
        "--locked",
        "doc",
        "--workspace",
        "--no-deps",
        "--document-private-items",
    ]);
}

/// Invokes the `rustup` command with the provided arguments.
///
/// Exits the process upon errors.
fn rustup<'a, T>(args: T)
where
    T: IntoIterator<Item = &'a str>,
{
    call("rustup", args)
}

/// Invokes the `cargo` command with the provided arguments.
///
/// Exits the process upon errors.
fn cargo<'a, T>(args: T)
where
    T: IntoIterator<Item = &'a str>,
{
    call("cargo", args)
}

/// Invokes the `git` command with the provided arguments.
///
/// Exits the process upon errors.
fn git<'a, T>(args: T)
where
    T: IntoIterator<Item = &'a str>,
{
    call("git", args)
}

/// Invokes the given command with the provided arguments.
///
/// Exits the process upon errors.
fn call<'a, T>(proc: &str, args: T)
where
    T: IntoIterator<Item = &'a str>,
{
    let args = args.into_iter().collect::<Vec<_>>();
    println!("Run: {} {}", proc, args.join(" "));
    let status = Command::new(proc).args(&args).status().unwrap_or_else(|_| {
        panic!("Failed to execute: {} {}", proc, args.join(" "))
    });
    match status.code() {
        Some(0) => (),
        Some(e) => std::process::exit(e),
        None => std::process::exit(1),
    }
}
