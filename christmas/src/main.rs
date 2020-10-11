fn main() {
    let num = ["first", "second", "third", "fourth", "fifth", "sixth", 
                "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let mut index=1;
    let mut tmp=0;

    println!("The Twelve Days of Christmas\n");

    for element in num.iter()
    {
        println!("On the {} day of Christmas. My true love sent to me:", element);
        
        tmp = index;
        while tmp > 0
        {
            daylylics(tmp);
            tmp = tmp - 1
        }
        index += 1;
    }
}

fn daylylics(day : i32) {
    if      day==1  {   println!("A Partridge in a Pear Tree.\n");  }
    else if day==2  {   println!("Two Turtle Doves and");           }
    else if day==3  {   println!("Three French Hens");              }
    else if day==4  {   println!("Four Calling Birds");             }
    else if day==5  {   println!("Five Golden Rings");              }
    else if day==6  {   println!("Six Geese a Laying");             }
    else if day==7  {   println!("Seven Swans a Swimming");         }
    else if day==8  {   println!("Eight Maids a Milking");          }
    else if day==9  {   println!("Nine Ladies Dancing");            }
    else if day==10 {   println!("Ten Lords a Leaping");            }
    else if day==11 {   println!("Eleven Pipers Piping");           }
    else if day==12 {   println!("12 Drummers Drumming");           }
}
