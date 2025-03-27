



fn main() {
    println!("Lyrics to the Christmas Carol: Twelve days of Christmas:");

    const TOTAL_LINES:usize = 12;
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for i in 0..TOTAL_LINES{
        println!("On the {} day of Christmas\nMy true love gave to me", days[i]);

        for j in (0..=i).rev(){
            if j==0 && i!=0{
                println!("And {}", gifts[j].to_lowercase())
            }
            else{
                println!("{}",gifts[j]);
            }
        }
    }


}
