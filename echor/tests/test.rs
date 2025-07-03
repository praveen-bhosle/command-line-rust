use assert_cmd::Command ; 

#[test]
fn plain_test() { 
let mut  cmd = Command::cargo_bin("echor").unwrap() ; 
cmd.assert().success() ; 
}

#[test]
fn test_with_input() { 
    let mut cmd = Command::cargo_bin("echor").unwrap() ; 
    cmd.arg("praveen bhosle").assert().success().stdout("praveen bhosle\n") ;
}

#[test] 
fn test_with_input_and_n_flag() { 
    let mut cmd = Command::cargo_bin("echor").unwrap() ; 
    cmd.args(&["praveen " , "-n"]).assert().success().stdout("praveen ") ;
}

#[test]
fn hello1() { 
    let expected = std::fs::read_to_string("tests/expected/hello1.txt").unwrap() ;
    let mut cmd = Command::cargo_bin("echor").unwrap(); 
    cmd.arg("Hello there").assert().success().stdout(expected) ; 
}

#[test]
fn hello1n() { 
    let expected = std::fs::read_to_string("tests/expected/hello1n.txt").unwrap() ; 
    let mut cmd = Command::cargo_bin("echor").unwrap() ; 
    cmd.args(&["-n" , "Hello there"]).assert().success().stdout(expected) ;  
}

#[test]
fn hello2() { 
    let expected = std::fs::read_to_string("tests/expected/hello2.txt").unwrap() ; 
    let mut cmd = Command::cargo_bin("echor").unwrap() ; 
    cmd.args(&["Hello" , "there"]).assert().success().stdout(expected) ;  
}

#[test]
fn hello2n() {
    let expected = std::fs::read_to_string("tests/expected/hello2n.txt").unwrap() ; 
    let mut cmd = Command::cargo_bin("echor").unwrap() ; 
    cmd.args(&["-n" , "Hello" , "there"]).assert().success().stdout(expected) ;  
}