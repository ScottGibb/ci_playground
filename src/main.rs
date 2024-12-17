pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn main() {
    let result = add(2, 2);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2)
        assert_eq!(result, 4)
    }

    // #[test]
    // fn it_also_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
