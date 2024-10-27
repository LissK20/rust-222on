#[test]
fn test1_1()
{
    fn main()
    {
        let v = "hello";
        need_static(v);

        println!("Success!")
    }

    fn need_static(r: &'static str)
    {
        assert_eq!(r, "hello");
    }

}

fn test1_2()
{
    fn main()
    {
        static V: &str = "hello";
        need_static(V);

        println!("Success!")
    }

    fn need_static(r: &'static str)
    {
        assert_eq!(r, "hello");
    }
}

fn test2()
{
    #[derive(Debug)]
    struct Config
    {
        a: String,
        b: String,
    }

    static mut config: Option<&mut Config> = None;

    fn init() -> Option<&'static mut Config>
    {
        unsafe
            {
                let config_instance = Config
                {
                    a: "A".to_string(),
                    b: "B".to_string(),
                };
                Some(Box::leak(Box::new(config_instance)))
            }
    }

    fn main()
    {
        unsafe
            {
                config = init();
                println!("{:?}", config);
            }
    }

}

fn test3()
{
    fn main()
    {
        {
            let static_string: &'static str = "I'm in read-only memory";
            println!("static_string: {}", static_string);
        }

        println!("static_string reference remains alive: {}", "I'm in read-only memory");
    }
}

fn test4()
{
    static NUM: i32 = 18;

    fn coerce_static<'a>(_: &'a i32) -> &'a i32
    {
        &NUM
    }

    fn main()
    {
        {
            let lifetime_num = 9;
            let coerced_static = coerce_static(&lifetime_num);
            println!("coerced_static: {}", coerced_static);
        }

        println!("NUM: {} stays accessible!", NUM);
    }
}

fn test5()
{
    use std::fmt::Debug;

    fn print_it<T: Debug + 'static>(input: T)
    {
        println!("'static value passed in is: {:?}", input);
    }

    fn print_it1(input: impl Debug + 'static)
    {
        println!("'static value passed in is: {:?}", input);
    }

    fn print_it2<T: Debug + 'static>(input: &T)
    {
        println!("'static value passed in is: {:?}", input);
    }

    fn main()
    {
        let i = 5;
        print_it(i);

        print_it1(5);

        print_it2(&i);
    }
}

fn test6()
{
    use std::fmt::Display;

    fn main()
    {
        let mut string = "First".to_owned();

        string.push_str(string.to_uppercase().as_str());
        print_a(&string);
        print_b(&string);
        print_c(&string as &dyn Display); // Зміна
        print_d(&string as &dyn Display); // Зміна
        print_e(&string);
        print_f(&string);
        print_g(&string); // Відключення для демонстрації
    }

    fn print_a<T: Display + 'static>(t: &T)
    {
        println!("{}", t);
    }

    fn print_b<T>(t: &T)
    where
        T: Display + 'static,
    {
        println!("{}", t);
    }

    fn print_c(t: &'static dyn Display)
    {
        println!("{}", t)
    }

    fn print_d(t: &'static dyn Display)
    {
        println!("{}", t)
    }

    fn print_e(t: &(dyn Display + 'static))
    {
        println!("{}", t)
    }

    fn print_f(t: &(impl Display + 'static))
    {
        println!("{}", t)
    }

    fn print_g(t: &'static String)
    {
        println!("{}", t);
    }

}
