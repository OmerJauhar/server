use clap::Parser;


#[derive(Parser)]
#[derive(Debug,Default)]
struct CliDataType
{
    id: String , 
    password: String
}

impl CliDataType
{
    fn new(a:String,b:String)-> CliDataType
    {
        CliDataType { id: a, password: b }
    }

    fn new_gui(a:String) -> CliDataType
    {
        CliDataType { id: a , password :Default::default()}
    } 
}
fn main() {
    let _args = CliDataType::parse();

    println!("{:?}",_args);
}
