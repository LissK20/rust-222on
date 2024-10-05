#[test]
fn test1()
{
    let v = {
        let mut x = 1;
        x += 2; // Це не повертає значення
        x // Повертаємо x, щоб v отримав значення 3
    };

    assert_eq!(v, 3);

    println!("Success!");
}

#[test]

fn test2()
{
    let v = {
        let x = 3; // Використовуємо блок для створення змінної
        x // Повертаємо x
    };

    assert!(v == 3); // Тепер це буде вірно

    println!("Success!");
}

#[test]
fn test3()
{
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}
fn sum(x: i32, y: i32) -> i32
{
    x + y // Не ставимо крапку з комою, щоб повернути результат
}

