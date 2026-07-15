pub(crate) fn env() {
    let env_vars = std::env::vars();
    for (var, val) in env_vars {
        println!("{}={}", var, val);
    }
}