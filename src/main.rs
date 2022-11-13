

use std::{env::args, process::exit};



fn caesar(text:&String, key:usize, encode:bool){
    let text = text.bytes().collect::<Vec<u8>>();
    let mut result = String::new();

    for i in 0..text.len(){
        let char = text[i];

        if !char.is_ascii_alphabetic(){result.push(std::str::from_utf8(&[char]).unwrap().chars().nth(0).unwrap());continue;}

        let res:String = if char.is_ascii_uppercase(){
            if encode{
                std::str::from_utf8(&[(&char + (key as u8)-65) % 26 + 65]).unwrap().to_string()
            }else{
                std::str::from_utf8(&[(&char - (key as u8)-65) % 26 + 65]).unwrap().to_string()
            }
        }else{
            if encode{
                std::str::from_utf8(&[(&char + (key as u8)-97) % 26 + 97]).unwrap().to_string()
            }else{
                std::str::from_utf8(&[(&char - (key as u8)-97) % 26 + 97]).unwrap().to_string()
            }
        };

        result.push(res.chars().nth(0).unwrap());
    }

    println!(">> {:#?}",result);

}


fn main() {




    let mut opts = args().collect::<Vec<String>>();
    opts.drain(0..1);
    if opts.len() <=1 || opts.len() <4{println!("Error. ");exit(0);}


    let mut phrase:&String = &String::new();
    let mut key:usize = 0;
    let mut encode:bool = true;

    if opts.contains(&"-k".to_string()){ 
        key = opts[1].parse::<usize>().unwrap();

        if opts.contains(&"-e".to_string()){phrase = &opts[3];encode = true;}
        if opts.contains(&"-d".to_string()){phrase = &opts[3];encode = false;}

    }else{println!("Error. \n Ex: cesar.exe -k 4 -e 'my text'");exit(0);}

    caesar(phrase,key,encode);




}









