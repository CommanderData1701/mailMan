use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = NAME)]
#[command(about = ABOUT, long_about = LONG_ABOUT)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Unlock mailmans keychain")]
    Authenticate {
        #[arg(short, long, required = false,  help = "The password to unlock the keychain")]
        password: Option<String>,
    },

    #[command(about = "Reload mrules in .mailman directory")]
    Reload,

    #[command(about = "Initialize the keychain")]
    Initialize,

    #[command(about = "Remove a password from the keychain")]
    RemovePassword {
        #[arg(short, long, help = "Identifier of the account that's supposed to be deleted.")]
        accountname: String,
    },

    #[command(about = "Add credentials for an accout")]
    AddCredentials {
        #[arg(short, long, required = false, help = "Account identifier")]
        accountname: Option<String>,

        #[arg(long, required = false, help = "Store as environment variable. Environment variables don't have to be authenticated")]
        env: bool,

        #[arg(short, long, required = false, help = "Username for the mail account")]
        username: Option<String>,
    }
}


const NAME: &str = "mailmanctl";
const ABOUT: &str = "mailmanctl - The mailman command line tool";
const LONG_ABOUT: &str = "mailmanctl is the cli tool to interact with the mailman filtering service.";
