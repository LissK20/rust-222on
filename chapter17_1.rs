#[test]
fn test1()
{
    fn main()
    {
        let i = 3; // `i` має найдовше життя ────────────────────────────────────────────────────────────────
        {
            let borrow1 = &i; // `borrow1` починає своє життя ──┐
            //                                                      │
            println!("borrow1: {}", borrow1); //                │
        } // `borrow1` закінчує своє життя ──────────────────────┘
        {
            let borrow2 = &i; // `borrow2` починає своє життя ──┐
            │
            println!("borrow2: {}", borrow2);                //
        } // `borrow2` закінчує своє життя ────────────────────┘
    }

}

fn test2()
{
    fn main()
    {
        {
            let r;                // ---------+-- 'a
                                        //          |
            {                           //          |
                let x = 5;         // -+-- 'b  |
                r = &x;                 //  |       |
            }                           // -+       |
                                        //          |
            println!("r: {}", r);       //          |
        }                               // ---------+
    }
}

fn test3()
{
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
    {
        if x.len() > y.len()
        {
            x
        }
        else
        {
            y
        }
    }

    fn main()
    {
        let string1 = String::from("long string");
        let string2 = String::from("short");

        let result = longest(&string1, &string2);
        println!("The longest string is: {}", result);
    }
}

fn test4_1()
{
    fn valid_output() -> String
    {
        String::from("foo")
    }

    fn main() {
        let s = valid_output();
        println!("{}", s);
    }
}

fn test4_2()
{
    fn static_output() -> &'static str
    {
        "foo"
    }

    fn main()
    {
        let s = static_output();
        println!("{}", s);
    }
}

fn test4_3()
{
    fn valid_output() -> Vec<String>
    {
        let vec = vec![String::from("foo")];
        vec
    }

    fn main()
    {
        let v = valid_output();
        println!("{}", v[0]);
    }
}

fn test5()
{
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32)
    {
        println!("x is {} and y is {}", x, y);
    }

    fn successful_borrow() -> i32
    {
        let x = 12;
        x
    }

    fn main()
    {
        let (four, nine) = (4, 9);

        print_refs(&four, &nine);

        let result = successful_borrow();
        println!("Successful borrow: {}", result);
    }
}

fn test6()
{
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    #[derive(Debug)]
    struct NamedBorrowed<'a>
    {
        x: &'a i32,
        y: &'a i32,
    }

    #[derive(Debug)]
    enum Either<'a>
    {
        Num(i32),
        Ref(&'a i32),
    }

    fn main()
    {
        let x = 18;
        let y = 15;

        let single = Borrowed(&x);
        let double = NamedBorrowed { x: &x, y: &y };
        let reference = Either::Ref(&x);
        let number = Either::Num(y);

        println!("x is borrowed in {:?}", single);
        println!("x and y are borrowed in {:?}", double);
        println!("x is borrowed in {:?}", reference);
        println!("y is *not* borrowed in {:?}", number);
    }
}

fn test7()
{
    #[derive(Debug)]
    struct NoCopyType {}

    #[derive(Debug)]
    struct Example<'a, 'b>
    {
        a: &'a u32,
        b: &'b NoCopyType,
    }

    fn main()
    {
        let var_a = 35;
        let example;

        {
            let var_b = NoCopyType {};
            example = Example { a: &var_a, b: &var_b };
        }

        println!("(Success!) {:?}", example);
    }
}

fn test8()
{
    #[derive(Debug)]
    struct NoCopyType {}

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Example<'a, 'b>
    {
        a: &'a u32,
        b: &'b NoCopyType,
    }

    fn fix_me<'b>(foo: &'b Example<'_, 'b>) -> &'b NoCopyType
    {
        foo.b
    }

    fn main()
    {
        let no_copy = NoCopyType {};
        let example = Example { a: &1, b: &no_copy };
        fix_me(&example);
        println!("Success!");
    }
}

fn test9()
{
    struct ImportantExcerpt<'a>
    {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a>
    {
        fn level(&self) -> i32
        {
            3
        }
    }

    fn main() {}

}

fn test10()
{
    fn input(x: &i32)
    {
        println!("`annotated_input`: {}", x);
    }

    fn pass(x: &i32) -> &i32
    {
        x
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
    {
        if x.len() > y.len()
        {
            x
        }
        else
        {
            y
        }
    }


    struct Owner(i32);

    impl Owner
    {
        fn add_one(&mut self)
        {
            self.0 += 1;
        }

        fn print(&self)
        {
            println!("`print`: {}", self.0);
        }
    }

    struct Person<'a>
    {
        age: u8,
        name: &'a str,
    }

    enum Either<'a>
    {
        Num(i32),
        Ref(&'a i32),
    }

    fn main()
    {
        let name = "Alice";
        let person = Person { age: 30, name };
        println!("Person: age = {}, name = {}", person.age, person.name);
    }

}