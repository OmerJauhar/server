use cursive::{self, views::TextView};
fn main() {
    println!("Meow, world!");
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(TextView::new("OJ's Server"));
    // siv.add_layer(TextView::new("Welcome to meow server press q to quit"));
    
    siv.run() ; 
}
