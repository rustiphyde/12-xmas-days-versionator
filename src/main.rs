use std::io;

fn main() {
    println!("The 12 Daze of X-mas - Rustiphyde");

    println!(
        "Which version would you like the lyrics to? Enter 1 for Traditional and 2 for Redneck."
    );
    let mut answ = String::new();

    io::stdin()
        .read_line(&mut answ)
        .expect("Failed to read the line.");

    let answ: &str = &answ.trim();

    if answ == "1" {
        println!("Ok, here's the Traditional version\n");
        for day in 1..13 {
            daze(day);
            for gift in (1..(day + 1)).rev() {
                trad(gift, if gift == 1 && day != 1 { "&" } else { "" });
            }
        }
    } else if answ == "2" {
        println!("Ok, here's the Redneck version\n");
        for day in 1..13 {
            daze(day);
            for gift in (1..(day + 1)).rev() {
                red(gift, if gift == 1 && day != 1 { "&" } else { "" });
            }
        }
    } else {
        println!("That is not a valid response.");
    }
}

fn daze(num: u32) {
    let day = match num {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };

    println!("\nOn the {} day of X-mas, my true love gave to me:\n", day);
}

fn trad(num: u32, prefix: &str) {
    let gifts = match num {
        1 => "A Partridge in a Pear Tree",
        2 => "Two Turtle Doves",
        3 => "Three French Hens",
        4 => "Four Calling Birds",
        5 => "Five Golden Rings",
        6 => "Six Geese a-Layin'",
        7 => "Seven Swans a-Swimmin'",
        8 => "Eight Maids a-Milkin'",
        9 => "Nine Ladies Dancin'",
        10 => "Lords a-Leapin'",
        11 => "Eleven Pipers Pipin'",
        12 => "Twelve Drummers Drummin'",
        _ => "",
    };

    println!("{} {}", prefix, gifts);
}

fn red(num: u32, prefix: &str) {
    let gifts = match num {
        1 => "Some parts to a Mustang GT",
        2 => "Two huntin' dogs",
        3 => "Three shot gun shells",
        4 => "Four big mud tires",
        5 => "Five flanne shirts",
        6 => "Six cans of spam",
        7 => "Seven packs of redman",
        8 => "Eight table dancers",
        9 => "Nine years probation",
        10 => "Tin of copenhagen",
        11 => "Eleven wrastlin' tickets",
        12 => "Twelve pack of Bud",
        _ => "",
    };

    println!("{} {}", prefix, gifts);
}
