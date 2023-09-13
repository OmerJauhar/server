use clap::Parser;


#[derive(Parser)]
#[derive(Debug,Default)]
#[clap(author = "Omer Jauhar", version = "0.1", about)]
/// CLI tool for accessing the server

struct CliDataType
{
    // #[clap{short,long}
    #[arg(short = 'u',long)]
    id: Option<String> ,


    // #[clap{short,long,forbid_empty_values= true}] 
    password: Option<String>
}

fn main() {
    let _args = CliDataType::parse();

    println!("{:?}",_args);
    // match &_args.id
    // {
    //     Some(a) => {
    //         // println!("The value of String is :{}",a);
    //     }
    //     None =>
    //     {
    //         // println!("No value found ");
    //     }
    // }
}
