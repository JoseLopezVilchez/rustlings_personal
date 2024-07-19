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

mod my_module {
    use super::Command;

    // TODO: Complete the function.
    pub fn transformer(input: Vec<(&str, Command)>) -> Vec<String> {
        let mut retorno = Vec::new();

        for (texto, comando) in input {
            let texto_transformado = match comando {
                Command::Uppercase => texto.to_string().to_uppercase(),
                Command::Trim => texto.to_string().trim().to_string() ,
                Command::Append(n) => {
                    let mut texto = texto.to_string();
                    for _ in 0..n {
                        texto += "bar";
                    }
                    texto
                }
            };
            retorno.push(texto_transformado);
        }

        retorno
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello", Command::Uppercase),
            (" all roads lead to rome! ", Command::Trim),
            ("foo", Command::Append(1)),
            ("bar", Command::Append(5)),
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
