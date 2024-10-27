#[test]
fn test1()
{
    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    fn main()
    {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}

fn test2()
{
    struct Meters(u32);

    fn main()
    {
        let i: u32 = 2;
        assert_eq!(i.pow(2), 4);
        let n = Meters(i);
        assert_eq!((n.0).pow(2), 4);
    }
}

fn test3()
{
    struct Years(i64);

    struct Days(i64);

    impl Years
    {
        pub fn to_days(&self) -> Days
        {
            Days(self.0 * 365)
        }
    }

    impl Days
    {
        pub fn to_years(&self) -> Years
        {
            Years(self.0 / 365)
        }
    }

    fn old_enough(age: &Years) -> bool
    {
        age.0 >= 18
    }

    fn main()
    {
        let age = Years(5);
        let age_days = age.to_days();
        println!("Old enough {}", old_enough(&age));
        println!("Old enough {}", old_enough(&age_days.to_years()));
    }
}

fn test4()
{
    use std::ops::Add;
    use std::fmt::{self, Display};

    struct Meters(u32);

    impl Display for Meters
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            write!(f, "There are still {} meters left", self.0)
        }
    }

    impl Add for Meters
    {
        type Output = Self;

        fn add(self, other: Meters) -> Self
        {
            Self(self.0 + other.0)
        }
    }

    fn calculate_distance(a: Meters, b: Meters) -> Meters
    {
        a + b
    }

    fn main()
    {
        let d = calculate_distance(Meters(10), Meters(20));
        assert_eq!(format!("{}", d), "There are still 30 meters left");
    }
}

fn test5()
{
    enum VeryVerboseEnumOfThingsToDoWithNumbers
    {
        Add,
        Subtract,
    }

    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    fn main()
    {
        let x = Operations::Add;
    }
}

fn test6()
{
    use VeryVerboseEnumOfThingsToDoWithNumbers as Operations;

    enum VeryVerboseEnumOfThingsToDoWithNumbers
    {
        Add,
        Subtract,
    }

    impl VeryVerboseEnumOfThingsToDoWithNumbers
    {
        fn run(&self, x: i32, y: i32) -> i32
        {
            match self
            {
                Operations::Add => x + y,
                Operations::Subtract => x - y,
            }
        }
    }
}

fn test7()
{
    fn my_function<const N: usize>() -> [u32; N]
    {
        [123; N]
    }

    fn main()
    {
        let arr = my_function::<5>();
        println!("{:?}", arr);
    }
}

fn test8()
{
    fn main()
    {
        let s: &str = "Hello there!";
        let arr: &[u8] = &[1, 2, 3];

        println!("String: {}", s);
        println!("Array: {:?}", arr);
    }
}

fn test9()
{
    use std::fmt::Display;

    fn foobar<T: Display>(thing: T)
    {
        println!("{}", thing);
    }

    fn foobar_display(thing: &dyn Display)
    {
        println!("{}", thing);
    }

    fn main()
    {
        let s = "Hello";
        foobar(s);
        foobar_display(&s);
    }
}