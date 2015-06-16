use std::io::{Write, BufRead};

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    stdout.lock().write("Welcome to arden :-)\n".as_bytes()).unwrap();
    loop {
        let res = prompt_user(&stdin, &stdout);
        stdout.lock().write(format!("You wrote: {}\n", res).as_bytes()).unwrap();
    }
}

fn prompt_user(stdin: &std::io::Stdin, stdout: &std::io::Stdout) -> std::string::String {
    let prompt = "=> ".to_string();
    stdout.lock().write(prompt.as_bytes()).unwrap();
    stdout.lock().flush().unwrap();

    stdin.lock().lines().next().unwrap().unwrap()
}
