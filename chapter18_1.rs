#[test]
fn test1()
{
    fn main()
    {
        let color = String::from("green");

        let print = move || println!("`color`: {}", color);

        print();
        print();

        println!("{}", color);
    }
}

fn test2()
{
    fn main()
    {
        let mut count = 0;

        let mut inc = ||
            {
            count += 1;
            println!("`count`: {}", count);
        };

        inc();
        inc();
        count = 0;

        assert_eq!(count, 0);
    }
}

fn test3()
{
    fn main()
    {
        let movable = Box::new(3);

        let consume = ||
            {
                println!("`movable`: {:?}", movable);
                take(movable);
            };

        consume();
        consume();
    }

    fn take<T>(_v: T) {}

}

fn test4()
{
    fn main()
    {
        let example_closure = |x| x;

        let s = example_closure(String::from("hello"));

        let n = example_closure(5 as i32);
    }

}

fn test5_1()
{
    fn fn_once<F>(func: F)
    where
        F: FnMut(usize) -> bool,
    {
        println!("{}", func(3));
        println!("{}", func(4));
    }

    fn main()
    {
        let mut x = vec![1, 2, 3];
        fn_once(|z| { z == x.len() });
    }
}

fn test5_2()
{
    fn fn_once<F>(func: F)
    where
        F: Fn(usize) -> bool,
    {
        println!("{}", func(3));
        println!("{}", func(4));
    }

    fn main()
    {
        let x = vec![1, 2, 3];
        fn_once(move |z| { z == x.len() });
    }

}

fn test6()
{
    fn main()
    {
        let mut s = String::new();

        let update_string = |str| s.push_str(str);

        exec(update_string);

        println!("{:?}", s);
    }

    fn exec<F>(mut f: F)
    where
        F: Fn(&str),
    {
        f("hello")
    }

}

fn test7()
{
    fn apply<F>(f: F)
    where
        F: Fn()
    {
        f();
    }

    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32
    {
        f(3)
    }

    fn main()
    {
        use std::mem;

        let greeting = "hello";
        let mut farewell = "goodbye".to_owned();

        let diary = ||
            {
            println!("I said {}.", greeting);
            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzz");
            mem::drop(farewell);
        };

        apply(diary);

        let double = |x| 2 * x;
        println!("3 doubled: {}", apply_to_3(double));
    }
}

fn test8()
{
    fn main()
    {
        let mut s = String::new();

        let update_string = |str| -> String
            {
                s.push_str(str);
                s
            };

        exec(update_string);
    }

    fn exec<F>(mut f: F)
    where
        F: Fn(&str) -> String
    {
        f("hello");
    }
}

fn test9()
{
    fn call_me<F>(f: F)
    where
        F: Fn()
    {
        f();
    }

    fn function()
    {
        println!("I'm a function!");
    }

    fn main()
    {
        let closure = || println!("I'm a closure!");

        call_me(closure);
        call_me(function);
    }

}

fn test10_1()
{
    fn create_fn() -> Box<dyn Fn(i32) -> i32>
    {
        let num = 5;
        Box::new(move |x| x + num)
    }

    fn main()
    {
        let fn_plain = create_fn();
        let result = fn_plain(1);
        println!("Result: {}", result);
    }

}

fn test10_2()
{
    fn create_fn() -> Box<dyn FnOnce(i32) -> i32>
    {
        let num = 5;
        Box::new(move |x| x + num)
    }

    fn main()
    {
        let fn_plain = create_fn();
        let result = fn_plain(1);
        println!("Result: {}", result);
    }

}
fn test11()
{
    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32>
    {
        let num = 5;

        if x > 1
        {
            Box::new(move |x| x + num)
        }
        else
        {
            Box::new(move |x| x + num)
        }
    }

    fn main()
    {
        let func = factory(2);
        println!("Result: {}", func(10)); // Output: Result: 15
    }
}