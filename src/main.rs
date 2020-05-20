use std::env;
use std::fs::{self, File};
use std::io::Write;


fn main() {
 
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; 

    let contents = read_File(filename.to_string());
    
    let y = parse_File(contents);
    write_To_File(y);
}

fn read_File(filename : String) -> String{
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents
}

fn parse_File (contents: String) -> Vec<String>{
    let mut fizzBuzzVec: Vec<String> = Vec::new();

    for line in contents.lines(){
       let x:i32 = line.trim().parse().expect("File does not contain numbers");
       
       if x%15 == 0{
        fizzBuzzVec.push("FizzBuzz".to_string());
       }
       else if x%5 == 0{
        fizzBuzzVec.push("Buzz".to_string());
       }
       else if x%3==0 {
        fizzBuzzVec.push("Fizz".to_string());
       }
       else {
           fizzBuzzVec.push(line.to_string());
       }
    }
    fizzBuzzVec
}

fn write_To_File(v:Vec<String>){
    let mut file = File::create("foo.txt").expect("Unable to create file");
    for i in v{                                                                                                                                                                  
        file.write_all((*i).as_bytes()).expect("Unable to write data");       
        file.write_all("\n".as_bytes()).expect("Unable to write data");                                                                                                                       
    }
}