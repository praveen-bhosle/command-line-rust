
use std::{error::Error} ; 
use clap::{ arg , command , ArgAction } ;
use std::io::{self,BufRead,BufReader} ;
use std::fs::File ; 

type MyResult<T> = Result<T,Box<dyn Error>> ;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config{ 
    files: Vec<String> , 
    number_lines : bool , 
    number_nonblank_lines : bool ,
}

fn open(filename:&str) -> MyResult<Box< dyn BufRead>> { 
    match filename { 
        "-" => Ok(Box::new(BufReader::new(io::stdin() ) )  ) , 
         _  => Ok(Box::new(BufReader::new(File::open(filename)?)))    
    } 
} 

fn read_buffer( buffer: Box<dyn BufRead> ) -> MyResult<()>  { 
    for line_result in buffer.lines() { 
        let line = line_result? ; 
        println!("{}" ,  line ) ; 
    }
    Ok(()) 
} 

fn read_buffer2( buffer : Box<dyn BufRead>) -> MyResult<()> {  
    let mut  i = 1 ;
    for line_result in  buffer.lines() { 
        let line = line_result? ;  
        println!("     {} {}", i ,  line  ) ; 
        i = i + 1 ;   
    }
    Ok(()) 
}


fn read_buffer3(buffer : Box<dyn BufRead> ) -> MyResult<()> {   
    let mut i  = 1 ; 
    for line_result  in buffer.lines() { 
        let line = line_result? ; 
        if line.trim() != "" { 
            println!("     {} {}"  , i ,  line) ; 
            i = i + 1 ; 
        }
    }
    Ok(())
}



pub fn run(config:Config) -> MyResult<()> { 
    if config.number_nonblank_lines { 
        for filename in config.files { 
           
            match  open(&filename) { 
               Ok(stuff) => if let Err(e) = read_buffer3(stuff)  { eprintln!("{}" , e )  }   ,
               Err(e) => eprint!("{}: {}" , filename , e   )  
             }
           
        }
        Ok(()) 
    }
    else if  config.number_lines {
        for filename in config.files { 
            
            match  open(&filename) { 
               Ok(stuff) => if let Err(e) = read_buffer2(stuff)  { eprintln!("{}" , e )  }   ,
               Err(e) => eprint!("{}: {}", filename , e)  
             }
          
        }
        Ok(())  
    }
    else { 
        for filename in config.files { 
            
            match  open(&filename) { 
               Ok(stuff) => if let Err(e) = read_buffer(stuff)  { eprintln!("{}" , e )  }   ,
               Err(e) => eprint!("{}: {}", filename , e)  
             }
          
        }
        Ok(()) 
    }
}



pub fn get_args() -> MyResult<Config> { 
    let matches = command!()
                .arg( arg!([files] ... "files") 
                      .default_value("-") 
                    )
                .arg( arg!( -n --number_lines ) 
                      .action(ArgAction::SetTrue)
                    )
                .arg( arg!( -b --number_nonblank_lines ) 
                      .action(ArgAction::SetTrue)
                    )
                .get_matches(); 
    Ok( Config { files: matches.get_many::<String>("files").unwrap_or_default().map(|s|  s.clone()  ).collect::<Vec<_>>() ,  
                number_lines : matches.get_flag("number_lines") ,
                number_nonblank_lines :  matches.get_flag("number_nonblank_lines") ,
    }) 
}



