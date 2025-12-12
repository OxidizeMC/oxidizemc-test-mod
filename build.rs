use oxidizemc_fabric::build::FABRICMC_MOD_LOADER;
use oxidizemc::build::OxidizeMCBuild;

fn main() {
    OxidizeMCBuild::new()
        .java_package("com.github.pitchblacknights.oxidizemc")
        .mod_loader(FABRICMC_MOD_LOADER)
        .finish();
}
