use std::error::Error;

use clap::{command,arg } ; 
use std::io::{ self  ,  BufRead , BufReader} ; 
use std::fs::File ;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config { 
    files : Vec<String>,
    number_of_lines : u32 , 
    number_of_bytes : u32 
}

type MyResult<T> =  Result<T,Box<dyn Error>> ; 

pub fn  get_args()  -> MyResult<Config> {  
    let  matches = command!()
              .arg( arg!( [files] ...  ) 
                    .default_values(["-"])           
               ) 
              .arg( arg!( -n --number_of_lines <LINES> "set number of lines in output" )
                    .default_value("10") 
                    .value_parser(clap::value_parser!(u32))
               )
              .arg( arg!( -b --number_of_bytes <BYTES> "set number of bytes in output" ) 
                    .default_value("10")
                    .value_parser(clap::value_parser!(u32))
               )
              .get_matches() ;
    Ok(Config { files: matches.get_many::<String>("files").unwrap_or_default().map(|s|  s.clone()).collect::<Vec<_>>()  ,
                number_of_lines: *matches.get_one::<u32>("number_of_lines").unwrap()  ,
                number_of_bytes: *matches.get_one::<u32>("number_of_bytes").unwrap()  
              } 
      )           
}

fn open(filename: &str) -> MyResult<Box< dyn BufRead>> {  
    match filename {  
        "-" => Ok(Box::new(BufReader::new(io::stdin())))  ,
         _  => Ok(Box::new(BufReader::new( File::open(filename)?)))  , 
     }
}


fn read(buf_read: Box<dyn BufRead> ,  no_of_lines:u32 ) -> MyResult<()> { 
    let mut  count:u32 = 0 ;  
    for  line_result in buf_read.lines() { 
        if count >= no_of_lines  { break ; }    
        let line = line_result? ; 
        println!("{}" , line ) ; 
        count+=1 ; 
    }
    Ok(()) 
}

pub fn run(config:Config) -> MyResult<()> { 
   for  file in config.files { 
         match open(&file) {  
          Ok(bufread) => 
            if let Err(e) =   read( bufread ,  config.number_of_lines)  { 
                   eprintln!("{}" ,e)
            }    , 
          Err(e) => eprintln!("{}" , e)
        } ; 
   }
   Ok(()) 
}