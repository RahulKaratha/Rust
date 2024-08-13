use std::io;
#[derive(Debug)]
struct Task {
    Sno: u8,
    Title: String,
    Done: bool,
}

impl Task {
    fn check(&mut self) -> () {
        self.Done = true;
    }
}

fn create(Sno: u8, Title: String, Done: bool) -> Task {
    Task { Sno, Title, Done }
}

fn input() -> String {
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Error");
    str
}

fn int_input() -> u8 {
    let mut int = String::new();
    io::stdin().read_line(&mut int).expect("Error");
    let i = int.trim().parse().expect("Enter an Integer");
    i
}
fn main() {
    let mut list: Vec<Task> = Vec::new();
    for n in 1..=3 {
        println!("\n\nTask {}", n);
        println!("Enter Task:");
        let mut title = input();
        let Task1 = create(n, String::from(title), false);
        list.push(Task1);
    }
    println!("\nCurrent Tasks in To-Do List\n");
    for tasks in &list {
        println!("{:?}", tasks);
    }
    while (true) {
        println!("\n\nEnter the S.No. of Task completed");
        let i: u8 = int_input();
        for mut tasks in &mut list {
            if (tasks.Sno) == i {
                tasks.check();
            }
        }

        println!("\nUpdated Tasks in To-Do List\n");
        for tasks in &list {
            println!("{:?}", tasks);
        }

        let mut check = false;
        for tasks in &list {
            if (tasks.Done == false) {
                check = true;
                break;
            }
        }
        if (check == false) {
            break;
        }
    }
}
