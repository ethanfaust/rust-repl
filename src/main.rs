use std::io::{self, Write};
use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

#[derive(Eq, Hash, PartialEq)]
enum CommandName {
    Get,
    Set,
    Vars,
    Del
}

// TODO: v2?
trait CommandMetadata {
    fn get_name(&self) -> &str;
    fn regex(&self) -> Option<Regex>;
}
trait CommandImpl {
    fn execute(&self, state: &mut State, input: &str);
}

struct GetCommand {}
impl CommandMetadata for GetCommand {
    fn get_name(&self) -> &str {
        return "get";
    }
    fn regex(&self) -> Option<Regex> {
        return Some(Regex::new(r"get ([a-z0-9_]+)").unwrap());
    }
}

impl CommandImpl for GetCommand {
    fn execute(&self, _state: &mut State, _input: &str) {
        let _name = self.get_name();
        // TODO: finish
        return;
    }
}
// end v2


struct Command {
    name: String,
    has_args: bool,
    regex: Option<Regex>,
    fn_impl: fn(state: &mut State, command: &Command, input: &str)
}

struct ReplCommands {
    commands: HashMap<CommandName, Command>
}

impl ReplCommands {
    pub fn new() -> Self {
        Self {
            commands: HashMap::<CommandName, Command>::new(),
        }
    }

    pub fn add_command(&mut self, name: CommandName, command: Command) {
        self.commands.insert(name, command);
    }
}

struct State {
    vars: HashMap<String, String>
}

impl State {
    pub fn new() -> Self {
        Self {
            vars: HashMap::<String, String>::new()
        }
    }
}

fn init_commands(repl: &mut ReplCommands) {
    let identifier = String::from(r"[a-z0-9_]+");
    let optional_whitespace = String::from(r"[ ]*");
    let value = String::from(r"[a-z0-9_]+");

    repl.add_command(CommandName::Vars, Command {
        name: String::from("vars"),
        has_args: false,
        regex: None,
        fn_impl: cmd_vars
        });

    let get_command_format = format!("get ({})", identifier);
    repl.add_command(CommandName::Get, Command {
        name: String::from("get"),
        has_args: true,
        regex: Some(Regex::new(&get_command_format).unwrap()),
        fn_impl: cmd_get
        });
    let set_command_format = format!("set ({}){}={}({})", identifier, optional_whitespace, optional_whitespace, value);
    repl.add_command(CommandName::Set, Command {
        name: String::from("set"),
        has_args: true,
        regex: Some(Regex::new(&set_command_format).unwrap()),
        fn_impl: cmd_set
        });
    let del_command_format = format!("del ({})", identifier);
    repl.add_command(CommandName::Del, Command {
        name: String::from("del"),
        has_args: true,
        regex: Some(Regex::new(&del_command_format).unwrap()),
        fn_impl: cmd_del
        });
}

fn init_repl() -> ReplCommands {
    let mut repl = ReplCommands::new();
    init_commands(&mut repl);
    return repl;
}

fn main() {
    let repl = init_repl();
    let mut state = State::new();
    let mut line = String::new();
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    loop {
        stdout.write_all(b"> ").expect("write failed");
        stdout.flush().expect("flush failed");
        line.clear();
        stdin.read_line(&mut line).expect("read failed");
        process_command(&repl, &mut state, &line);
    }
}

fn cmd_set(state: &mut State, command: &Command, input: &str) {
    let re = command.regex.as_ref().unwrap();
    let captures = re.captures(input).unwrap();
    let key = captures.get(1).unwrap().as_str().to_string();
    let value = captures.get(2).unwrap().as_str().to_string();
    state.vars.insert(key, value);
}

fn cmd_get(state: &mut State, command: &Command, input: &str) {
    let re = command.regex.as_ref().unwrap();
    let captures = re.captures(input).unwrap();
    let key = captures.get(1).unwrap().as_str();
    if state.vars.contains_key(key) {
        let value = state.vars.get(key).unwrap();
        println!("{} = {}", key, value);
    }
    else {
        println!("no value set for key {}", key);
    }
}

fn cmd_del(state: &mut State, command: &Command, input: &str) {
    let re = command.regex.as_ref().unwrap();
    let captures = re.captures(input).unwrap();
    let key = captures.get(1).unwrap().as_str();
    if state.vars.contains_key(key) {
        state.vars.remove(key);
        println!("removed {}", key);
    }
    else {
        println!("{} was not set so not removed", key);
    }
}

fn cmd_vars(state: &mut State, _command: &Command, _input: &str) {
    if state.vars.len() == 0 {
        println!("(none)");
    }
    for (k, v) in state.vars.iter().sorted_by_key(|x| x.0) {
        println!("{} = {}", k, v);
    }
}

fn process_command(repl: &ReplCommands, state: &mut State, input: &str) {
    if input.trim().len() == 0 {
        return;
    }
    println!("cmd {}", input);

    let mut found: bool = false;

    for (_name, command) in repl.commands.iter() {
        if command.has_args {
            let test_prefix = format!("{} ", command.name);
            if input.starts_with(&test_prefix) {
                (command.fn_impl)(state, &command, input);
                found = true;
                break;
            }
        } else {
            if command.name.eq(input.trim()) {
                (command.fn_impl)(state, &command, input);
                found = true;
                break;
            }
        }
    }
    if !found {
        println!("unrecognized command {}", input);
    }
}
