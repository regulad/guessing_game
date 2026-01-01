fn main() {
    // range is exclusive of endpoint
    for i in 1..13 {
        let ordinal_day = match i {
            1 => "1st",
            2 => "2nd",
            3 => "3rd",
            // luckily since we don't go past the "teens" we don't need to properly check for the
            // last base 10 digit
            _ => &format!("{i}th"),
        };
        println!("On the {ordinal_day} of Christmas my true love gave to me");

        if i >= 12 {
            println!("Twelve drummers drumming");
        }
        if i >= 11 {
            println!("Eleven pipers piping");
        }
        if i >= 10 {
            println!("Ten lords a-leaping");
        }
        if i >= 9 {
            println!("Nine ladies dancing");
        }
        if i >= 8 {
            println!("Eight maids a-milking");
        }
        if i >= 7 {
            println!("Seven swans a-swiming");
        }
        if i >= 6 {
            println!("Six geese a-laying");
        }
        if i >= 5 {
            println!("Five golden rings");
        }
        if i >= 4 {
            println!("Four calling birds");
        }
        if i >= 3 {
            println!("Three french hens");
        }
        if i >= 2 {
            println!("Two turtle doves and");
        }
        println!("A partridge in a pear tree");
        println!(); // emit newline
    }
}
