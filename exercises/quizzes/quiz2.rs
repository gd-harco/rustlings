// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

fn up(input: String) -> String {
    return input.to_uppercase()
}

fn trim(input: String) -> String {
    input.trim().to_string()
}

fn append(mut input: String, time: usize) -> String {
    let mut cur = 0;
    while cur < time {
        input.push_str("bar");
        cur+= 1;
    }
    input
}

mod my_module {
    use crate::{append, trim, up};

    use super::Command;

    // TODO: Complete the function.
    // pub fn transformer(input: ???) -> ??? { ??? }
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut result = Vec::new();
        for elem in input {
            match elem.1 {
                Command::Uppercase => result.push(up(elem.0)),
                Command::Append(time) => result.push(append(elem.0, time)),
                Command::Trim => result.push(trim(elem.0))
            };
        }
        result
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {

    // TODO: What do we need to import to have `transformer` in scope?
    // use self::transformer;
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
