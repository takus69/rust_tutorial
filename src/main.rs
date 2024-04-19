fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let _word = first_word(&s);

    s.clear(); // error! （エラー！）

    let _s3 = &s;
    let _s2 = &mut s;

    let mut s = String::from("hello");

    let _r1 = &s; // 問題なし
    let _r2 = &s; // 問題なし
    let r3 = &mut s; // 大問題！

    println!("{}", r3);
    r3.push_str(", world");
    println!("{}", r3);
    // println!("{}", r1);


    // println!("the first word is: {}", word);
}
