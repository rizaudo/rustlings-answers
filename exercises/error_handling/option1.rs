// option1.rs
// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Execute `rustlings hint option1` for hints :)



pub fn pop_too_much() -> Option<()> {
    let mut list = vec![3];

    let last = list.pop();
    match last {
        None => None,
        Some(i) => Some(println!("The last item in the list is {:?}", i)),
    };
    let second_to_last = list.pop();
    let result = match second_to_last {
        Some(i) => Some(println!(
            "The second-to-last item in the list is {:?}",
            second_to_last
        )),
        None => None,
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic() {
        // Yes, pop_too_much() always return None.
        assert_eq!(pop_too_much(), None);
    }
}
