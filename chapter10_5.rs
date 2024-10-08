#[test]
fn test1()
{
    struct Container(i32, i32);

    // USING associated types to re-implement trait Contains.
    trait Contains
    {
        type A;
        type B;

        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container
    {
        type A = i32;
        type B = i32;

        fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool
        {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        // Grab the first number.
        fn first(&self) -> i32 { self.0 }

        // Grab the last number.
        fn last(&self) -> i32 { self.1 }
    }

    fn difference<C: Contains>(container: &C) -> i32
    {
        container.last() - container.first()
    }

    fn main()
    {
        let number_1 = 3;
        let number_2 = 10;

        let container = Container(number_1, number_2);

        println!("Does container contain {} and {}: {}",
                 &number_1, &number_2,
                 container.contains(&number_1, &number_2));
        println!("First number: {}", container.first());
        println!("Last number: {}", container.last());

        println!("The difference is: {}", difference(&container));
    }
}

#[test]
fn test2()
{
    use std::ops::Sub;

    #[derive(Debug, PartialEq)]
    struct Point<T>
    {
        x: T,
        y: T,
    }

    // Implementation using default generic parameters (for i32)
    impl Sub for Point<i32>
    {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output
        {
            Point
            {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    // Implementation using default generic parameters (for f64)
    impl Sub for Point<f64>
    {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output
        {
            Point
            {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    // Implementation without default generic parameters
    impl<T: Sub<Output = T>> Sub for Point<T>
    {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output
        {
            Point
            {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    fn main()
    {
        assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
                   Point { x: 1, y: 3 });

        // Test with f64
        let point1 = Point { x: 5.0, y: 10.0 };
        let point2 = Point { x: 2.0, y: 3.0 };
        assert_eq!(point1 - point2, Point { x: 3.0, y: 7.0 });

        println!("Success!");
    }
}

#[test]
fn test3()
{
    trait Pilot
    {
        fn fly(&self) -> String;
    }

    trait Wizard
    {
        fn fly(&self) -> String;
    }

    struct Human;

    impl Pilot for Human
    {
        fn fly(&self) -> String
        {
            String::from("This is your captain speaking.")
        }
    }

    impl Wizard for Human
    {
        fn fly(&self) -> String
        {
            String::from("Up!")
        }
    }

    impl Human
    {
        fn fly(&self) -> String
        {
            String::from("*waving arms furiously*")
        }
    }

    fn main()
    {
        let person = Human;

        // Викликаємо метод fly з трейту Pilot
        assert_eq!(Pilot::fly(&person), "This is your captain speaking.");

        // Викликаємо метод fly з трейту Wizard
        assert_eq!(Wizard::fly(&person), "Up!");

        // Викликаємо метод fly з імплементації структури Human
        assert_eq!(person.fly(), "*waving arms furiously*");

        println!("Success!");
    }
}

#[test]
fn test4()
{
    trait Person
    {
        fn name(&self) -> String;
    }

    // Person is a supertrait of Student.
    // Implementing Student requires you to also impl Person.
    trait Student: Person
    {
        fn university(&self) -> String;
    }

    trait Programmer
    {
        fn fav_language(&self) -> String;
    }

    // CompSciStudent (computer science student) is a subtrait of both Programmer
    // and Student. Implementing CompSciStudent requires you to impl both supertraits.
    trait CompSciStudent: Programmer + Student
    {
        fn git_username(&self) -> String;
    }

    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String
    {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }

    struct CSStudent
    {
        name: String,
        university: String,
        fav_language: String,
        git_username: String,
    }

    // IMPLEMENT the necessary traits for CSStudent to make the code work
    impl Person for CSStudent
    {
        fn name(&self) -> String
        {
            self.name.clone()
        }
    }

    impl Student for CSStudent
    {
        fn university(&self) -> String
        {
            self.university.clone()
        }
    }

    impl Programmer for CSStudent
    {
        fn fav_language(&self) -> String
        {
            self.fav_language.clone()
        }
    }

    impl CompSciStudent for CSStudent
    {
        fn git_username(&self) -> String
        {
            self.git_username.clone()
        }
    }

    fn main()
    {
        let student = CSStudent
        {
            name: "Sunfei".to_string(),
            university: "XXX".to_string(),
            fav_language: "Rust".to_string(),
            git_username: "sunface".to_string(),
        };

        // FILL in the blank
        println!("{}", comp_sci_student_greeting(&student));
    }
}

#[test]
fn test5()
{
    use std::fmt;

    // DEFINE a newtype `Pretty` here
    struct Pretty(String); // Новий тип `Pretty`, що містить рядок.

    impl fmt::Display for Pretty
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            write!(f, "\"{}\"", self.0.clone() + ", world")
        }
    }

    fn main()
    {
        let w = Pretty("hello".to_string());
        println!("w = {}", w);
    }
}