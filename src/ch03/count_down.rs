/// 运行 `cargo test --  --nocapture` 命令，这样在控制台中可看到打印的信息。
pub fn count_down(i: i32) {
    println!("{}", i);

    // 基线条件
    if i <= 1 {
        return;
    }
    // 递归条件
    count_down(i - 1);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn count_down_success() {
        count_down(3);
    }

}