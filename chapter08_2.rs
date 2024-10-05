#[test]
fn test1()
{
    fn main()
    {
        match_number(3); // Ви можете протестувати різні значення
    }

    fn match_number(n: i32)
    {
        match n
        {
            // Match a single value
            1 => println!("One!"),
            // Fill in the blank with `|`, DON'T use `..` or `..=`
            2 | 3 | 4 | 5 => println!("match 2 -> 5"), // Використання | для об'єднання значень
            // Match an inclusive range
            6..=10 =>
                {
                println!("match 6 -> 10")
            },
            _ =>
                {
                println!("match -infinite -> 0 or 11 -> +infinite")
            }
        }
    }
}

#[test]
fn test2()
{
    struct Point
    {
        x: i32,
        y: i32,
    }

    fn main()
    {
        // Fill in the blank to let p match the second arm
        let p = Point { x: 3, y: 20 }; // Значення, що відповідає другій гілці

        match p
        {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            // Second arm
            Point { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
}

#[test]
fn test3()
{
    enum Message
    {
        Hello { id: i32 },
    }

    fn main()
    {
        let msg = Message::Hello { id: 5 };

        match msg
        {
            Message::Hello { id: 3..=7 } => println!("Found an id in range [3, 7]: {}", id),
            Message::Hello { id: newid @ (10 | 11 | 12) } =>
                { // Використання дужок для об'єднання
                println!("Found an id in another range [10, 12]: {}", newid);
            },
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}

#[test]
fn test4()
{
    let num = Some(4);
    let split = 5;
    match num
    {
        Some(x) if x < split => assert!(x < split), // Використання `if` для перевірки умови
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}

#[test]
fn test5()
{
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers
    {
        (first, .., last) =>
            { // Використання `..` для пропуску середніх значень
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }
    println!("Success!");
}

#[test]
fn test6()
{
    let mut v = String::from("hello,");
    let r = &mut v;

    match r
    {
        value => value.push_str(" world!") // Прибираємо `&mut` перед `value`
    }

    println!("{}", v); // Додаємо вивід для перевірки результату
}