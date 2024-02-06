mod brew;
mod config {
    pub mod git;
}
mod packages;

fn main() {
    let verbose = false;
    brew::install_homebrew();
    packages::packages();
    config::git::configure_git(verbose)
}
