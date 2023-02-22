/**
 * 用户结构体
 */
#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
/** 元组结构题 */
#[allow(dead_code)]
struct Color(i32, i32, i32);

fn main() {
    // 下划线开头忽略未使用的变量、函数
    // format_print();
    // var_bind_destruct();
    // println!("计算结果: {}", statement_expression());
    // ownership();
    // complex_type();
    flow_control();

    // dead_end();
}

/**
 * 格式化输出
 */
fn _format_print() {
    let days = String::from("31天");
    println!("{} days", days);

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("Base 10:               {}", 69420); //69420
    println!("Base 2 (binary):       {:b}", 69420); //10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); //207454
    println!("Base 16 (hexadecimal): {:x}", 69420); //10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); //10F2C

    println!("{number:>5}", number = 1);

    println!("{number:0<5}", number = 1);

    println!("{number:0>width$}", number = 1, width = 5);

    println!("My name is {0}, {1} {0}", "Bond", "Ann");

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}

/**
 * 变量绑定与解构
 */
fn _var_bind_destruct() {
    // Rust 的变量在默认情况下是不可变的，一旦设置绑定值，就不能再修改。若要改变变量，则需要用关键字 mut 声明 let mut x = 10; （mut 只能修改变量值，不能修改变量类型）
    let x = 5;
    let mut y = 10;
    println!("x => {}, y => {}", x, y);
    y = 20;
    println!("x => {}, y => {}", x, y);

    // 变量解构
    let (mut a, b) = ("你好", 101);
    println!("a = {:?}, b = {:?}", a, b);
    a = "👋";
    println!("a = {:?}, b = {:?}", a, b);

    // 单元类型 () 作为一个值用来占位，但是完全不占用任何内存
}

/**
 * 语句（statement）和表达式（expression）
 *
 * 语句（statement）: 会执行一些操作但是不会返回一个值，结尾包含分号(;)
 * 表达式（expression）: 表达式会在一些操作后返回一个值，结尾不能包含分号(;)。表达式如果不返回任何值，会隐式地返回一个 ()
 *
 * 注意点：表达式不能包含分号。这一点非常重要，一旦你在表达式后加上分号，它就会变成一条语句，再也不会返回一个值，请牢记。
 */
fn _statement_expression() -> i32 {
    let (x, y) = (5, 78);
    x + y
}

/**
 * 所有权
 * 1. Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
 * 2. 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
 * 3. 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
 *
 * 任何基本类型的组合可以 Copy，不需要分配内存或某种形式资源的类型是可以 Copy 的。可变引用 &mut T 是不可以 Copy的。
 *
 * 引用和借用
 * 1. 引用 & 指向的值默认也是不可变的
 * 2. 同一作用域，特定数据只能有一个可变引用
 * 3. 可变引用与不可变引用不能同时存在
 * 4. 悬垂引用，悬垂引用也叫做悬垂指针，意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在，其指向的内存可能不存在任何值或已被其它变量重新使用。
 * 5. 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用；引用必须总是有效的
 */
fn _ownership() {
    let s1 = String::from("hello");
    let s2 = &s1; // 栈指针移动 move，s1 成为无效的引用
    let s3 = s1.clone(); // 克隆 clone（深拷贝）

    println!("{} - ({}->{}) - {}, world!", s1, s2, *s2, s3);

    let x: &str = "hello, world";
    let y = x; // 拷贝
    println!("{},{}", x, y);

    let mut s = String::from("可变值");
    let rs = &mut s;
    println!("==> {}", rs);
    _change(rs);
    println!("==> {}", s);
}

fn _change(str: &mut String) {
    str.push_str(", 同一作用域，特定数据只能有一个可变引用");
}

/**
 * 复合类型
 *
 * 字符串字面值是不可变的。就字符串字面值来说，我们在编译时就知道其内容，最终字面值文本被直接硬编码进可执行文件中，这使得字符串字面值快速且高效，这主要得益于字符串字面值的不可变性。
 *
 */
fn _complex_type() {
    println!("========================= 字符串 ================================");
    let s = String::from("Hello world");
    let len = s.len();
    // 切片
    let hello = &s[0..5];
    let world = &s[6..len]; // 或者 &s[6..]
    println!("==> {} - {}", hello, world);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    // 字符串采用UTF-8编码，一个汉字占3个字节
    let mut ch_str = String::from("中国人");
    ch_str.push_str("！");
    println!("{} ==> {}", ch_str, &ch_str[0..3]);
    ch_str.clear();
    println!("Clear后：{}", ch_str);

    // 连接字符串
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";

    println!("连接字符串 + -> {}", result);

    // 遍历 UTF-8 字符串
    for c in "中国人".chars() {
        println!("{}", c);
    }

    println!("========================= 元组 =================================");
    // 元组是由多种类型组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组中元素的顺序也是固定的
    let tup = (500, 6.4, 100);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}, z is: {}", y, tup.2);

    println!("========================= 结构体 ================================");
    // 1. 初始化实例时，每个字段都需要进行初始化
    // 2. 初始化时的字段顺序不需要和结构体定义时的顺序一致
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("但丁"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("dante7qx@gmail.com");
    println!("用户1 -> {:?}", user1);

    // 结构体更新语法
    // 把结构体中具有所有权的字段转移出去后，将无法再访问该字段，但是可以正常访问其它的字段。(user1 的 email 就无法访问了)
    let user2 = User {
        username: String::from("尤娜"),
        ..user1
    };
    println!("用户2 -> {:?}", user2);

    let black = Color(0, 0, 0);
    println!("黑色 -> {}", black.0);

    println!("========================= 数组 =================================");
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("月份 => {:?} \r\n6-9月: {:?}", months, &months[5..8]);
}

/**
 * 当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )，特别的，这种语法往往用做会导致程序崩溃的函数
 */
fn _dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}

/**
 * 流程控制
 *
 */
fn flow_control() {
    println!("========================= 表达式 ================================");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is: {}", number);

    let n = 28;
    if n % 4 == 0 {
        println!("被4整除！");
    } else if n % 3 == 0 {
        println!("被3整除！");
    } else if n % 2 == 0 {
        println!("被2整除！");
    } else {
        println!("不能被4、3、2整除！");
    }

    for i in 1..=5 {
        println!("{}", i)
    }

    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }

    let mut connter = 0;
    loop {
        connter += 1;
        if connter == 5 {
            println!("跳出loop循环。");
            break;
        }
        
        println!("当前计数: {}", connter);
    }

    // 模式匹配
    /*
    match target {
        模式1 => 表达式1,
        模式2 => {
            语句1;
            语句2;
            表达式2
        },
        _ => 表达式3    （默认方式）
    }
    */
    let dire = "WEST";
    match dire {
        "EAST" => println!("东方"),
        "WEST" => println!("西方"),
        "SOURTH" => println!("南方"),
        "NORTH" => println!("北方"),
        _ => println!("未知方向"),
    };
}
