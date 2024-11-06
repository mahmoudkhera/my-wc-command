
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead,BufReader};


pub  struct Config{
    pub flag:String,
    pub file_path:String,

}

impl Config {

    pub fn build(
        mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        args.next();

        let flag= match args.next() {

            Some(arg) =>arg,
            None => return Err(" inter  flag"),
            
        };
        
        let file_path= match args.next() {

            Some(arg)=>arg,
            None=>{
                if flag.chars().next()==Some('-'){
                    return Err("The system cannot find the file");
                }
                String::from(" ")
            },
            
        };

    Ok(Config{flag,file_path})

    }
    
}

pub fn  run(config :Config) -> Result<(), Box<dyn Error>>{

     let (file_path,flag)=if config.flag.chars().next()!=Some('-') {
        
         (config.flag,String::from("-lwc"))
     }
     else{
        (config.file_path,config.flag)
     };


   let file =fs::File::open(file_path)?;
   let reader: BufReader<File>=BufReader::new(file);


   let flag:&str=&flag;
   
   
   match flag {

    "-lwc" =>{
        let (l,w,c)=l_w_c(reader);
        println!("{} {} {}",l,w,c);
    },
    "-l" => println!("{}",l_comand(reader)),
    "-m" =>println!("{}",m_command(reader)),
    "-w" =>println!("{}",w_command(reader)),
    "-c"=>println!("{}",c_command(reader)),
    _ =>println!("undifend command ")
       
   }






   Ok(())


}


 
// count lines ,words and bytes
pub fn l_w_c(reader : BufReader<File>)->(usize,usize,usize){


      let mut line_count = 0;
      let mut word_count = 0;
      let mut byte_count = 0;
  
      // Process each line in the file
      for line in reader.lines() {
          let line = line.unwrap();
          line_count += 1;
          word_count += line.split_whitespace().count();
          byte_count += line.len();
      }

      (line_count,word_count,byte_count)
    

    

}

// count lines
pub fn l_comand(reader : BufReader<File>) ->usize{

    reader.lines().count()

    
}
//Counts characters
pub fn m_command(reader : BufReader<File>) ->usize{
   

    let mut chars=0;
    for line in reader.lines() {
        let line = line.unwrap();
        chars += line.chars().count();
    }

    chars

    
}
//count words 
pub fn w_command(reader : BufReader<File>) ->usize{
   

    let mut words=0;
    for line in reader.lines() {
        let line = line.unwrap();
        words += line.split_whitespace().count();
    }

    words

    
}
//count bytes
pub fn c_command(reader : BufReader<File>) ->usize{
   

    let mut bytes=0;
    for line in reader.lines() {
        let line = line.unwrap();
        bytes += line.len();
    }

    bytes

    
}



# [cfg(test)]

mod tests{

    use super::*;


    #[test]
    fn test_lwc(){
    let file =fs::File::open("code.txt").unwrap();
    let reader: BufReader<File>=BufReader::new(file);
        
        assert_eq!((1,2,20),l_w_c(reader));

    }

}