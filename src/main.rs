
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
}
