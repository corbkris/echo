pub mod basic;
mod basic_test;
pub mod connection;
mod connection_test;
pub mod generic;
mod generic_test;
pub mod table;
pub mod tables;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn multiply(left: usize, right: usize) -> usize {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
