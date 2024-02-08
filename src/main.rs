mod brew {
    pub mod brew;
    pub mod packages;
}
mod config {
    pub mod git;
    pub mod go;
}

fn main() {
    let verbose = false;
    brew::brew::install_homebrew();
    brew::packages::packages();
    config::go::configure_go();
    config::git::configure_git(verbose)
}
