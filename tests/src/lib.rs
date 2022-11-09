use std::fs;
use sword_proto_parser::{parse};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_proto() {
        let file = fs::read_to_string("./proto.test.ts").unwrap();
        println(&file)
    }
}