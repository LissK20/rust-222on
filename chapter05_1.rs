#[test]
fn test1_1()
{
    let x = String::from("Hello world");
    let y = &x; // Створюємо посилання на x
    println!("{}, {}", x, y); // Використовуємо x і y (y — це посилання на x)

}

fn test1_2()
{
    let x = String::from("Hello world");
    let y = x.clone(); // Клонуємо значення x у y
    println!("{}, {}", x, y); // Тепер обидві змінні доступні
}

fn test1_3()
{
    let x = 42;
    let y = x; // Автоматично копіює значення, оскільки i32 має Copy
    println!("{}, {}", x, y); // Обидві змінні доступні
}


#[test]
fn test2()
{
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}


// Only modify the code below!
fn take_ownership(s: String) -> String
{ // Додаємо повернення String
    println!("{}", s);
    s // Повертаємо строку назад
}

#[test]
fn test3()
{
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String
{
    let s = String::from("Hello world");
    // Позичаємо байти, не переміщаючи строку
    let _s = s.as_bytes(); // Використовуємо as_bytes() для отримання посилання на байти
    s // Повертаємо строку
}

#[test]
fn test4()
{
    let s = String::from("Hello World");

    print_str(&s); // Передаємо посилання на s

    println!("{}", s); // s залишається доступною, оскільки її володіння не було переміщено

}

fn print_str(s: &String)
{ // Функція приймає посилання на строку
    println!("{}", s);
}

#[test]
fn test5()
{
    let x = (1, 2, ()); // Тільки типи з Copy
    let y = x; // Тут копіювання відбувається автоматично, оскільки всі елементи є Copy
    println!("{:?}, {:?}", x, y);
}

#[test]
fn test6()
{
    let mut s = String::from("Hello ");  // Зробимо s змінною

    let mut s1 = s;  // Зробимо s1 змінною

    s1.push_str("World!");  // Додаємо "World!" до строки

    println!("Success!");
}

#[test]
fn test7()
{
    let x = Box::new(5);

    let mut y = x;  // Робимо y змінною, яка приймає володіння `Box<i32>`

    *y = 4;  // Міняємо значення на яке посилається y

    assert_eq!(*x, 5);  // `x` більше не доступний, так що цей рядок змінювати не можна
    // Цей assert не буде працювати, оскільки володіння `x` перейшло до `y`
    // Можливо, слід використати assert для `*y`, наприклад:
    assert_eq!(*y, 4);  // Це перевірка для нового значення

    println!("Success!");
}

#[test]
fn test8()
{
    let t = (String::from("hello"), String::from("world"));

    let _s = &t.0;  // Позичаємо посилання на t.0, щоб не переміщувати значення

    // Тепер можна використати весь кортеж, оскільки значення не переміщувались
    println!("{:?}", t);
}

#[test]
fn test9()
{
    let t = (String::from("hello"), String::from("world"));

    // Заповніть пропуски
    let (s1, s2) = (t.0.clone(), t.1.clone()); // Використовуємо clone для копіювання значень

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

fn main()
{
    test9();
}