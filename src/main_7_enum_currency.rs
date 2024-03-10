enum CurrencyType {
    WHITE,
    BLACK
}
enum Currency {
    USD,
    PKS,
    INR(CurrencyType),
}

fn main() {
    let obamas_currency = Currency::USD;
    let shahid_currency = Currency::PKS;
    let my_currency = Currency::INR(CurrencyType::BLACK);
    let his_currency = Currency::INR(CurrencyType::WHITE);

    println!("My currency details are");
    check_currency(my_currency);

    println!("His currency details are");
    check_currency(his_currency);

    println!("Obamas currency details are");
    check_currency(obamas_currency);

    println!("Shahid currency details are");
    check_currency(shahid_currency);

}

fn check_currency(currency: Currency) {
    match currency {
        Currency::USD => {
            println!("Currercy is American");
        }
        Currency::PKS => {
            println!("Currercy is pakisthans");
        }
        Currency::INR(CurrencyType::BLACK) => {
            println!("Currency is black Indian ");
        }
        Currency::INR(CurrencyType::WHITE) => {
            println!("Currency is white Indian ");
        }
    }
}