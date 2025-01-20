use std::io;

#[derive(Debug)]

struct Book {
    title: String,
    author: String,
    year: u32,
    is_available: bool,
}

fn input_fn(prompt: &str) -> String {
    let mut input = String::new();
    
    loop{
        println!("{}", prompt);
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let result = input.trim().to_string();
                if !result.is_empty() {
                    return result;
                } else {
                    println!("Ввод не может быть пустым. Попробуйте снова.");
                }
            }
            Err(_) => {
                println!("Ошибка ввода, попробуйте снова.");
                continue;
            }
        }
    }
}

fn input_year(prompt: &str) -> u32 {
    loop {
        let input = input_fn(prompt);
        match input.parse::<u32>() {
            Ok(year) => return year,
            Err(_) => println!("Пожалуйста, введите корректный год (целое число)"),
        }
    }
}

impl Book {
    fn new() -> Book {
        println!("Put inf about new book");
        let title = input_fn("Title:");
        let author = input_fn("Author:");
        let year = input_year("Year:");
        let is_available = true;
        Book {
            title,
            author,
            year,
            is_available,
        }
    }
}

impl Book {
    fn display_info(&self) {
        println!("<|");
        println!("Title: {}({})", self.title, self.year);
        println!("Author: {}", self.author);
        println!("Status: {}", if self.is_available==true {"available"} else {"unavailable"});
        println!("|>");
    }
}

impl Book{
    fn toggle_availability(&mut self) -> bool{
        self.is_available = !self.is_available;
        self.is_available
    }
}

fn main() {

    let mut book1 = Book {
        title: String::from("Title1"),
        author: String::from("author1"),
        year: 2020,
        is_available: true,
    };
    let mut book2 = Book {
        title: String::from("Title2"),
        author: String::from("author1"),
        year: 2021,
        is_available: true,
    };
    let mut book3 = Book {
        title: String::from("Title3"),
        author: String::from("author1"),
        year: 2022,
        is_available: false,
    };
    let mut book4 = Book {
        title: String::from("Title4"),
        author: String::from("author2"),
        year: 2020,
        is_available: false,
    };
    let mut book5 = Book {
        title: String::from("Title5"),
        author: String::from("author2"),
        year: 2020,
        is_available: true,
    };

    
    book1.display_info();
    book1.toggle_availability();
    book1.display_info();
    let book6 = Book::new();
    book6.display_info();
}
