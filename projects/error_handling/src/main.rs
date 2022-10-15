use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

#[allow(dead_code)]
fn main2() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    println!("File is {:?}", f);

    Ok(())
}

fn main() {
    // statements that cause a panic
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];

    // the file hello.txt must be missing for the errors to be shown
    // let f = File::open("hello.txt");

    // match on one error
    // one_error(&f);

    // match two errors
    // two_errors(&f);

    // match three errors
    // three_errors(f);

    // shortcuts for Panic on Error: unwrap and expect
    // let f = File::open("hello.txt").unwrap();

    // let f = File::open("hello.txt").expect("Failed to open file hello.txt");
    // println!("File {:?}", f);

    // create a file hello.txt for these functions to succeed
    let s = read_username_from_file();
    println!("String is {:?}", s);

    let s = read_username_from_file_shortcut1();
    println!("String is {:?}", s);

    let s = read_username_from_file_shortcut2();
    println!("String is {:?}", s);

    let s = read_username_from_file_shortcut3();
    println!("String is {:?}", s);
}

#[allow(dead_code)]
fn one_error(file: &std::io::Result<File>) {
    // if cannot find file panics
    let _f = match &file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

#[allow(dead_code)]
fn two_errors(file: &std::io::Result<File>) {
    // if cannot find file displays not found or other error
    let _f = match &file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("File not found"),
            other_error => panic!("other error {:?}", other_error),
        },
    };
}

#[allow(dead_code)]
fn three_errors(file: std::io::Result<File>) {
    // if cannot find file, tries creating a file,
    // if error panics if any other error panics
    let _f = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file2) => file2,
                Err(error2) => panic!("Error creating file {:?}", error2),
            },
            other_error => panic!("other error {:?}", other_error),
        },
    };
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut1() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shortcut2() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shortcut3() -> Result<String, std::io::Error> {
    fs::read_to_string("hello.txt")
}
