#[test]
fn test1()
{
    trait Bird
    {
        fn quack(&self) -> String;
    }

    struct Duck;

    impl Duck
    {
        fn swim(&self)
        {
            println!("Look, the duck is swimming")
        }
    }

    struct Swan;

    impl Swan
    {
        fn fly(&self)
        {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }

    impl Bird for Duck
    {
        fn quack(&self) -> String
        {
            "duck duck".to_string()
        }
    }

    impl Bird for Swan
    {
        fn quack(&self) -> String
        {
            "swan swan".to_string()
        }
    }

    fn main()
    {
        // FILL in the blank.
        let duck = Duck; // Тут ми створюємо об'єкт Duck
        duck.swim();

        let bird = hatch_a_bird(2);
        // This bird has forgotten how to swim, so below line will cause an error.
        // bird.swim();
        // But it can quak.
        assert_eq!(bird.quack(), "duck duck");

        let bird = hatch_a_bird(1);
        // This bird has forgotten how to fly, so below line will cause an error.
        // bird.fly();
        // But it can quak too.
        assert_eq!(bird.quack(), "swan swan");

        println!("Success!");
    }

    // IMPLEMENT this function.
    fn hatch_a_bird(num: i32) -> Box<dyn Bird>
    { // Використовуємо Box для динамічного типу
        if num == 1
        {
            Box::new(Swan) // Повертаємо новий Swan
        }
        else
        {
            Box::new(Duck) // Повертаємо новий Duck
        }
    }
}

#[test]
fn test2()
{
    trait Bird
    {
        fn quack(&self);
    }

    struct Duck;
    impl Duck
    {
        fn fly(&self)
        {
            println!("Look, the duck is flying")
        }
    }
    struct Swan;
    impl Swan
    {
        fn fly(&self)
        {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }

    impl Bird for Duck
    {
        fn quack(&self)
        {
            println!("{}", "duck duck");
        }
    }

    impl Bird for Swan
    {
        fn quack(&self)
        {
            println!("{}", "swan swan");
        }
    }

    fn main()
    {
        // FILL in the blank to make the code work.
        let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck), Box::new(Swan)]; // Використовуємо Box для динамічного типу

        for bird in birds
        {
            bird.quack();
            // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
            // So, the code below will cause an error.
            // bird.fly(); // Це викличе помилку компіляції, оскільки bird не знає, як літати.
        }
    }
}

#[test]
fn test3()
{
    trait Draw
    {
        fn draw(&self) -> String;
    }

    impl Draw for u8
    {
        fn draw(&self) -> String
        {
            format!("u8: {}", *self)
        }
    }

    impl Draw for f64
    {
        fn draw(&self) -> String
        {
            format!("f64: {}", *self)
        }
    }

    fn main()
    {
        let x = 1.1f64;
        let y = 8u8;

        // Draw x.
        draw_with_box(Box::new(x)); // Використовуємо Box для динамічного типу

        // Draw y.
        draw_with_ref(&y); // Передаємо посилання на y

        println!("Success!");
    }

    fn draw_with_box(x: Box<dyn Draw>)
    {
        println!("{}", x.draw()); // Додаємо вивід результату
    }

    fn draw_with_ref(x: &dyn Draw)
    { // Використовуємо посилання на динамічний трейт
        println!("{}", x.draw()); // Додаємо вивід результату
    }
}


#[test]
fn test4()
{
    trait Foo
    {
        fn method(&self) -> String;
    }

    impl Foo for u8
    {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }

    impl Foo for String
    {
        fn method(&self) -> String { format!("string: {}", *self) }
    }

    // IMPLEMENT below with generics.
    fn static_dispatch<T: Foo>(item: T)
    {
        println!("{}", item.method());
    }

    // Implement below with trait objects.
    fn dynamic_dispatch(item: &dyn Foo)
    {
        println!("{}", item.method());
    }

    fn main()
    {
        let x = 5u8;
        let y = "Hello".to_string();

        static_dispatch(x);
        dynamic_dispatch(&y);

        println!("Success!");
    }
}

#[test]
fn test5()
{
    trait MyTrait
    {
        fn f(&self) -> Self;
    }

    impl MyTrait for u32
    {
        fn f(&self) -> Self { 42 }
    }

    impl MyTrait for String
    {
        fn f(&self) -> Self { self.clone() }
    }

    // Додатковий підхід 1: Реалізація для i32
    impl MyTrait for i32
    {
        fn f(&self) -> Self { 99 }
    }

    // Додатковий підхід 2: Реалізація для Vec<u8>
    impl MyTrait for Vec<u8>
    {
        fn f(&self) -> Self { self.clone() }
    }

    fn my_function(x: Box<dyn MyTrait>)
    {
        x.f()
    }

    fn main()
    {
        my_function(Box::new(13_u32));
        my_function(Box::new(String::from("abc")));

        println!("Success!");
    }
}

