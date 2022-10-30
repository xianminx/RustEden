# Rust Eden

[//]: # ([![Build Status]&#40;https://travis-ci.org/EdenServer/eden.svg?branch=master&#41;]&#40;https://travis-ci.org/EdenServer/eden&#41;)

[//]: # ([![Build status]&#40;https://ci.appveyor.com/api/projects/status/0q7q7q7q7q7q7q7q?svg=true&#41;]&#40;https://ci.appveyor.com/project/EdenServer/eden&#41;)

[//]: # ([![Discord]&#40;https://img.shields.io/discord/308323056592486420.svg&#41;]&#40;https://discord.gg/0q7q7q7q7q7q7q7q&#41;)

[//]: # ([![License]&#40;https://img.shields.io/badge/license-MIT-blue.svg&#41;]&#40;&#41;)

A Code Repo for Quick Rust Testbed.

## Organization
The code base is organized using Cargo's [**workspace**](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) feature. The workspace is defined in the root `Cargo.toml` file.

## Building
To build the project, run `cargo build` from the root directory. This will build all the crates in the workspace.

Cargo's workspace share the same **/target** folder, which means all submodules of this project share the compilation of the dependent crates. This is a good thing, because it means that the compilation of the dependent crates is only done once, and then shared between all the submodules.

To run a submodule, just go to the submodule and run `cargo build  / cargo run`. This will build the submodule and all its dependencies, and then run the submodule.

The submodule can be either a binary or a library module. If it is a binary module, it will be compiled to a binary file in the **/target/debug** folder. If it is a library module, it will be compiled to a **.rlib** file in the **/target/debug/deps** folder.

To add a testbed submodule, just under the project root directory, `cargo new {module_name} --lib/bin` and write your test or trial code. 


