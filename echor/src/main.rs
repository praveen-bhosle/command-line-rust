use clap::{arg,command,ArgAction}  ; 


fn main() { 
   let matches = command!()
       .arg(arg!([text] ... "strings"  ))
	   .arg( arg!( -n --new_line "\n wont be append at the end" )   
	         .action(ArgAction::SetTrue) 
	   ) 
	   .get_matches()  ;
 
 let   flag_present =  matches.get_flag("new_line")  ;

 let input = matches
                                             .get_many::<String>("text")
                                             .unwrap_or_default()
                                             .map( |s| s.as_str())
                                             .collect::<Vec<_>>()
                                             .join(" ")  ;
  
  print!("{}{}" , input , if flag_present {""} else {"\n"}) 
}