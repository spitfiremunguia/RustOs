use std::thread;
use std::sync::{Mutex, Arc};
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
extern crate colored;
use colored::*;
 
struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}
 
impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }
 
    fn eat(&self, table: &Table) {
        let mut _left = table.forks[self.left].lock().unwrap();
        let mut _right = table.forks[self.right].lock().unwrap();
    
        println!("{} is eating", self.name.cyan());
        //do something with the files,like write on them or something
        let mut leftVectorContent=_left.content.lines().collect::<Vec<&str>>();
        let mut rightVectorContent=_right.content.lines().collect::<Vec<&str>>();
        //delete left fork register file
        println!("{} is reading:{}",self.name.green(),leftVectorContent[0].green());
        leftVectorContent.remove(0);
        let mut LeftStrings:Vec<String>= leftVectorContent.iter().map(|x| x.to_string()).collect();
        let mut leftFile=OpenOptions::new().write(true).open(&_left.path).unwrap();
        writeln!(leftFile, "{}", LeftStrings.join("\n")).unwrap();
        _left.content=LeftStrings.join("\n");
        //delete right for register file
        println!("{} is reading:{}",self.name.yellow(),rightVectorContent[0].yellow());
        rightVectorContent.remove(0);
        let mut RightStrings:Vec<String>= rightVectorContent.iter().map(|x| x.to_string()).collect();
        let mut rightFile=OpenOptions::new().write(true).open(&_right.path).unwrap();
        writeln!(rightFile, "{}", RightStrings.join("\n")).unwrap();
        _right.content=RightStrings.join("\n");
        thread::sleep_ms(3000);

        
        println!("{} is done eating", self.name.magenta());
        println!();
 
    }
}
 
struct MyFile
{
    path:String,
    content:String,
}
struct Table {
    forks: Vec<Mutex<MyFile>>,
}
fn main()->std::io::Result<()> {


    let mut cars = File::open("cars.txt")?;
    let mut fruits=File::open("fruits.txt")?;
    let mut animals=File::open("animals.txt")?;
    let mut languages=File::open("languages.txt")?;
    let mut orientations=File::open("orientations.txt")?;
    //create a buffer for eacch file in order to access the information
    let mut carsContent=String::new();
    let mut fruitsContent=String::new();
    let mut animalsContent=String::new();
    let mut languagesContent=String::new();
    let mut orientationsContent=String::new();
    cars.read_to_string(&mut carsContent);
    fruits.read_to_string(&mut fruitsContent);
    animals.read_to_string(&mut animalsContent);
    languages.read_to_string(&mut languagesContent);
    orientations.read_to_string(&mut orientationsContent);
    //create the objects
    
    

    let mut table = Arc::new(Table { forks: vec![
        Mutex::new(MyFile{  
            path:"cars.txt".to_string(),
            content:carsContent
        }),
        Mutex::new(MyFile{
            path:"fruits.txt".to_string(),
            content:fruitsContent
        }),
        Mutex::new(MyFile{
            path:"animals.txt".to_string(),
            content:animalsContent
        }),
        Mutex::new(MyFile{
            path:"languages.txt".to_string(),
            content:languagesContent
        }),
        Mutex::new(MyFile{
            path:"orientations.txt".to_string(),
            content:orientationsContent
        })
    ]});
 
    let mut philosophers = vec![
        Philosopher::new("David", 0, 1),
        Philosopher::new("Oso", 1, 1),
        Philosopher::new("Kathy", 2, 1),
        Philosopher::new("Chang", 3, 1),
        Philosopher::new("Jeff",0,4)
    ];
 
    let mut handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();
        
        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();
   
    
    for h in handles {
        h.join().unwrap();
    }
    Ok(())
   
}

