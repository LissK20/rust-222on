#[test]
fn test1()
{
    let s: &str = "hello, world";

    println!("Success!");
}

#[test]
fn test2_1()
{
    let s: Box<str> = "hello, world".into();
    greetings(&s); // Розіменовуємо Box, щоб отримати &str
}

fn greetings(s: &str)
{
    println!("{}", s);
}

#[test]
fn test2_2()
{
    let s: Box<str> = "hello, world".into();
    greetings2(s); // Передаємо Box<str> без змін
}

fn greetings2(s: Box<str>)
{
    println!("{}", s);
}

#[test]
fn test3()
{
    let mut s = String::new(); // Ініціалізуємо порожній рядок типу String
    s.push_str("hello, world"); // Додаємо рядок до змінної s
    s.push('!'); // Додаємо символ до змінної s

    assert_eq!(s, "hello, world!"); // Перевіряємо, чи рядок s дорівнює "hello, world!"

    println!("Success!");
}

#[test]
fn test4()
{
    let mut s = String::from("hello"); // Змінюємо на mut, оскільки змінна буде змінюватися
    s.push(',');
    s.push_str(" world"); // Використовуємо push_str для додавання рядка
    s += &"!".to_string(); // Додаємо посилання на рядок з методом to_string

    println!("{}", s);
}

#[test]
fn test5()
{
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats"); // Використовуємо метод replace

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

#[test]
fn test6()
{
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Додаємо посилання на s2

    assert_eq!(s3, "hello,world!");
    println!("{}", s2); // s1 більше не доступний, тому використовуємо s2
}


#[test]
fn test7_1()
{
    fn main()
    {
        let s = "hello, world";
        greetings(s);
    }

    fn greetings(s: &str)
    {
        println!("{}", s);
    }
}

#[test]
fn test7_2()
{
    fn main() {
        let s = "hello, world".to_string(); // Перетворюємо &str у String
        greetings(s);
    }

    fn greetings(s: String) {
        println!("{}", s);
    }
}

#[test]
fn test8_1()
{
    let s = "hello, world".to_string();
    let s1: &str = &s; // Взяти посилання на String

    println!("Success!");
}

#[test]
fn test8_2()
{
    let s = "hello, world".to_string();
    let s1: &str = s.as_str(); // Використовуємо метод as_str()

    println!("Success!");
}

#[test]
fn test9()
{
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73t!"; // \x73 це "s", додали "t"
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

#[test]
fn test10()
{
    let raw_str = "Escapes don't work here: ? ℝ"; // Звичайний рядок без екранування
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // Якщо потрібно вказати лапки в сирому рядку, додайте пару #.
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // Якщо потрібно включити "# у ваш рядок, просто використайте більше # в роздільнику.
    // Ви можете використовувати до 65535 #.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Заповніть пропуск
    let long_delimiter = r#"Hello, "##""#; // Використання сирого рядка для лапок і ##
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Успіх!");
}



#[test]
fn test11()
{
    let s1 = String::from("hi,中国");

    // Зміна цієї стрічки, щоб отримати перший символ
    let h = s1.chars().nth(0).unwrap(); // Використання `chars()` для отримання символа
    assert_eq!(h.to_string(), "h"); // Порівнюємо з рядком

    // Зміна цієї стрічки, щоб отримати символ "中"
    let h1 = &s1[3..4]; // Діапазон змінено на 3..4
    assert_eq!(h1, "中");

    println!("Успіх!");
}

#[test]
fn test12()
{
    // Заповніть пропуск, щоб надрукувати кожен символ у "你好，世界"
    for c in "你好，世界".chars()
    {
        println!("{}", c);
    }
}