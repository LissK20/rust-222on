#[test]
fn test1()
{
    struct Person
    {
        name: String,
        age: u8,
        hobby: String,
    }

    fn main()
    {
        let age = 30;
        let p = Person
        {
            name: String::from("sunface"),
            age,
            hobby: String::from("coding"), // Додано ініціалізацію для hobby
        };

        println!("Успіх!");
    }

}

#[test]
fn test2()
{
    struct Unit;

    trait SomeTrait
    {
        // ...Some behaviors defined here.
    }

    // Ми не дбаємо про поля в Unit, але дбаємо про його поведінку.
    // Тому ми використовуємо структуру без полів і реалізуємо для неї деякі поведінки
    impl SomeTrait for Unit { }

    fn main()
    {
        let u = Unit;
        do_something_with_unit(u);

        println!("Успіх!");
    }

    // Заповніть пропуск, щоб код працював
    fn do_something_with_unit(u: Unit) { } // Тип параметра - Unit

}

#[test]
fn test3()
{
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    fn main()
    {
        let v = Point(0, 127, 255); // Ініціалізація Point з правильними значеннями
        check_color(Color(v.0, v.1, v.2)); // Передача значень з Point до Color

        println!("Успіх!");
    }

    fn check_color(p: Color)
    {
        let (x, y, z) = p; // Зберігаємо всі елементи
        assert_eq!(x, 0);
        assert_eq!(y, 127); // Використовуємо y для другого елемента
        assert_eq!(z, 255); // Використовуємо z для третього елемента
    }
}

#[test]
fn test4()
{
    struct Person
    {
        name: String,
        age: u8,
    }
    fn main()
    {
        let age = 18;
        let mut p = Person { // Додано `mut` для того, щоб `p` була змінною
            name: String::from("sunface"),
            age,
        };

        // Як ви можете вірити, що sunface тільки 18?
        p.age = 30;

        // Заповніть пропуск
        p.name = String::from("sunfei"); // Зміна значення поля `name`

        println!("Успіх!");
    }
}

#[test]
fn test5()
{
    struct Person
    {
        name: String,
        age: u8,
    }
    fn main()
    {
        println!("Success!");
    }

    fn build_person(name: String, age: u8) -> Person
    {
        Person
        {
            age,
            name, // Заповнюємо пропуск, вказуючи параметр `name`
        }
    }

}

#[test]
fn test6()
{
    struct User
    {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn main()
    {
        let u1 = User
        {
            email: String::from("someone@example.com"),
            username: String::from("sunface"),
            active: true,
            sign_in_count: 1,
        };

        let u2 = set_email(u1);

        println!("Success!");
    }

    fn set_email(u: User) -> User
    {
        User
        {
            email: String::from("contact@im.dev"),
            // Заповнюємо пропуски, копіюючи інші поля з `u`
            active: u.active,
            username: u.username,
            sign_in_count: u.sign_in_count,
        }
    }

}

#[test]
fn test7()
{
    #[derive(Debug)] // Додано атрибут для автоматичної реалізації трейту Debug
    struct Rectangle
    {
        width: u32,
        height: u32,
    }

    fn main()
    {
        let scale = 2;
        let rect1 = Rectangle
        {
            width: dbg!(30 * scale), // Виводить інформацію про `30 * scale` в stderr
            height: 50,
        };

        dbg!(&rect1); // Виводить інформацію про `rect1` в stderr

        println!("{:?}", rect1); // Виводить інформацію про `rect1` в stdout
    }
}

#[test]
fn test8()
{
    #[derive(Debug)]
    struct File
    {
        name: String,
        data: String,
    }

    fn main()
    {
        let f = File
        {
            name: String::from("readme.md"),
            data: "Rust By Practice".to_string(),
        };

        let _name = f.name;

        // ONLY modify this line
        println!("{}, {}, {:?}", _name, f.data, f); // Використовуємо _name замість f.name
    }

}

