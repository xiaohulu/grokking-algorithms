use std::collections::HashMap;

pub fn check_voter(voted: &mut HashMap<&'static str, bool>, name: &'static str) {
    if voted.get(name).is_some() {
        println!("kick tem out!");
    } else {
        voted.insert(name, true);
        println!("let tem vote");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_voter_success() {
        let mut voted: HashMap<&'static str, bool> = HashMap::new();

        check_voter(&mut voted, "tom");
        check_voter(&mut voted, "mike");
        check_voter(&mut voted, "mike");
    }

}