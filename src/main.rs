use std::collections::hash_map;
use::std::io;

struct Bill {
    name: String,
    amount: f64,
}

struct Bills {
    bills: hash_map::HashMap<String, Bill>,
}

impl Bills {
    fn add_bill(&mut self) {
        let name = ask_for_input("Enter the name of the bill");
        let is_key_present = self.bills.contains_key(&name);
        match is_key_present {
            true => println!("Bill already exists"),
            false => {
                let amount = ask_for_input("Enter the amount of the bill").parse::<f64>().unwrap();
                let bill = Bill { name: name.clone(), amount };
                self.bills.insert(name, bill);
                println!("Bill added successfully");
            }
        }
    }

    fn update_bill(&mut self) {

        let name = ask_for_input("Enter the name of the bill");

        let is_key_present = self.bills.contains_key(&name);
        if !is_key_present {
            return println!("Bill not found");
        }else {
            let amount = ask_for_input("Enter the amount of the bill").parse::<f64>().unwrap();
            let undo = ask_for_input("Do you want to confirm? (y/n)");
            match undo.as_str() {
                "y" => {
                    let bill = self.bills.get_mut(&name).unwrap();
                    bill.amount = amount;
                    println!("Bill updated successfully");
                },
                "n" => {
                    println!("Bill not updated");
                },
                _ => {
                    ask_for_input("Invalid input, please try again");
                }
            }
        }

    }

    fn remove_bill(&mut self) {
        let name = ask_for_input("Enter the name of the bill");
        match self.bills.remove(&name) {
            Some(bill) => println!("Bill removed successfully with name: {}, amount: {}", bill.name, bill.amount),
            None => println!("Bill not found"),
        }
    }
    fn view_bill(&self) {
        let name = ask_for_input("Enter the name of the bill");
        match self.bills.get(&name) {
            Some(bill) => println!("Name: {}, Amount: {}", bill.name, bill.amount),
            None => println!("Bill not found"),
        }
    }  
    fn view_bills(&self) {
        let mut total = 0.0;
        for (name, bill) in self.bills.iter() {
            total += bill.amount;
            println!("Name: {}, Amount: {}", name, bill.amount);
        }
        println!("Total: {}", total);
    }
}


fn get_input() -> io::Result<String> {
    let mut buffer: String = String::new();
    io::stdin().read_line(&mut  buffer)?;
    Ok(buffer.trim().to_owned())
}

fn ask_for_input(question: &str) -> String {
    println!("{}", question);
    match get_input() {
        Ok(input) => {
            if input.is_empty() {
                ask_for_input("Invalid input, please try again")
            } else {
                input
            }
        },
        Err(_) => ask_for_input("Invalid input, please try again")
    }
}

fn provide_instrutions() {
    println!("Please enter the command you want to execute");
    println!("1. Add bill");
    println!("2. Remove bill");
    println!("3. View bill");
    println!("4. View all bills");
    println!("5. Exit");
}


fn main() {

    provide_instrutions();
    let mut bills = Bills { bills: hash_map::HashMap::new() };

    loop {
        let input = get_input().unwrap();
        match input.as_str() {
            "1" => {
                bills.add_bill();
            },
            "2" => {
                bills.remove_bill();
            },
            "3" => {
                bills.view_bill();
            },
            "4" => {
                bills.view_bills();
            },
            "5" => {
                bills.update_bill();
            },
            "clear" => {
                provide_instrutions();
            }
            _ => {
                println!("Invalid command, please try again");
            }
        }
    }

}