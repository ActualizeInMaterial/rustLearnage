
use std::env;
use std::ffi::OsString;
//use std::convert::From; //don't need this, the error was confusing!

//Rust has a keyword, match, that allows you to replace complicated if/else groupings with something more powerful. Check it out:
fn main() {
    //src: http://doc.rust-lang.org/book/match.html
    let x = 3;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    //match is also an expression
    println!("{}", match x {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "something else",
    });


//    let Option:y=Some(1);
    let z=env::var_os("RUST_BACKTRACE");
    let qq=OsString::from("2");
//    let qq=z.as_os_str().to_str();
//    let qq="1".to_os_string();
    println!("val={}", match z {
        Some(q) => { //XXX: this does not use the above 'q' ! and no warning!
//            println!("{}", q.as_ref());
            if q == qq {//ok, == doesn't work as I expect it! '1' == '2' yields true
                println!("{}", q.to_string_lossy());
            }
            2 
        },
        None => 1,
    })

}

// vim note: = , the indent command can take motions. So, gg to get the start of the file, = to indent, G to the end of the file, gg=G.
// src: https://stackoverflow.com/questions/506075/how-do-i-fix-the-indentation-of-an-entire-file-in-vi

