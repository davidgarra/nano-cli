use std::io::{self, stdin, stdout, Write};

fn main() -> io::Result<()> {
    println!(" ##### NANO-CLI #####");
    loop {
        let input = get_user_input()?;
        let command = match Command::try_from(input) {
            Ok(command) => command,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };
        command.execute()
    }
}

fn get_user_input() -> io::Result<String> {
    let mut input = String::new();
    print!("> ");
    stdout().flush()?;

    stdin().read_line(&mut input)?;
    Ok(input)
}

#[derive(Debug)]
enum CommandType {
    Echo,
}

#[derive(Debug)]
struct Command {
    command_type: CommandType,
    args: Vec<String>,
}

impl Command {
    fn execute(&self) {
        match self.command_type {
            CommandType::Echo => self.echo(),
        }
    }

    fn echo(&self) {
        println!("{}", self.args.get(0).unwrap_or(&"".to_string()))
    }
}

impl TryFrom<String> for Command {
    type Error = String;
    fn try_from(str_command: String) -> Result<Self, Self::Error> {
        let word_list: Vec<String> = str_command
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let Some(name) = word_list.get(0) else {
            return Err("No command found".to_string());
        };

        let command_type = CommandType::try_from(name.to_string())?;

        let args: Vec<String> = word_list[1..].to_vec();

        Ok(Command { command_type, args })
    }
}

impl TryFrom<String> for CommandType {
    type Error = String;
    fn try_from(str_command_type: String) -> Result<Self, Self::Error> {
        match str_command_type.as_str() {
            "echo" => Ok(CommandType::Echo),
            _ => Err(format!("Command {} is not valid", str_command_type)),
        }
    }
}
