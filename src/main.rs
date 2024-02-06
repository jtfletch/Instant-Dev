mod brew;
mod packages;

fn main() {
    brew::install_homebrew();
    packages::packages();
}
