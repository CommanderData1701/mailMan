use clap::Parser;
use parser::{Cli, Commands};

mod parser;
mod subcommands;

pub fn run() -> Result<String, (String, i32)> {
    let args = Cli::parse();

    match args.command {
        Commands::Authenticate {password} => 
            subcommands::authenticate(password),
        Commands::Reload => subcommands::reload(),
        Commands::Initialize => subcommands::initialize(),
        Commands::RemovePassword {accountname} => 
            subcommands::remove_password(accountname),
        Commands::AddCredentials {accountname, username, env} =>
            subcommands::add_credentials(accountname, username, env),
    }
}
