use std::io;

fn main() {
    enum Fastfood {
        Zinger_burger,
        Cheese_mayo_garlic_frise,
        Club_sandwitch,
    }

    fn Price(fastfood: Fastfood) -> u32 {
        match fastfood {
            Fastfood::Zinger_burger => 250,
            Fastfood::Cheese_mayo_garlic_frise => 150,
            Fastfood::Club_sandwitch => 200,
        }
    }

    struct Customer_details {
        Name: String,
        Phone_no: u32,
        Address: String,
    };

    fn details(customer: &Customer_details) {
        println!("Name of the customer: {}", customer.Name);
        println!("Phone Number of customer: {}", customer.Phone_no);
        println!("Address of customer: {}", customer.Address);
    }

    loop {
        println!("What would you like to have?");
        println!("Zinger Burger: Rs = 250");
        println!("Cheese Mayo Garlic Frise: Rs = 150");
        println!("Club Sandwitch: Rs = 200");

        let mut order = String::new();

        io::stdin()
            .read_line(&mut order)
            .expect("Please input something tasty");

        println!("Please tell us your good name");

        let mut name = String::new();

        io::stdin().read_line(&mut name).expect("Your name please");

        println!("You phone number please");

        let mut phone_no = String::new();

        io::stdin()
            .read_line(&mut phone_no)
            .expect("You have entered an invalid number");
        let phone_no = phone_no.trim().parse::<u32>().unwrap();

        println!("Your Address please");

        let mut address = String::new();

        io::stdin()
            .read_line(&mut address)
            .expect("Please provide your correct address");

        let customer1 = Customer_details {
            Name: name,
            Phone_no: phone_no,
            Address: address,
        };

        details(&customer1);

        if order.trim() == "Zinger Burger" {
            let purchase = Fastfood::Zinger_burger;
            let total_price = Price(purchase);
            println!(
                "Congratulations! Your order has been placed sucessfully and your amount is: {}",
                total_price
            );

            break;
        } else if order.trim() == "Cheese Mayo Garlic Frise" {
            let purchase = Fastfood::Cheese_mayo_garlic_frise;
            let total_price = Price(purchase);
            println!(
                "Congratulations! Your order has been placed sucessfully and your amount is: {}",
                total_price
            );

            break;
        } else if order.trim() == "Club Sandwitch" {
            let purchase = Fastfood::Club_sandwitch;
            let total_price = Price(purchase);
            println!(
                "Congratulations! Your order has been placed sucessfully and your amount is: {}",
                total_price
            );

            break;
        } else {
            println!("You have entered incorrect order");
        }
    }
}
