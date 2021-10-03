struct Board {
    price_before: f64,
    price_after: f64,
    p_of: u16,
}
impl Board {
    fn add_percent(&mut self, percent: u8) {
        self.price_after = (self.price_before * (percent as f64 * 0.01)) + self.price_before;
    }
}
fn main() {
    // let mut obnizka = Board {
    //     price_before: 10.0,
    //     price_after: 0.,
    //     p_of: 11,
    // };
    let mut obnizka = obnizka_from_keyboard();
    match obnizka.p_of {
        0..=10 => obnizka.price_after = obnizka.price_before.clone(),
        11..=20 => obnizka.add_percent(5),
        21.. => println!("za duza cena"),
    }
    println!("Stara cena: {}", obnizka.price_before);
    println!("Nowa cena: {}", obnizka.price_after);
}

fn input(print: &str, typ:bool) -> f64 {
    println!("{}",print);
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {}
    }
    match input.trim().to_string().parse::<f64>() {
        Ok(e) => e,
        Err(_) => {
            println!("podałeś nieprawidłąwą numer \"{}\" zostanie użyta domyślna wartość", input.trim());
            if typ {
                13.
            } else {
                1000.
            }
        }
    }
}
fn obnizka_from_keyboard() -> Board {
    let a = input("Podaj cenę początkową płyty głównej: ",false);
    let b = input("Podaj procent o jaki cena wzrosła: ",true);
    Board {
        price_before: a,
        price_after: 0.,
        p_of: b as u16
    }
}