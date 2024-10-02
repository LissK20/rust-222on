#[test]
use std::mem::size_of_val;

fn test1()
{
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4); // Розмір символу 'a' дорівнює 4 байти

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4); // Розмір символу '中' також дорівнює 4 байти

    println!("Success!");
}

#[test]
fn test2()
{
    let c1 = "中"; // Рядок, який містить один символ
    print_char(c1.chars().next().unwrap()); // Отримуємо перший символ і передаємо його

}

fn print_char(c: char)
{
    println!("{}", c);
}

#[test]
fn test3()
{
    let _f: bool = false;

    let t = false; // Змінено на false, щоб умова стала істинною
    if !t
    {
        println!("Success!"); // Тепер це буде виконано
    }
}

#[test]
fn test4()
{
    let f = true;
    let t = true && true; // Змінено на true, щоб t дорівнювало f
    assert_eq!(t, f);

    println!("Success!");
}

#[test]
fn test5()
{
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, (2, 3)); // Порівнюємо кортеж v з кортежем (2, 3)

    println!("Success!");
}

fn implicitly_ret_unit()
{
    println!("I will return a ()");
}

#[test]
use std::mem::size_of_val;
fn test6()
{
    let unit: () = ();
    assert!(size_of_val(&unit) == 1); // Змінено 4 на 1

    println!("Success!");
}