fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    let bytes = input.as_bytes();
    let mut first_nw = 0;
    let mut last_nw = input.len();
    for (i, &item) in bytes.iter().enumerate() {
        if item != b' '{
            first_nw = i;
            break;
        }
    }
    for (i, &item) in bytes.iter().enumerate().rev() {
        if item != b' '{
            last_nw = i;
            break;
        }
    }
    &input[first_nw..last_nw + 1]
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    String::from(input) + " world!"
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
    // TODO: Replace "cars" in the string with "balloons".
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
