mod libs;
use std::env;
use std::process;

use libs::{run,Config};
fn main() {
    

    let args=env::args();
    let config=Config::build(args).unwrap_or_else(|err|{
        println!("{err}");
        process::exit(1);

    });

    if let Err(e) =run(config){
        println!("{e}");
        process::exit(1);
    }


}
