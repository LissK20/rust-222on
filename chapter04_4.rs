#[test]
fn test1()
{
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32
{ // Додаємо тип для x та y, а також повертаємо i32
    x + y // Не ставимо крапку з комою, щоб повернути результат
}

#[test]
fn test2()
{
    print();
}
// Змінено тип повернення на ()
fn print() -> () {
    println!("Success!");
}

#[test]
fn test3()
{
    never_return();

    println!("Failed!");
}
fn never_return() -> !
{
    std::process::exit(1); // Завершує програму з кодом 1
}

#[test]
fn test4()
{
    println!("Success!");
}
fn get_option(tp: u8) -> Option<i32>
{
    match tp {
        1 => Some(42), // Повертаємо Some(42) для tp = 1
        _ => None, // Повертаємо None для інших значень
    }
}

fn never_return_fn() -> !
{
    std::process::exit(1); // Завершує програму з кодом виходу 1
}

#[test]
fn test5()
{
    // Заповнюємо пропущену змінну
    let b = true; // Ви можете також спробувати `false` для виклику паніки

    let _v = match b {
        true => 1, // Якщо b == true, повертаємо 1
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic"); // Паніка при значенні false
        }
    };

    println!("Exercise Failed if printing out this line!");
}