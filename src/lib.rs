use java_oxide::Env;
use oxidizemc_fabric::{Logger, LoggerFactory};

fn run(env: Env<'_>) {
    let logger: Logger = LoggerFactory::get_logger(env, "oxidizemc-rust");
    let name: String = logger.get_name();

    logger.info(format!("Hello world! I am {}!", name));
    logger.info("I AM MOD WRITTEN IN 99.99% RUST, RUNNING ON THE FABRIC MODDING API FOR MINECRAFT JAVA ".to_string());

    println!("Normal printing");
    logger.info("Cool!".to_string());

    panic!("Am I panicking like I should be?")
}

#[unsafe(no_mangle)]
extern "system" fn Java_com_github_pitchblacknights_oxidizemc_Natives_run(
    jni_env: Env<'_>,
    _class: *mut (),
) {
    run(jni_env);
}
