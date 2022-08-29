# I'm following a Rust tutorial!

The tutorial is here: https://doc.rust-lang.org/book/title-page.html

This is just a repo of me following along with random thoughts and comparisons.

# Setups
I'm on Windows 10 so this is what I did to set things up.

## How to setup Rust
I ran `winget install rustup`, everything was ready.

To make a new project, run `cargo new [project_name]`. This makes a new folder with a `.git/` folder and a `.gitignore` file. I've deleted these for this repo.

To run a project, navigate to the folder and run `cargo run`.

## How to setup C#
I've installed Visual Studio 2022, but there's probably a `winget` command for it.

To make a new project, create a new folder with `New-Item -Type Directory [project_name]`, `Set-Location [project_name]`, then `dotnet new console` and rename the project file.

To run a project, navigate to the folder and run `dotnet run`.