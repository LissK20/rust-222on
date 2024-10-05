#[test]
fn test1()
{
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2]; // Змінено на посилання на частину масиву

    let s2: &str = "hello, world"; // Змінено на посилання на рядок

    println!("Успіх!");
}

#[test]
fn test2()
{
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // Зміна '8' на '16', оскільки розмір зрізу складається з вказівника (8 байт) і довжини (8 байт) на 64-бітній системі
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Успіх!");
}

#[test]
fn test3()
{
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Заповніть пропуски, щоб код працював
    let slice: &[i32] = &arr[1..4]; // Вказано тип зрізу і діапазон

    assert_eq!(slice, &[2, 3, 4]);

    println!("Успіх!");
}

#[test]
fn test4()
{
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Заповніть пропуск, щоб код працював, не використовуючи 0..2 знову
    let slice2 = &s[..2]; // Використано синтаксис для зрізу з початку до індексу 2

    assert_eq!(slice1, slice2);

    println!("Успіх!");
}

#[test]
fn test5()
{
    let s = "你好，世界";
    // Виправте цей рядок, щоб код працював
    let slice = &s[0..3]; // Зміни: береться 3 байти, щоб отримати символ "你"

    assert!(slice == "你");

    println!("Успіх!");
}

#[test]
fn test6()
{
    fn main()
    {
        let mut s = String::from("hello world");

        let letter = first_letter(&s);

        // Перемістіть очистку рядка нижче
        println!("the first letter is: {}", letter);

        s.clear(); // Тепер це не викличе помилок

    }

    fn first_letter(s: &str) -> &str
    {
        &s[..1]
    }
}