#[derive(Debug)]
struct Command {
    name: String,
    run_async: bool,
}

fn main() {
    let command1 = Command {
        name: "wget".into(),
        run_async: true,
    };
    // let command2 = Command {
    //     name: command1.name,
    //     run_async: false,
    // };
    println!("{:?}", command1);
}
