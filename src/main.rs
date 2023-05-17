use std::collections::HashMap;
fn main() {
    // let number = 3;
    // if number < 5 {
    //     println!("condition was true");     //条件は真
    // } else {
    //     println!("condition was false");    //条件が偽
    // }

    // let number = 3;
    // if number {
    //     println!("number was three.");  //条件式がboolでないためエラー
    // }

    // let number = 6;
    // if number % 4 == 0{
    //     println!("number is divisible by 4");   //4で割れないため表示されない
    // } else if number % 3 == 0{
    //     println!("number is divisible by 3");   //3で割れるため表示される
    // } else if number % 2 == 0{
    //     println!("number is divisible by 2");   //２で割れるがelseif文のため表示されない
    // } else {
    //     println!("number is no divisible by 4, 3, or 2");   //表示されない
    // }

    // let condition = true;
    // let number = if condition {5} else {6};
    //
    // println!("The value of number is: {}", number); //numberの値は{}です

    // let mut x = 0;
    // loop{
    //     println!("{}again!",x); //無限ループ
    //     x = x + 1;
    // }

    // let mut count = 0;
    // 'counting_up: loop {                //外ループにループラベルcounting_upをつける
    //     println!("count = {}", count);
    //     let mut remaining = 10;

    //     loop {                          //内ループ
    //         println!("remaining = {}", remaining);
    //         if remaining == 9 {
    //             break;                  //内ループに適用される
    //         }
    //         if count == 2 {
    //             break 'counting_up;     //外ループcounting_upに適用される
    //         }
    //         remaining-= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {}", count);

    // let mut number = 3;
    // while number != 0 {
    //     println!("{}!", number);
    //     number -= 1;
    // }
    // println!("LIFTOFF!!!");

    // let a = [10, 20, 30, 40, 50];
    // for element in a {   //エラーを減らす
    //     println!("The value is: {}", element);
    // }

    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");

    //配列を作成する
    let numbering = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    //歌詞を表示する
    for number in 0..12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            numbering[number]
        );
        if number >= 11 {
            println!("Twelve drummers drumming");
        }
        if number >= 10 {
            println!("Eleven pipers piping");
        }
        if number >= 9 {
            println!("Ten lords a-leaping");
        }
        if number >= 8 {
            println!("Nine ladies dancing");
        }
        if number >= 7 {
            println!("Eight maids a-milking");
        }
        if number >= 6 {
            println!("Seven swans a-swimming");
        }
        if number >= 5 {
            println!("Six geese a-laying");
        }
        if number >= 44 {
            println!("Five golden rings");
        }
        if number >= 3 {
            println!("Four calling birds");
        }
        if number >= 2 {
            println!("Three French hens");
        }
        if number >= 1 {
            println!("Two turtledoves");
        }
        println!("And a partridge in a pear tree\n");
    }
    println!("And a partridge in a pear tree\n")
}
