use std::thread;
use std::sync::{Mutex, Arc};
 
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
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
    
        println!("{} is eating.", self.name);
 
        thread::sleep_ms(5000);
 
        println!("{} is done eating.", self.name);
 
    }
}
 
 
struct Table {
    forks: Vec<Mutex<()>>,
}
fn main() {
let mut flag=true;
while flag{

    let mut table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(())
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
    flag=false;
    for h in handles {
        h.join().unwrap();
    }
    handles=Vec::new();
    philosophers= vec![
        Philosopher::new("David", 0, 1),
        Philosopher::new("Oso", 1, 2),
        Philosopher::new("Kathy", 2, 3),
        Philosopher::new("Chang", 0, 3)
    ];
    table= Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(())
    ]});
}

}