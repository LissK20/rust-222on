#[test]
fn test1()
{
    enum Direction
    {
        East,
        West,
        North,
        South,
    }

    fn main()
    {
        let dire = Direction::South;
        match dire
        {
            Direction::East => println!("East"),
            Direction::South | Direction::North =>
                { // Співставлення для South або North
                println!("South or North");
            },
            _ => println!("West"), // Обробка для West
        };
    }
}

#[test]
fn test2()
{
    let boolean = true;

    // Використовуємо match для перетворення boolean в ціле число
    let binary = match boolean
    {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);
    println!("Success!");
}

#[test]
fn test3()
{
    enum Message
    {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn main()
    {
        let msgs = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0),
        ];

        for msg in msgs
        {
            show_message(msg)
        }

        println!("Success!");
    }

    fn show_message(msg: Message)
    {
        match msg
        {
            Message::Move { x: a, y: b } =>
                { // Співставлення Message::Move
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            },
            Message::ChangeColor(_, g, b) =>
                {
                assert_eq!(g, 255); // Співставляємо g з 255
                assert_eq!(b, 0);   // Співставляємо b з 0
            }
            Message::Quit | Message::Write(_) => println!("no data in these variants"), // Обробляємо інші варіанти
        }
    }
}

#[test]
fn test4()
{
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    // Використовуємо matches! для перевірки, чи є символ великою літерою
    for ab in alphabets
    {
        assert!(matches!(ab, 'A'..='Z'))
    }

    println!("Success!");
}

#[test]
fn test5()
{
    enum MyEnum
    {
        Foo,
        Bar,
    }

    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in &v
    {
        if *e == MyEnum::Foo
        {
            count += 1;
        }
    }

    // Перевіряємо, що кількість Foo дорівнює 2
    assert_eq!(count, 2);
}

#[test]
fn test6()
{
    let o = Some(7);

    // Використовуємо if let замість match
    if let Some(i) = o
    {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }
}

#[test]
fn test7()
{
    enum Foo
    {
        Bar(u8),
    }

    fn main()
    {
        let a = Foo::Bar(1);

        // Використовуємо if let для вилучення значення з Foo::Bar
        if let Foo::Bar(i) = a
        {
            println!("foobar holds the value: {}", i);
            println!("Success!");
        }
    }
}

#[test]
fn test8()
{
    enum Foo
    {
        Bar,
        Baz,
        Qux(u32),
    }

    fn main()
    {
        let a = Foo::Qux(10);

        match a
        {
            Foo::Bar => println!("match foo::bar"),
            Foo::Baz => println!("match foo::baz"),
            _ => println!("match others"),
        }
    }
}

#[test]
fn test9()
{
    let age = Some(30);
    if let Some(a) = age
    { // Зміна імені змінної в if let
        assert_eq!(a, 30); // Порівнюємо з 30 без використання Some
    } // Змінна `a` виходить з області видимості тут

    match age
    {
        // Затінюємо змінну `age` в match
        Some(b) => println!("age is a new variable, it's value is {}", b),
        _ => ()
    }
}