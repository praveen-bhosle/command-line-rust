use assert_cmd::Command ; 

#[test]
fn main() {   
let mut cmd = Command::cargo_bin("false").unwrap() ; 
cmd.assert().failure() ; 
} 
