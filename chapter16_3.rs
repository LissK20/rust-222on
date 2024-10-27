#[test]
fn test1()
{
    fn main()
    {
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
        assert_eq!(format!("{1}{0}", 1, 2), "21");
        assert_eq!(format!("{1}{0}{1}{0}", 1, 2), "2112");
        println!("Success!");
    }
}

fn test2()
{
    fn main()
    {
        println!("{argument}", argument = "test");

        assert_eq!(format!("{name}{}", 1, name = 2), "21");
        assert_eq!(format!("{a} {c} {b}", a = "a", b = 'b', c = 3), "a 3 b");

        println!("{abc} {}", 2);
        println!("Success!");
    }
}

fn test3()
{
    fn main()
    {
        println!("Hello {:5}!", "x");
        println!("Hello {:1$}!", "x", 5);

        assert_eq!(format!("Hello {1:5}!", 5, "x"), "Hello x    !");
        assert_eq!(format!("Hello {x:width}!", "x", width = 5), "Hello x    !");

        println!("Success!");
    }
}

fn test4()
{
    fn main()
    {
        println!("Hello {:<5}!", "x");
        assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
        assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");

        assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&!");
        println!("Success!");
    }
}

fn test5()
{
    fn main()
    {
        println!("Hello {:5}!", 5);
        println!("Hello {:+}!", 5);
        println!("Hello {:05}!", 5);
        println!("Hello {:05}!", -5);

        assert!(format!("{number:0>width$}", number = 1, width = 6) == "000001");
        println!("Success!");
    }
}

fn test6()
{
    fn main()
    {
        let v = 3.1415926;

        println!("{:.1$}", v, 4);

        assert_eq!(format!("{:.2}", v), "3.14");
        assert_eq!(format!("{:+.2}", v), "+3.14");
        assert_eq!(format!("{:.0}", v), "3");

        println!("Success!");
    }
}

fn test7()
{
    fn main()
    {
        let s = "Hello, world!";

        println!("{0:.5}", s);

        assert_eq!(format!("Hello {1:.3}!", 3, "abcdefg"), "Hello abc!");

        println!("Success!");
    }
}

fn test8()
{
    fn main()
    {
        assert_eq!(format!("{:b}", 27), "11011");
        assert_eq!(format!("{:o}", 27), "33");
        assert_eq!(format!("{:x}", 27), "1b");
        assert_eq!(format!("{:X}", 27), "1B");

        println!("{:x}!", 27);
        println!("{:#010b}", 27);

        println!("Success!");
    }
}

fn test9()
{
    fn get_person() -> String
    {
        String::from("sunface")
    }

    fn get_format() -> (usize, usize)
    {
        (4, 1)
    }

    fn main()
    {
        let person = get_person();
        println!("Hello, {person}!");

        let (width, precision) = get_format();
        let scores = [("sunface", 99.12), ("jack", 60.34)];

        for (name, score) in scores
        {
            println!("{name}: {score:.1}");
        }
    }
}