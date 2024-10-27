#[test]
fn test1()
{
    fn main()
    {
        let arr = [0; 10];
        for &i in arr.iter()
        {
            println!("{}", i);
        }
    }

}

fn test2()
{
    fn main()
    {
        let mut v = Vec::new();
        for n in 0..100
        {
            v.push(n);
        }
        assert_eq!(v.len(), 100);
    }
}

fn test3_1()
{
    fn main()
    {
        let mut v1 = vec![1, 2].into_iter();

        assert_eq!(v1.next(), Some(1));
        assert_eq!(v1.next(), Some(2));
        assert_eq!(v1.next(), None);
    }
}

fn test3_2()
{
    fn main()
    {
        let v1 = vec![1, 2];

        let mut iter = v1.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }
}

fn test4()
{
    fn main()
    {
        let arr = vec![0; 10];
        for i in &arr
        {
            println!("{}", i);
        }

        println!("{:?}", arr);
    }
}

fn test5()
{
    fn main()
    {
        let mut names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter_mut()
        {
            *name = match *name
            {
                "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }

        println!("names: {:?}", names);
    }
}

fn test6()
{
    fn main()
    {
        let mut values = vec![1, 2, 3];
        let mut values_iter = values.iter_mut();

        if let Some(v) = values_iter.next()
        {
            *v = 0;
        }

        assert_eq!(values, vec![0, 2, 3]);
    }
}

fn test7()
{
    struct Fibonacci
    {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci
    {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item>
        {
            let curr = self.curr;
            self.curr = self.next;
            self.next += curr;
            Some(curr)
        }
    }

    fn fibonacci() -> Fibonacci
    {
        Fibonacci { curr: 0, next: 1 }
    }

    fn main()
    {
        let mut fib = fibonacci();
        assert_eq!(fib.next(), Some(0));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
    }
}

fn test8()
{
    fn main()
    {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        let total: i32 = v1_iter.clone().sum();

        assert_eq!(total, 6);

        println!("{:?}", v1);
    }
}

fn test9()
{
    use std::collections::HashMap;

    fn main()
    {
        let names = [("sunface", 18), ("sunfei", 18)];
        let folks: HashMap<_, _> = names.into_iter().collect();

        println!("{:?}", folks);

        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().cloned().collect();

        assert_eq!(v2, vec![1, 2, 3]);
    }
}

fn test10()
{
    fn main()
    {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }
}

fn test11()
{
    use std::collections::HashMap;

    fn main()
    {
        let names = ["sunface", "sunfei"];
        let ages = [18, 18];
        let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

        println!("{:?}", folks);
    }
}

fn test12()
{
    #[derive(PartialEq, Debug)]
    struct Shoe
    {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>
    {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    fn main()
    {
        let shoes = vec![
            Shoe
            {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe
            {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe
            {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe
                {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe
                {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}