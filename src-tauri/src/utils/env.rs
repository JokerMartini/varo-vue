use std::env;

pub fn expand_env_vars(input: &str) -> String {
    let mut output = input.to_string();
    for (key, value) in env::vars() {
        output = output.replace(&format!("${{{}}}", key), &value);
    }
    output
}
