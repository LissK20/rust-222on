#[test]
fn test()
{
    assert_eq!(2 + 3, 5);
}

#[test]
fn test1()
{
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

#[test]
fn test2()
{
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

#[test]
fn test3()
{
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
        println!("The value of x is {} and value of y is {}", x, y);
    }

}

#[test]
fn test4()
{
    let x = define_x();  // Виклик функції для отримання значення x
    println!("{}, world", x);
}
fn define_x() -> &'static str
{
    let x = "hello";
    x  // Повернення значення
}

#[test]
fn test5()
{
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); // Тут змінна `x` всередині блоку має значення 12
    }

    assert_eq!(x, 5); // Тут зовнішня змінна `x` все ще має значення 5

    let x = 42;
    println!("{}", x); // Виводить "42"
}

#[test]
fn test6()
{
    let mut x: i32 = 1;
    x = 7;
    // Перекриття і перев'язування змінної
    let x = x;

    let y = 4;
    // Перекриття змінної
    let y = "Я також можу бути прив'язаним до тексту!";

    println!("Success!");
}

#[test]
fn test7_1()
{
    let _x = 1; // Префікс `_` ігнорує попередження
}
fn test7_2()
{
    let x = 1;
    dbg!(x); // Використання для відладки, що не впливає на результат програми
}

#[test]
fn test8()
{
    let (mut x, y) = (1, 2); // Робимо x змінною за допомогою `mut`
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

#[test]
fn test9()
{
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // Заповнюємо порожнє місце для правильного порівняння
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}