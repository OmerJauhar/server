use clap::Parser;
use clap::Subcommand;
#[derive(Parser)]
#[derive(Debug)]
#[clap(author = "Omer Jauhar", version = "0.1", about)]
#[command(propagate_version = true)]
struct CliDataType
{
    #[command(subcommand)]
    command: Commands 
}
#[derive(Debug)]
#[derive(Subcommand)]
// #[derive(Copy)] // copy trait cannot be implemented due to non primitive data types 
enum Commands
{
    Guimode { flag: Option<bool> },
    Climode {
            #[clap{short,long}]
    #[arg(short = 'u',long)]
    id: Option<String> ,

    #[clap{short,long}] 
    password: Option<String>,
    }
}
impl Clone for Commands {
    fn clone(&self) -> Self {
        match self {
            Commands::Guimode { flag } => Commands::Guimode { flag: flag.clone() },
            Commands::Climode { id, password } => Commands::Climode { id: id.clone(), password: password.clone() },
        }
    }
}

fn main() {
    let _args = CliDataType::parse();

    println!("{:?}",_args);
    println!("{:?}",_args.command);
    match  _args.command.clone()
    {
        Commands::Guimode { flag } =>
        {
            if flag == Some(true) 
            {
                println!("inside gui mode")
            }
        }

        Commands::Climode { id, password } =>
        {
            println!("{:?} {:?} ",id , password);
        }
        // Commands::Guimode { flag }
    }
}