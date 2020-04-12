/// 运行 `cargo test --  --nocapture` 命令，这样在控制台中可看到打印的信息。

fn greet2(name: &str) {
    println!("how are you, {}?", name);
}

fn bye() {
    println!("ok bye!");
}

pub fn greet(name: &str) {
    println!("hello, {}!", name);
    greet2(name);
    println!("getting ready to say bye...");
    bye();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn count_down_success() {
        greet("adit");
    }

}