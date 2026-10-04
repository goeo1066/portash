use std::process::{Command, Stdio};

pub fn run_command(args: &Vec<String>) {
    if args.is_empty() {
        return;
    } else {
        let mut command = Command::new(&args[0]);
        command.stdin(Stdio::inherit()).stdout(Stdio::inherit());

        for (idx, arg) in args.into_iter().enumerate() {
            if idx == 0 {
                continue;
            }

            command.arg(arg);
        }

        match command.output() {
            Ok(_) => {}
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}
