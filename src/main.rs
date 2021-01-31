use std::env;
use std::fs;


fn main() {
    let arg = env::args()
        .skip(1)
        .collect::<Vec<String>>();
    if arg.iter().count() == 2 {
        let _result = fs::rename(arg[0].clone(), arg[1].clone());
        match _result {
            Ok(_) => {println!("File renamed")}
            Err(_) => {println!("Error in renaming file")}
        }
    } else {
        println!("Incorrect number of arguments passed")
    }
}
