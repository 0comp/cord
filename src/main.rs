
use std::{io, vec};


fn main() {
    
    let mut matrix = getmatrix();
    loop {
        render(&matrix);
        matrix = vad_göras(&matrix);
    }
    
}



fn getv() -> i32 {
    let mut v: String = String::new();
    io::stdin()
        .read_line(&mut v )
        .expect("nuh uh");
    v.replace("\r", "").replace("\n", "").parse().expect("Failed to parse string to integer")
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum Sak {
    Inget,
    Prick,
    Linjesida,
    Linjeupner
}


fn getmatrix() -> Vec<Vec<Sak>> {
    println!("hur lång ska den vara?");
    let y = getv();
    println!("hur bred ska den vara");
    let x = getv();
    vec![vec![Sak::Inget; x as usize]; y as usize]
}

fn dot(mut matrix: Vec<Vec<Sak>>) -> Vec<Vec<Sak>>{
    
    
    println!("vilken x ska din prick vara på ");
    let x  = getv();
    println!("vilken y ska din prick vara på ");
    let y  =  getv();

    matrix[y as usize][x as usize] = Sak::Prick;
    matrix
}   
fn linjesida(mut matrix: Vec<Vec<Sak>>) -> Vec<Vec<Sak>>{
    println!("vid vilken x ska din linje börja på");
    let start = getv();
    println!("vid vilken x ska din linje sluta på");
    let stop = getv();
    println!("Och på villken y ska linjen vara");
    let y = getv();
    let mut counter = 0;
    loop {
        if counter == start {
            loop {
                matrix[y as usize][counter as usize] = Sak::Linjesida;
                if counter == stop {return matrix}
                counter += 1;
            }
        }
    counter += 1;
    }
}
fn linjeupner(mut matrix: Vec<Vec<Sak>>)-> Vec<Vec<Sak>>{
    println!("vid vilken y ska din linje börja på");
    let start = getv();
    println!("vid vilken y ska din linje sluta på");
    let stop = getv();
    println!("Och på villken x ska linjen vara");
    let x = getv();
    let mut counter = 0;
    loop {
        if counter == start {
            loop {
                matrix[counter as usize][x as usize] = Sak::Linjeupner;
                if counter == stop {return matrix}
                counter += 1;
            }
        }
    counter += 1;
    }
}



fn render(matrix: &Vec<Vec<Sak>>) {
    dbg!(&matrix);
    matrix.iter().for_each(|v| {
        v.iter().for_each(|v| {
            if *v == Sak::Inget {print!("=")}
            if *v == Sak::Prick {print!("*")}
            if *v == Sak::Linjesida {print!("-")}
            if *v == Sak::Linjeupner {print!("|")}
        });
    println!()
    });
}


fn vad_göras(mut matrix: &Vec<Vec<Sak>>) -> Vec<Vec<Sak>>{
    loop {
        let mut v: String = String::new();
        println!("vad vill du göra\n");
        println!("Skriv \"help\" för en lista på alla komandon\n");
    
        io::stdin()
            .read_line(&mut v )
            .expect("nuh uh");
        v = v.replace("\r", "").replace("\n", "");
        dbg!(&v);
        if v == "help".to_string() {
            println!("linje\nprick\nny matrix\n");
        }
        if v == "linje".to_string() {
            let mut x = String::new();
            println!("ska den gå från sida till sida eller up och ner?\n");
            io::stdin()
                .read_line(&mut x )
                .expect("nuh uh");
            x = x.replace("\r", "").replace("\n", "");
            dbg!(&x);
            if x == "sida till sida".to_string() || x == "sida".to_string() {
                let mut randvec = vec![vec![Sak::Inget]];
                randvec = linjesida(matrix.clone());
                return randvec
            }
            if x == "up och ner".to_string() ||x == "up".to_string() {
                let mut randvec = vec![vec![Sak::Inget]];
                randvec = linjeupner(matrix.clone());
                return randvec
            }}
        if v == "prick".to_string() {
            let mut randvec = vec![vec![Sak::Inget]];
            randvec = dot(matrix.clone());
            return randvec 
         }

        if v == "ny".to_string() || v == "ny matrix".to_string() || v == "matrix".to_string(){
            let y  = getmatrix();
            return y
        }
    }
}



















// fn push_rows()  {
//     let space_between_rows = getx();
//     let list: Vec<String> = getbase();
//     let mut counter = 0;
//     let list =list.iter().map(|edit| {
//         let mut edit = edit.to_string();
//         counter += 1;
//         if counter == space_between_rows {
//             edit = "1".to_string();
            
//         }
//         edit
//     }).collect();
//     dbg!(list);
// }
// - 0
// /n 1
// * 2
// 
// 
// 
// 
// 
// 

// fn render() {
//     for 
// }

// sh("0");
//                 counter2 += 1;
//                 break[x];
//             if counter1 == xamount && counter2 == yamount {
//                     break[x]
//             };
//             }
//         };
//     }
// }
        
// gör render sen de e aids gör antingen 0 = - 1= \n 2= *
// eller 
// let counter = 0
// map(|data|{
// counter += 1 
// if counter == x {
//  new_x = 1   (returnar här)
// }
// });
    
    
    // for that in result {
    //         print!("{}", that)
    // }
    // dbg!(result);
    
    
    // }
    
    // let result = loop {
    //     counter1 += 1;
    //     x.push("-");
    // if counter1 == xamount {
    //     break x;
    //     }
    // };
    // for that in result {
    //     print!("{}", that)
    // }


    // fn maketxt() {
//     fs::File::create("./game.txt").unwrap();
// }




















// fn getsize() -> i32 {
//     let x = getx();
//     let y = gety();
//     x*y+y
// }

// fn getbase() -> Vec<String> {
//     let size = getsize();
//     let mut  counter = 0;
//     let mut list: Vec<String> = vec![];
//     loop {
//         counter += 1;
//         list.push("0".to_string());
//         if counter == size {
//             dbg!(&list);
//             return list
//         }
//     }
// }

