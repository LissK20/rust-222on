#[test]
fn test1()
{
    enum Number
    {
        Zero,
        One,
        Two,
    }

    enum Number1
    {
        Zero = 0,
        One = 1,
        Two = 2,
    }

    // C-like enum
    enum Number2
    {
        Zero = 0,
        One = 1,
        Two = 2,
    }

    fn main()
    {
        // An enum variant can be converted to a integer by `as`
        assert_eq!(Number1::One as i32, Number2::One as i32); // Порівнюємо значення як цілі числа
        assert_eq!(Number::One as i32, Number1::One as i32); // Порівнюємо значення як цілі числа

        println!("Success!");
    }

}

#[test]
fn test2()
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
        let msg1 = Message::Move { x: 1, y: 2 }; // Ініціалізація з x = 1, y = 2
        let msg2 = Message::Write(String::from("hello, world!")); // Ініціалізація з "hello, world!"

        println!("Success!");
    }
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
        let msg = Message::Move { x: 1, y: 2 };

        if let Message::Move { x, y } = msg
        {
            assert_eq!(x, 1); // Перевіряємо, що x дорівнює 1
            assert_eq!(y, 2); // Перевіряємо, що y дорівнює 2
        }
        else
        {
            panic!("NEVER LET THIS RUN！");
        }
        println!("Success!");
    }
}

#[test]
fn test4()
{
    use std::fmt; // Додаємо імпорт для формату

    enum Message
    {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Реалізуємо трейт Display для enum Message
    impl fmt::Display for Message
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            match self
            {
                Message::Quit => write!(f, "Quit"),
                Message::Move { x, y } => write!(f, "Move to ({}, {})", x, y),
                Message::Write(text) => write!(f, "Write: {}", text),
                Message::ChangeColor(r, g, b) => write!(f, "Change color to RGB({}, {}, {})", r, g, b),
            }
        }
    }

    fn main()
    {
        let msgs: [Message; 3] = [ // Вказуємо тип масиву як [Message; 3]
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0),
        ];

        for msg in msgs
        {
            show_message(msg);
        }
    }

    fn show_message(msg: Message)
    {
        println!("{}", msg); // Тепер це працює, оскільки реалізовано Display
    }
}

#[test]
fn test5()
{
    fn main()
    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        if let Some(n) = six
        { // Перевіряємо, чи є six Some(n)
            println!("{}", n); // Друкуємо n, якщо six не None

            println!("Success!");
        }
        // Переміщуємо panic в else, щоб уникнути його виконання при Some(n)
        else
        {
            panic!("NEVER LET THIS RUN！"); // Тепер це не запуститься, якщо six є Some
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32>
    {
        match x
        {
            None => None, // Повертаємо None, якщо x — None
            Some(i) => Some(i + 1), // Додаємо 1 до i і повертаємо
        }
    }
}

#[test]
fn test6()
{
    use crate::List::*;

    enum List
    {
        // Cons: Tuple struct that wraps an element and a pointer to the next node
        Cons(u32, Box<List>),
        // Nil: A node that signifies the end of the linked list
        Nil,
    }

    // Methods can be attached to an enum
    impl List
    {
        // Create an empty list
        fn new() -> List
        {
            // `Nil` has type `List`
            Nil
        }

        // Consume a list, and return the same list with a new element at its front
        fn prepend(self, elem: u32) -> List
        {
            // `Cons` also has type List
            Cons(elem, Box::new(self))
        }

        // Return the length of the list
        fn len(&self) -> u32
        {
            match *self
            {
                Cons(_, ref tail) => 1 + tail.len(),
                Nil => 0,
            }
        }

        // Return representation of the list as a (heap allocated) string
        fn stringify(&self) -> String
        {
            match *self {
                Cons(head, ref tail) =>
                    {
                    // Format the string representation
                    format!("{}, {}", head, tail.stringify())
                },
                Nil =>
                    {
                    format!("Nil")
                },
            }
        }
    }

    // Testing the linked list functionality
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Assert the length of the list
    assert_eq!(list.len(), 3);
    assert_eq!(list.stringify(), "3, 2, 1, Nil");
}
