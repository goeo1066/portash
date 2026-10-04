use crate::runners::PortashRunner;

mod configs;
mod parsers;
mod runners;

fn main() {
    // if let Ok(()) = ctrlc::set_handler(|| {}) {
    // } else {
    //     eprintln!("Failed at setting ctrlc");
    // }
    if let Ok(portash_runner) = &mut PortashRunner::load(false) {
        if let Ok(()) = portash_runner.run_shell() {
            println!("bye!");
        } else {
            println!("Couldn't launch shell");
        }
    } else {
        println!("Couldn't load config");
    }
}
