fn main() {

    for number in 1..10 {
        fibonacci(number);
    }

}

fn fibonacci(index : i32) {
    let mut first = 0;
    let mut second = 1;
    let mut third = first + second;

    if index==1         { println!("The result is : {}", first); }
    else if index==2    { println!("The result is : {}", second); }
    else if index>2
    {    
        for _number in 1..index {
            third = first + second;
            
            first = second;
            second = third;
        }

        println!("The result is : {}", third);
    }
}