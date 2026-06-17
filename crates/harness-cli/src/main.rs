#![forbid(unsafe_code)]

use harness_core::CoreBoundary;

fn main() {
    println!("{}", placeholder_message());
}

fn placeholder_message() -> &'static str {
    let _core = CoreBoundary::new();
    "harness-cli: implementation skeleton only"
}

#[cfg(test)]
mod tests {
    use super::placeholder_message;

    #[test]
    fn cli_placeholder_is_explicit() {
        assert_eq!(
            placeholder_message(),
            "harness-cli: implementation skeleton only"
        );
    }
}
