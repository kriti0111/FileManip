use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::time::Instant;


fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments please specify the file name");
    }
    let filename = &args[1]; 

    let contents = read_file(filename.to_string());
    
    let data = parse_file(contents);
    write_file(data);
   let duration = start.elapsed();

   println!("Time elapsed is: {:?}", duration);
}

fn read_file(filename : String) -> String{
    let contents = match fs::read_to_string(filename){
        Ok(fc) => fc,
        Err(e) => panic!("Problem reading the file: {:?}", e),
    };
    return contents
}

fn parse_file (contents: String) ->String{
    let mut fizzBuzzVec: Vec<String> = Vec::new();
    
    for line in contents.lines(){
       let number:i32 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => {fizzBuzzVec.push(line.to_string());  
            continue;},
    };

    if number%15 == 0{
        fizzBuzzVec.push("FizzBuzz".to_string());
       }
       else if number%5 == 0{
        fizzBuzzVec.push("Buzz".to_string());
       }
       else if number%3==0 {
        fizzBuzzVec.push("Fizz".to_string());
       }
       else {
           fizzBuzzVec.push(line.to_string());
       }
       
    }
    return fizzBuzzVec.join("\n")
}

fn write_file(data: String){
    let mut file = match File::create("foo.txt"){
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
    };
                                                                                                                                                                    
    assert!(file.write_all((data).as_bytes()).is_ok());       

}
