use std::thread;
use std::sync::{Mutex, Arc};
use std::fs::File;
use std::fs;
use std::io::prelude::*;


 
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
    
    fn eat(&self, table: &Table) -> std::io::Result<()>{
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
    
        println!("{} is eating.", self.name);
        //thread::sleep_ms(3000);

        println!("{}",&_left);
        println!("{} is done eating.", self.name);
        let mut file = File::create("foo.txt")?;
        file.write_all(b"Hello, world!")?;
        Ok(())
    }
}
 
 
struct Table {
    forks: Vec<Mutex<String>>,
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
    
    

    let mut table = Arc::new(Table { forks: vec![
        Mutex::new((carsContent)),
        Mutex::new((fruitsContent)),
        Mutex::new((animalsContent)),
        Mutex::new((languagesContent)),
        Mutex::new((orientationsContent))
    ]});
 
    let mut philosophers = vec![
        Philosopher::new("David", 0, 1),
        Philosopher::new("Oso", 1, 2),
        Philosopher::new("Kathy", 2, 3),
        Philosopher::new("Chang", 3, 4),
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

