use oxidizemc_fabric::{JLogger, LoggerFactory};
use oxidizemc::entrypoint;

#[entrypoint(ModInitializer)]
fn main() {
    let logger: JLogger = LoggerFactory::get_logger("oxidizemc-rust");
    let name: String = logger.get_name();

    logger.info(format!("Hello world! I am {}!", name));
    logger.info("I AM MOD WRITTEN IN 99.99% RUST, RUNNING ON THE FABRIC MODDING API FOR MINECRAFT JAVA ".to_string());

    println!("Normal printing");
    logger.info("Cool!".to_string());
}

// #[entrypoint(DedicatedServerModInitializer)]
// fn main2() {
//     let logger: JLogger = LoggerFactory::get_logger("oxidizemc-rust-server");
//     logger.info("I'm running on a server".to_string());
// }

// #[entrypoint(ClientModInitializer)]
// fn main3() {
//     let logger: JLogger = LoggerFactory::get_logger("oxidizemc-rust-client");
//     logger.info("I'm running on a client".to_string());
// }
