fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let presents = [
        "twelve drummers drumming",
        "eleven pipers piping",
        "ten lords a-leaping",
        "nine ladies dancing",
        "eight maids a-milking",
        "seven swans a-swimming",
        "six geese a-laying",
        "five golden rings",
        "four calling birds",
        "three French hens",
        "two turtle doves",
        "a partridge in a pear tree",
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[i]
        );

        let start = 11 - i;
        for j in start..12 {
            if j == 11 {
                println!("{}{}", if start < 11 { "and " } else { "" }, presents[j])
            } else {
                println!("{},", presents[j]);
            }
        }

        println!();
    }
}
