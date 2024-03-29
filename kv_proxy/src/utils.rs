use log::error;

pub fn exit_with_msg(msg: &str) -> ! {
    error!("{msg}");
    std::process::exit(1);
}
