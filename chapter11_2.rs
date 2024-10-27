#[test]
fn test1()
{
    fn main()
    {
        let arr: [u8; 3] = [1, 2, 3];
        let v = Vec::from(arr);
        is_vec(&v);

        let v = vec![1, 2, 3];
        is_vec(&v);

        let v = vec!(1, 2, 3);
        is_vec(&v);

        let mut v1 = Vec::new();
        for &item in &arr
        {
            v1.push(item);
        }
        is_vec(&v1);

        assert_eq!(v, v1);
        println!("Success!");
    }
    fn is_vec(v: &Vec<u8>) {}
}

fn test2()
{
    fn main()
    {
        let mut v1 = Vec::from([1, 2, 4]);
        v1.pop();
        v1.push(3);

        let v2 = Vec::from([1, 2, 3]);

        assert_eq!(v1, v2);
        println!("Success!");
    }
}

fn test3()
{
    fn main()
    {
        let arr = [1, 2, 3];
        let v1 = Vec::from(arr);
        let v2: Vec<i32> = arr.to_vec();

        assert_eq!(v1, v2);

        let s = "hello".to_string();
        let v1: Vec<u8> = s.into_bytes();

        let s = "hello".to_string();
        let v2 = s.into_bytes();
        assert_eq!(v1, v2);

        let s = "hello";
        let v3 = Vec::from(s);
        assert_eq!(v2, v3);

        let v4: Vec<i32> = [0; 10].into_iter().collect();
        assert_eq!(v4, vec![0; 10]);

        println!("Успіх!");
    }
}

fn test4()
{
    fn main()
    {
        let mut v = Vec::from([1, 2, 3]);

        for i in 0..3
        {
            println!("{:?}", v[i]);
        }

        for i in 0..5
        {
            v.push(i + 2);
        }

        assert_eq!(v, vec![1, 2, 3, 2, 3, 4, 5, 6]);
        println!("Успіх!");
    }
}

fn test5()
{
    fn main()
    {
        let mut v = vec![1, 2, 3];
        let slice1 = &v[..];

        let slice2 = &v[0..3];

        assert_eq!(slice1, slice2);

        let vec_ref: &mut Vec<i32> = &mut v;
        vec_ref.push(4);

        let slice3 = &v[0..4];

        assert_eq!(slice3, &[1, 2, 3, 4]);
        println!("Успіх!");
    }
}

fn test6()
{
    fn main()
    {
        let mut vec = Vec::with_capacity(10);

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 10);

        for i in 0..10
        {
            vec.push(i);
        }
        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 10);

        vec.push(11);
        assert_eq!(vec.len(), 11);
        assert!(vec.capacity() >= 11);

        let mut vec = Vec::with_capacity(100);
        for i in 0..100
        {
            vec.push(i);
        }

        assert_eq!(vec.len(), 100);
        assert_eq!(vec.capacity(), 100);
        println!("Успіх!");
    }
}

fn test7()
{
    #[derive(Debug, PartialEq)]
    enum IpAddr
    {
        V4(String),
        V6(String),
    }

    fn main()
    {
        let v: Vec<IpAddr> = vec![
            IpAddr::V4("127.0.0.1".to_string()),
            IpAddr::V6("::1".to_string()),
        ];

        assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
        assert_eq!(v[1], IpAddr::V6("::1".to_string()));

        println!("Успіх!");
    }
}

fn test8()
{
    trait IpAddr
    {
        fn display(&self);
    }

    struct V4(String);
    impl IpAddr for V4
    {
        fn display(&self)
        {
            println!("ipv4: {:?}", self.0);
        }
    }

    struct V6(String);
    impl IpAddr for V6
    {
        fn display(&self)
        {
            println!("ipv6: {:?}", self.0);
        }
    }

    fn main()
    {
        let v: Vec<Box<dyn IpAddr>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];

        for ip in v
        {
            ip.display();
        }
    }
}