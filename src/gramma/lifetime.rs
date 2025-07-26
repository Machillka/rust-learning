// 返回的是对 x 或 y 的引用, 不能保证返回的引用生命周期可以保持
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 在堆区里开辟一个新的区域存储 res, 并且移交所有权，规避了对引用的生命周期问题
pub fn string_longest(x: &String, y: &String) -> String {
    let res = if x.len() > y.len() {
        x.clone()
    } else {
        y.clone()
    };

    res
}

#[test]
pub fn lifetime_test() {
    let x = String::from("Fuck");
    let l;
    {
        let y = "Hello world!";
        l = longest(x.as_str(), &y);
    }
    println!("{l}");
    let l = string_longest(&x, &String::from("Hello world!"));

    println!("{l}");
}
