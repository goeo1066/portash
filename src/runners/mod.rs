use std::io::Write;

use crate::{configs::PortashConfig, parsers::parse_prompt, runners::command::run_command};

mod command;

pub struct PortashRunner {
    continued: bool,
    input_buffer: String,
    config: PortashConfig,
}

impl PortashRunner {
    pub fn load(is_prod: bool) -> Result<PortashRunner, String> {
        if let Ok(config) = Self::load_config(is_prod) {
            let mut runner = Self::default();
            runner.config = config;
            return Ok(runner);
        } else {
            return Err("Failed to load config".into());
        }
    }

    pub fn load_config(is_prod: bool) -> Result<PortashConfig, String> {
        if is_prod {
            PortashConfig::load_from_home()
        } else {
            PortashConfig::load_from_home_or_current()
        }
    }

    pub fn run_shell(&mut self) -> Result<(), String> {
        let prompt_header: String = self.config.get_prompt_header().into();
        if let Ok(()) = ctrlc::set_handler(move || {
            println!("[^C] is not supported");
            print_header(&prompt_header);
        }) {
        } else {
            eprintln!("Failed at setting ctrlc");
        }

        self.print_msg_on_login();
        loop {
            self.print_header();
            self.get_next_line();
            if self.continued {
                continue;
            }

            let command = self.get_prompt_lines();

            if Self::is_prompt_exit(&command) {
                break;
            }

            let prompts = self.get_prompts_from_command(&command);
            if let Some(prompts) = prompts {
                self.run_command(&prompts);
            } else {
                self.print_msg_on_not_allowed_command();
            }

            self.reset_prompt_lines();
        }

        Ok(())
    }

    fn get_prompts_from_command(&self, command: &Vec<String>) -> Option<Vec<String>> {
        let program = &command[0];
        for e in self.config.get_config_command_list() {
            if e.is_command_equal(&program) {
                return Some(e.get_execute());
            } else {
                continue;
            }
        }
        None
    }

    fn print_msg_on_login(&self) {
        println!("{}", self.config.get_config_general().msg_on_login)
    }

    fn print_msg_on_not_allowed_command(&self) {
        let config_general = self.config.get_config_general();
        println!("{}", config_general.msg_on_not_allowed_command);
    }

    fn run_command(&mut self, prompts: &Vec<String>) {
        run_command(prompts);
    }

    fn is_prompt_exit(prompts: &Vec<String>) -> bool {
        prompts.len() == 1 && prompts[0].eq("exit".into())
    }

    fn get_prompt_lines(&mut self) -> Vec<String> {
        parse_prompt(&self.input_buffer)
    }

    fn reset_prompt_lines(&mut self) {
        self.input_buffer = String::new();
    }

    fn get_next_line(&mut self) -> bool {
        if let Ok(_) = std::io::stdin().read_line(&mut self.input_buffer) {}
        let continued = self.input_buffer.ends_with("\\\n");
        self.continued = continued;
        continued
    }

    fn print_header(&self) {
        let header = if self.continued {
            self.config.get_chained_header()
        } else {
            self.config.get_prompt_header()
        };
        print_header(header);
    }
}

fn print_header(prompt_header: &str) {
    print!("{} ", prompt_header);
    if let Ok(_) = std::io::stdout().flush() {}
}

impl Default for PortashRunner {
    fn default() -> Self {
        Self {
            continued: false,
            input_buffer: String::default(),
            config: PortashConfig::default(),
        }
    }
}
