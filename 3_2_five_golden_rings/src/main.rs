fn main() {
    let sings_strings = [
        ("first", "1" ),
        ("second", "2" ),
        ("삼", "3" ),
        ("사", "4" ),
        ("오", "5 golden rings" ),
        ("육", "6" ),
        ("칠", "7" ),
        ("팔", "8" ),
        ("구", "9" ),
        ("십", "10"),
        ("십일", "11"),
        ("십이", "12"),
    ];

    for (i, each_num) in sings_strings.iter().enumerate() {
        println!("크리스마스 {} 번째", each_num.0);

        for count in (0..(i+1)).rev(){
            println!("{}", sings_strings[count].1);
        }
    }
}