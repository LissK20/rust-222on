#[test]
fn test1()
{
    let n = 5;

    if n < 0
    {
        println!("{} is negative", n);
    }
    else if n > 0
    {
        println!("{} is positive", n);
    }
    else
    {
        println!("{} is zero", n);
    }
}

#[test]
fn test2()
{
    let n = 5;

    let big_n =
        if n < 10 && n > -10
        {
            println!(", and is a small number, increase ten-fold");

            10 * n
        }
        else
        {
            println!(", and is a big number, halve the number");

            n / 2  // Замінили 2.0 на 2, щоб уникнути проблеми з типами
        };

    println!("{} -> {}", n, big_n);
}

#[test]
fn test3()
{
    for n in 1..100
    { // Змінили на 1..100, щоб цикл не включав 100
        if n == 100
        {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
}

#[test]
fn test4()
{
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names
    { // Використовуємо посилання на елементи масиву
        // Do something with name...
    }

    println!("{:?}", names); // Тепер можна використовувати names

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy, so there is no move here
    for n in numbers
    {
        // Do something with n...
    }

    println!("{:?}", numbers); // Numbers можна використовувати, бо елементи Copy
}

#[test]
fn test5()
{
    // Змінна лічильника
    let mut n = 1;

    // Цикл, поки умова істинна
    while n <= 10
    {
        if n % 15 == 0
        {
            println!("fizzbuzz");
        }
        else if n % 3 == 0
        {
            println!("fizz");
        }
        else if n % 5 == 0
        {
            println!("buzz");
        }
        else
        {
            println!("{}", n);
        }

        n += 1; // Збільшуємо значення n
    }

    println!("n досягнуло {}, тому цикл закінчено", n);
}

#[test]
fn test6()
{
    // Змінна лічильника
    let mut n = 1;

    // Цикл, поки умова істинна
    while n <= 10
    {
        if n % 15 == 0
        {
            println!("fizzbuzz");
        }
        else if n % 3 == 0
        {
            println!("fizz");
        }
        else if n % 5 == 0
        {
            println!("buzz");
        }
        else
        {
            println!("{}", n);
        }
        n += 1; // Збільшуємо значення n
    }

    println!("n досягнуло {}, тому цикл закінчено", n);
}

#[test]
fn test7()
{
    let mut n = 0;
    for i in 0..=100
    {
        if n == 66
        {
            break; // Завершуємо цикл, коли n дорівнює 66
        }
        n += 1;
    }

    assert_eq!(n, 66);
    println!("Success!");
}

#[test]
fn test8()
{
    let mut n = 0;
    for i in 0..=100
    {
        if n != 66
        {
            n += 1;
            continue; // Пропускаємо залишок циклу, якщо n не дорівнює 66
        }

        break; // Виходимо з циклу, коли n дорівнює 66
    }

    assert_eq!(n, 66);
    println!("Success!");
}

#[test]
fn test9()
{
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop
    {
        count += 1;

        if count == 3
        {
            println!("three");

            // Skip the rest of this iteration
            continue; // Пропустити решту ітерації
        }

        println!("{}", count);

        if count == 5
        {
            println!("OK, that's enough");

            break; // Вийти з циклу
        }
    }

    assert_eq!(count, 5);
    println!("Success!");
}

#[test]
fn test10()
{
    let mut counter = 0;

    let result = loop
    {
        counter += 1;

        if counter == 10
        {
            break 20; // Виходимо з циклу і повертаємо 20
        }
    };

    assert_eq!(result, 20);
    println!("Success!");
}

#[test]
fn test11()
{
    let mut count = 0;
    'outer: loop
    {
        'inner1: loop
        {
            if count >= 20
            {
                // This would break only the inner1 loop
                break 'inner1; // `break` is also works.
            }
            count += 2;
        }

        count += 5;

        'inner2: loop
        {
            if count >= 30
            {
                // This breaks the outer loop
                break 'outer;
            }

            // This will continue the outer loop
            continue 'outer;
        }
    }

    assert!(count == 30); // Заповнено пропуск
    println!("Success!");
}