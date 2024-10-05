#[test]
fn test1()
{
    struct Rectangle
    {
        width: u32,
        height: u32,
    }

    impl Rectangle
    {
        // Complete the area method which returns the area of a Rectangle.
        fn area(&self) -> u32
        { // Додаємо параметр self і вказуємо, що метод повертає u32
            self.width * self.height // Обчислюємо площу
        }
    }

    fn main()
    {
        let rect1 = Rectangle { width: 30, height: 50 };

        assert_eq!(rect1.area(), 1500);
        println!("Success!");
    }
}

#[test]
fn test2()
{
    #[derive(Debug)]
    struct TrafficLight
    {
        color: String,
    }

    impl TrafficLight
    {
        pub fn show_state(&self)
        { // Додаємо & перед self, щоб взяти позичку
            println!("the current state is {}", self.color); // Використовуємо self для доступу до color
        }
    }

    fn main()
    {
        let light = TrafficLight
        {
            color: "red".to_owned(),
        };
        // Don't take the ownership of `light` here.
        light.show_state();
        // ... Otherwise, there will be an error below
        println!("{:?}", light);
    }
}

#[test]
fn test3()
{
    struct TrafficLight
    {
        color: String,
    }

    impl TrafficLight
    {
        // Using `Self` to fill in the blank.
        pub fn show_state(&self)
        { // Додаємо &self для отримання позички
            println!("the current state is {}", self.color);
        }

        // Fill in the blank, DON'T use any variants of `Self`.
        pub fn change_state(&mut self)
        { // Додаємо &mut self для зміни стану
            self.color = "green".to_string();
        }
    }

    fn main()
    {
        println!("Success!");
    }
}

#[test]
fn test4()
{
    #[derive(Debug)]
    struct TrafficLight
    {
        color: String,
    }

    impl TrafficLight
    {
        // 1. Implement an associated function `new`,
        // 2. It will return a TrafficLight that contains color "red"
        // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
        pub fn new() -> Self
        { // Використання Self для повернення нового екземпляра
            Self { color: "red".to_string() } // Повертаємо новий TrafficLight з кольором "red"
        }

        pub fn get_state(&self) -> &str
        {
            &self.color
        }
    }

    fn main()
    {
        let light = TrafficLight::new();
        assert_eq!(light.get_state(), "red");

        println!("Success!");
    }
}

#[test]
fn test5()
{
    struct Rectangle
    {
        width: u32,
        height: u32,
    }

    // First impl block for area method
    impl Rectangle
    {
        fn area(&self) -> u32
        {
            self.width * self.height
        }
    }

    // Second impl block for can_hold method
    impl Rectangle
    {
        fn can_hold(&self, other: &Rectangle) -> bool
        {
            self.width > other.width && self.height > other.height
        }
    }

    fn main()
    {
        println!("Success!");
    }
}

#[test]
fn test6()
{
    #[derive(Debug)]
    enum TrafficLightColor
    {
        Red,
        Yellow,
        Green,
    }

    // Implement TrafficLightColor with a method.
    impl TrafficLightColor
    {
        // Method to return the string representation of the color
        pub fn color(&self) -> &str
        {
            match self
            {
                TrafficLightColor::Red => "red",
                TrafficLightColor::Yellow => "yellow",
                TrafficLightColor::Green => "green",
            }
        }
    }

    fn main()
    {
        let c = TrafficLightColor::Yellow;

        assert_eq!(c.color(), "yellow");

        println!("{:?}", c);
    }
}

