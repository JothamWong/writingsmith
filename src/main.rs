use clap::Parser;

mod wsmodules;
use wsmodules::*;

mod structs;
use structs::{Message, Module};

#[derive(Parser)]
struct Cli {
    /// Path to file to read
    path: std::path::PathBuf,
    /// Modules to use
    #[arg(short, long, value_delimiter = ',')]
    modules: Option<Vec<String>>,
    // TODO: Think about formatting flags
}

/// Takes in the messages and outputs the report
fn generate_report(messages: Vec<Message>) {
    if messages.is_empty() {
        println!("No issues found!");
        return;
    }

    println!("Found {} issue(s)", messages.len());

    // TODO: Specify formatting here
    // For now, we simply print line by line
    let mut sorted_messages = messages;
    sorted_messages.sort_by_key(|m| m.line_no);

    for m in sorted_messages {
        println!("Line {}: {}", m.line_no, m.message);
    }
}

fn run_module(name: &str, filepath: &str) -> Option<Vec<Message>> {
    match name {
        "weasel" => Some(WeaselModule::check(filepath)),
        _ => None,
    }
}

fn main() {
    let args = Cli::parse();
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let filepath = args.path.to_str().unwrap();
    // Should check for modules to use here
    let mut messages: Vec<Message> = Vec::new();
    let all_modules = vec!["weasel".to_string()];

    let modules_to_run = args.modules.unwrap_or(all_modules.clone());
    for module_name in modules_to_run {
        match run_module(&module_name, filepath) {
            Some(mut module_messages) => messages.extend(module_messages),
            None => eprint!("Warning: module {} not found", module_name),
        }
    }

    generate_report(messages);
}




