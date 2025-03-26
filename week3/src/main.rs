
#[derive(Debug)]
struct CryptoTap {
    keg: Keg,
    balance: f32,
    investor: Option<u32>
}

#[derive(Debug)]
struct Keg {
    style: String,
    volume: f32,
    level: f32,
    price_per_liter: f32,
}

#[derive(Debug)]
struct Pint {
    volume: f32,
    price: f32,
}

impl Pint {

    fn new(volume: f32, price: f32) -> Self {
        Self {
            volume: volume,
            price: price
        }
    }

    fn setPrice(&mut self, price: f32) {
        self.price = price;
    }
}

fn main() {
    println!("Hi, my friend! Welcome in my new CryptoTap!");
    println!("We offer just one sort of beer and we are sold out last night, but we are working on it.");
    println!("Do you want to invest in our CryptoTap?");
    println!("We have a special offer for you: you can invest in our CryptoTap and get a part of the profit.");
    println!("We are working on a smart contract for this, but for now we can do it manually.");
    println!("You just need to invest the money for one keg of beer and you will get a part of the profit.");
    println!("The price of the keg is 5.0 EUR per liter.");
    println!("The keg has 50 liters of beer, you get 50% of the profit.");
    println!("The profit is the difference between the price of the sold beer and the price of the keg.");

    let keg = Keg {
        style: "IPA".to_string(),
        volume: 50.0,
        level: 0.0,
        price_per_liter: 5.0,
    };

    let mut pint = Pint::new(0.568, 5.0);
    let half_pint = Pint::new(0.284, 3.0);
    let third_pint = Pint::new(0.189, 1.75);

    pint.setPrice(5.45);

    let cryptotap = CryptoTap {
        keg: keg,
        balance: 0.0,
        investor: None
    };

    println!("The price of the keg is: {}", cryptotap.keg.volume * cryptotap.keg.price_per_liter);
    println!("The price of a Pint is: {}", pint.price);
    println!("The price of a Half Pint is: {}", half_pint.price);        
    println!("The price of a Third Pint is: {}", third_pint.price);

    println!("The profit of a Pint is: {}", pint.price - cryptotap.keg.price_per_liter * pint.volume);
    println!("The profit of a Half Pint is: {}", half_pint.price - cryptotap.keg.price_per_liter * half_pint.volume);
    println!("The profit of a Third Pint is: {}", third_pint.price - cryptotap.keg.price_per_liter * third_pint.volume);

    let keg_volume = cryptotap.keg.volume;

    let pint_profit = ((pint.price*(1.0/pint.volume)) - cryptotap.keg.price_per_liter) * (keg_volume / pint.volume);
    let half_pint_profit = ((half_pint.price*(1.0 / half_pint.volume)) - cryptotap.keg.price_per_liter) * (keg_volume / half_pint.volume);
    let third_pint_profit = ((third_pint.price*(1.0/ third_pint.volume)) - cryptotap.keg.price_per_liter) * (keg_volume / third_pint.volume);

    println!("The profit of a Pint is: {}", pint_profit);
    println!("The profit of a Half Pint is: {}", half_pint_profit);
    println!("The profit of a Third Pint is: {}", third_pint_profit);

    println!("Are you interested in investing in our CryptoTap?");
    println!("How much money do you want to invest?");

}
