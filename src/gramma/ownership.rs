pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn no_way() -> ! {
    panic!("永不返回")
}

pub fn take_ownership_int(x: i32) {
    println!("{}", x);
}

pub fn take_ownership_string(s: String) {
    println!("{}", s);
}

pub fn giveback_ownership(s: String) -> String {
    println!("{}", s);
    s
}

pub fn ownership() {
    let x = 5;
    let y = x;
    // 直接在栈区复制, 相当于对该引用进行了拷贝, ( 从栈区开辟了一个新的内存存储 y ) 所以 x 对 5 的所有权不会消失 访问 x 不会有问题
    println!("x = {}, y = {}", x, y);
    // 输出不一样的地址
    println!("*x = {:p}, *y = {:p}", &x, &y);

    let s = String::from("Hello Rust");
    // 复杂变量, 在堆区拷贝
    let s1 = s;
    // 此语句报错，因为 "Hello Rust" 的所有权已经给了 s1, 所以 s 不再指向任何一个有效的值 (被释放了), 所以无法访问 s
    // println!("s = {}", s);

    // 深拷贝的操作, 相当于给 s2 开辟了新的空间 同时不影响 s1 的指向
    let s2 = s1.clone();
    assert_eq!(s1, s2);

    let x = 32;
    // 执行函数的时候, x 的所有权交给了这个 fn？还是说是函数从栈区新开辟了一块内存 ( 后者的可能性大一点 )
    take_ownership_int(x);
    println!("{}", x);

    let s = String::from("Hello rust");
    take_ownership_string(s);
    // 报错, 和上面的一样, s 的所有权被函数拿走了, 又因为函数执行之后被释放 ( drop ) , 所以此时已经无法访问 s
    // println!("{}", s);
    let mut s = String::from("Hello rust");
    s = giveback_ownership(s);
    println!("{}", s);

    // 获取变量的引用, 称为借用
    let s = String::from("Hello rust");
    borrow_test(&s);
    // 在 borrow 中只传递了 s 的不可变引用, 所以不影响 s 的所有权, s 依旧可以访问
    println!("{}", s);
    let mut s = String::from("Hello, ");
    // 传递可变引用进去, 所以会直接修改 s 的内容
    mut_borror_test(&mut s);
    println!("{}", s);

    // 可变引用只能存在一个, 但是不可变引用可以存在多个
}

pub fn borrow_test(s: &String) {
    println!("{}", s);
}

pub fn mut_borror_test(s: &mut String) {
    s.push_str("Rust");
}

pub fn variables_test() {
    let x = 5;
    // 重复绑定
    let x: i32 = x + 1;
    println!("{}", x);
    // 语句无返回值 但是表达式有
    let x: i32 = {
        let temp: i32  = x;
        temp * 2
    };

    println!("{}", x);

    println!("1 + 2 = {}", add(1, 2));
}

pub fn try_match_test()
{
    let _x = Some(13);

}