#[test]
fn test1()
{
    use std::fmt;

    struct Structure(i32);

    impl fmt::Debug for Structure
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            write!(f, "Structure({})", self.0)
        }
    }

    fn main()
    {
        println!("{} months in a year.", 12);

        println!("Now {:?} will print!", Structure(3));
    }
}

fn test2()
{
    #[derive(Debug)]
    struct Person
    {
        name: String,
        age: u8,
    }

    fn main()
    {
        let person = Person { name: "Sunface".to_string(), age: 18 };

        println!("{:?}", person);
    }
}

fn test3()
{
    use std::fmt;

    #[derive(Debug)]
    struct Structure(i32);

    struct Deep(Structure);

    impl fmt::Debug for Deep
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            write!(f, "{}", (self.0).0)
        }
    }

    fn main()
    {
        println!("Now {:?} will print!", Deep(Structure(7)));
    }
}

fn test4()
{
    use std::fmt;

    struct Point2D
    {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            write!(f, "Display: {} + {}i", self.x, self.y)
        }
    }

    impl fmt::Debug for Point2D
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
        }
    }

    fn main()
    {
        let point = Point2D { x: 3.3, y: 7.2 };
        assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
        assert_eq!(format!("{:?}", point), "Debug: Complex { real: 3.3, imag: 7.2 }");

        println!("Success!");
    }
}

fn test5()
{
    use std::fmt;

    struct List(Vec<i32>);

    impl fmt::Display for List
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            let vec = &self.0;

            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate()
            {
                if count != 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", count, v)?;
            }
            write!(f, "]")
        }
    }

    fn main()
    {
        let v = List(vec![1, 2, 3]);
        assert_eq!(format!("{}", v), "[0: 1, 1: 2, 2: 3]");
        println!("Success!");
    }
}