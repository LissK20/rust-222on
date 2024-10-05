#[test]
fn test1()
{
    let _t0: (u8, i16) = (0, -1);
    // Кортежі можуть містити інші кортежі
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Заповніть пропуски, щоб код працював
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Успіх!");
}

#[test]
fn test2()
{
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface"); // Змінено з t.1 на t.2

    println!("Успіх!");
}

#[test]
fn test3()
{
    let too_long_tuple = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]; // Використання вектора
    println!("too long tuple: {:?}", too_long_tuple);
}

#[test]
fn test4()
{
    let tup = (1, 6.4, "hello");

    // Заповніть пропуск, щоб код працював
    let (x, z, y) = tup; // Розпаковка кортежу в змінні

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Успіх!");
}

#[test]
fn test5()
{
    let (x, y, z);

    // Заповніть пропуск
    (y, z, x) = (1, 2, 3); // Розпаковка зворотного порядку для відповідності з assert_eq!

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Успіх!");
}

#[test]
fn test6()
{
    fn main()
    {
        // Заповніть пропуск, потрібно кілька обчислень тут.
        let (x, y) = sum_multiply((2, 3)); // Передача кортежу (2, 3)

        assert_eq!(x, 5);
        assert_eq!(y, 6);

        println!("Успіх!");
    }

    fn sum_multiply(nums: (i32, i32)) -> (i32, i32)
    {
        (nums.0 + nums.1, nums.0 * nums.1)
    }

}
