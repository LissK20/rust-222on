#[test]

fn test1()
{
    let x: i32 = 5;
    let mut y = 5; // Видаляємо тип `u32`, компілятор виведе тип `i32`, щоб відповідати `x`

    y = x;

    let z = 10; // Тип z — це i32, так як компілятор виводить цілочисельні літерали як i32

    println!("Success!");
}

#[test]
fn test2()
{
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

#[test]
fn test3()
{
    let x = 5; // x має тип i32 за замовчуванням
    assert_eq!("i32".to_string(), type_of(&x)); // Оновлюємо очікуваний тип на i32

    println!("Успіх!");
}

fn type_of<T>(_: &T) -> String
{
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test4()
{
    assert_eq!(i8::MAX, 127); // Максимальне значення для i8
    assert_eq!(u8::MAX, 255); // Максимальне значення для u8

    println!("Успіх!");
}

#[test]
fn test5()
{
    let v1 = 251_u8.checked_add(8).unwrap_or_else(|_| 0); // Використовуємо checked_add для u8
    let v2 = i8::checked_add(251, 8).unwrap_or_else(|_| 0); // Використовуємо checked_add для i8

    println!("{}, {}", v1, v2); // Виведемо v1 та v2
}

#[test]
fn test6()
{
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // Додаємо різні числові значення
    assert!(v == 1579); // Перевіряємо, чи v дорівнює 1579

    println!("Success!");
}

#[test]
fn test7()
{
    let x: f64 = 1_000.000_1; // Явно вказуємо тип як f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string()); // Оновлюємо значення на "f64"
    println!("Success!");
}

fn type_of<T>(_: &T) -> String
{
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test8()
{
    let result = 0.1 + 0.2;
    let epsilon = 1e-10; // Допустима похибка

    assert!((result - 0.3).abs() < epsilon); // Порівнюємо з допустимою похибкою

    println!("Success!");
}

#[test]
fn test9()
{
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5); // Виправлено на -5

    // Виводимо числові значення символів 'a' до 'z'
    for c in 'a'..='z' {
        print!("{} ", c as u32); // Використовуємо print для виводу без нового рядка
    }

    // Додаємо останнє виведення в правильному форматі
    println!("- {}", ('a' as u32) - 1); // 'a' - 1 дорівнює 96

    println!("Success!");
}

#[test]
use std::ops::{Range, RangeInclusive};

fn test10()
{
    assert_eq!((1..5), Range { start: 1, end: 5 }); // Заповнено 5
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); // Заповнено 5

    println!("Success!");
}

#[test]
fn test11()
{
    // Integer addition
    assert!(1u32 + 2 == 3); // Заповнено 3

    // Integer subtraction
    assert!(1i32 - 2 == -1); // Заповнено -1
    assert!(1u8.wrapping_sub(2) == 255); // Використовуємо wrapping_sub, щоб уникнути паніки

    assert!(3 * 50 == 150); // Заповнено 150

    // Виправлення ділення з плаваючою комою
    assert!(9.6 / 3.2 == 3.0); // Це працює, оскільки обидва значення є f64

    assert!(24 % 5 == 4); // Заповнено 4
    // Short-circuiting boolean logic
    assert!(true && false == false); // Заповнено false
    assert!(true || false == true); // Заповнено true
    assert!(!true == false); // Заповнено false

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
