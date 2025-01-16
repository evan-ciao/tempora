use std::io::{self, stdout, Write};
use chrono::{self, TimeZone};

fn main() {
    let alphabet = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    
    println!("arvelie time passed since gregorian zero date");
    
    loop {
        println!("[greg->arvelie] enter gregorian zero date");
        
        let mut month = String::new();
        let mut day = String::new();
        let mut year = String::new();

        print!("[month] ");
        io::stdout().flush();
        io::stdin()
            .read_line(&mut month)
            .expect("failed to read input");
        
        print!("[day] ");
        io::stdout().flush();
        io::stdin()
            .read_line(&mut day)
            .expect("failed to read input");

        print!("[year] ");
        io::stdout().flush();
        io::stdin()
            .read_line(&mut year)
            .expect("failed to read input");

        let month: u32 = month.trim().parse().expect("failed conversion to int");
        let day: u32 = day.trim().parse().expect("failed conversion to int");
        let year: i32 = year.trim().parse().expect("failed conversion to int");

        let date_now = chrono::offset::Local::now().date();
        let date = chrono::offset::Local.ymd(year, month, day);

        let days_passed = (date_now - date).num_days();
        
        let arvelie_day = days_passed % 14;
        let arvelie_year = days_passed / 364;
        let arvelie_month = (days_passed / 14) - (arvelie_year * 26);

        println!("[arvelie date output] {:0>2}{}{:0>2}", arvelie_year, alphabet[arvelie_month as usize], arvelie_day);

        break;
    }
}
