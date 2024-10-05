#[test]
fn test1()
{
    let x = 5;
    // Заповніть пропуск
    let p = &x; // Створюємо посилання на x

    println!("the memory address of x is {:p}", p); // Один з можливих виходів: 0x16fa3ac84
}

#[test]
fn test2()
{
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y); // Розіменовуємо y, щоб отримати значення

    println!("Success!");
}

#[test]
fn test3()
{
    let mut s = String::from("hello, ");

    borrow_object(&s); // Передаємо посилання на s

    println!("Success!");
}

fn borrow_object(s: &String)
{
    // Можна додати логіку тут, якщо потрібно
}

#[test]
fn test4()
{
    let mut s = String::from("hello, ");

    push_str(&mut s); // Передаємо мутабельне посилання на s

    println!("Success! {}", s); // Додано вивід s для підтвердження успішного виконання

}
fn push_str(s: &mut String)
{
    s.push_str("world"); // Додаємо "world" до s
}

#[test]
fn test5()
{
    let mut s = String::from("hello, ");

    // Заповніть пропуск, щоб створити мутабельне посилання на s
    let p = &mut s; // Створюємо мутабельне посилання на s

    p.push_str("world"); // Додаємо "world" до s

    println!("Success!");
}

#[test]
fn test6()
{
    let c = '中';

    let r1 = &c;
    // Заповніть пропуск
    let r2: char = c; // Оголошуємо r2 як символ

    assert_eq!(*r1, r2);

    // Перевірка рівності двох адрес
    assert_eq!(get_addr(r1), get_addr(&r2)); // Передаємо посилання на r2

    println!("Success!");
}


// Отримання рядка адреси пам'яті
fn get_addr(r: &char) -> String
{
    format!("{:p}", r)
}

#[test]
fn test7()
{
    let mut s = String::from("hello");

    let r1 = &mut s;
    // Видаліть або коментуйте наступний рядок, щоб уникнути конфлікту
    // let r2 = &mut s;

    println!("{}", r1); // Друкуємо тільки r1

    println!("Success!");
}

#[test]
fn test8()
{
    // Виправте помилку, змінивши цей рядок
    let mut s = String::from("hello, "); // Додано mut для змінної s

    borrow_object(&mut s); // Передаємо мутабельне посилання

    println!("Success!");
}

fn borrow_object(s: &mut String)
{
    // Можна додати логіку тут, якщо потрібно
}


#[test]
fn test9()
{
    let mut s = String::from("hello, ");

    borrow_object(&mut s); // Передаємо мутабельне посилання

    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &mut String)
{
    // Можна змінити s тут, якщо потрібно
}

#[test]
fn test10()
{
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    // закоментуйте або видаліть наступний рядок
    // let r2 = &mut s; // Не можна мати r2, коли r1 все ще активне
    // r2.push_str("!");

    println!("{}", r1);
}

#[test]
fn test11()
{
    let mut s = String::from("hello, ");

    let r1 = &mut s; // r1 - мутабельне посилання на s
    let r2 = &mut s; // Цей рядок викликає помилку

    // Додаємо рядок нижче, щоб викликати помилку компіляції:
    r1.push_str("world"); // Використання r1
}