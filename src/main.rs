use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};



fn read<T>()->Vec<T> 
where 
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err : std::fmt::Debug,
{
    let mut inpt = String::new();
    std::io::stdin().read_line(&mut inpt).unwrap();
    let v : Vec<T> = inpt.trim().split(" ").map(|x| x.parse().expect("NO")).collect();
    return v;
}

fn parsing(line: String) -> Vec<String>{
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in line.match_indices(|c: char| !(c.is_alphanumeric() || c == '\'')) {
        if last != index {
            result.push((&line[last..index]).to_string());
        }
        result.push(matched.to_string());
        last = index + matched.len();
    }
    if last < line.len() {
        result.push((&line[last..]).to_string());
    }
    return result;
}


fn main() {
    let mut is_variable: HashMap<String,bool> = HashMap::new();
    // let filename = read::<String>()[0].clone();
    let filename = "src/file.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    
    let mut generator = |line: Vec<String>| {
        let mut currentLine = String::from("");
        let mut lastWord = String::from("");
        let mut quotes: i8 = 0;
        for i in 0..line.len(){
            let currentWord = &line[i];
            if currentWord == "\""{
                quotes += 1;
                quotes %= 2;
            }
            if currentWord == " " || quotes == 1{
                continue;
            }
            if currentWord == "=" {
                if quotes == 0 {
                    is_variable.insert(lastWord , true);
                    lastWord = "".to_string();
                }
            }else{
                lastWord = (&currentWord).to_string();
            }
        }
        for i in 0..line.len(){
            let mut currentWord = line[i].clone();
            if currentWord == " "{
                currentLine.push(' ');
                continue;
            }
            if is_variable.contains_key(&currentWord){
                currentWord.insert(0,'$');
                currentLine.push_str(&currentWord);
            }else{
                currentLine.push_str(&currentWord);
            }
        }
        currentLine = currentLine.replace("{{" , "<?php ");
        currentLine = currentLine.replace("}}" , "?>");
        println!("{}" , currentLine);
        
    };
    

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); 
        let newLine = parsing(line);
        generator(newLine);

    }
    let exit:i32 = read()[0];
}