/// 运行 `cargo test --  --nocapture` 命令，这样在控制台中可看到打印的信息。

pub fn fact(x: i32) -> i32 {
    if x == 1 {
        return 1
    }
    x * fact(x - 1)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn count_down_success() {
        println!("{}", fact(5));
    }

}