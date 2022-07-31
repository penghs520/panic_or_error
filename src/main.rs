use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    /*let file = match File::open("hello.txt") {
        Ok(file) => { file }
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => {
                    match File::create("hello.txt") {
                        Ok(file) => { file }
                        Err(create_err) => panic!("Problem create the file: {:?}", create_err)
                    }
                }
                other_error => panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };
*/
    //let file1 = File::open("hello1.txt").unwrap();
    //let error = File::open("hello1.txt").unwrap_err();
    //println!("err {:?}", error);

    //let file1 = unsafe { File::open("hello1.txt").unwrap_unchecked() };
    //println!("file1 {:?}", file1);

    //let file1 = File::open("hello1.txt").unwrap_or_default();
    //println!("file1 {:?}", file1);

    let file = File::open("hello1.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello1.txt").unwrap_or_else(|err| {
                panic!("Problem creating the file: {:?}", err);
            })
        } else {
            panic!("Problem opening the file: {:?}", err);
        }
    });

    let mut file = File::open("hello2.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|err| {
                panic!("create file err: {:?}", err);
            })
        } else {
            panic!("Problem opening the file: {:?}", err);
        }
    });
    let mut username = String::new();
    file.read_to_string(&mut username)?
}
