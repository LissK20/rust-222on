#[test]
fn test1()
{
    use std::fmt;
    struct Point
    {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            write!(f, "The point is ({}, {})", self.x, self.y)
        }
    }

    fn main()
    {
        let origin = Point { x: 0, y: 0 };
        assert_eq!(origin.to_string(), "The point is (0, 0)");
        assert_eq!(format!("{}", origin), "The point is (0, 0)");

        println!("Success!");
    }
}

fn test2()
{
    use std::str::FromStr;

    fn main()
    {
        let parsed: i32 = i32::from_str("5").unwrap();
        let turbo_parsed = i32::from_str("10").unwrap();
        let from_str = i32::from_str("20").unwrap();

        let sum = parsed + turbo_parsed + from_str;
        assert_eq!(sum, 35);

        println!("Success!");
    }
}

fn test3()
{
    use std::str::FromStr;
    use std::num::ParseIntError;

    #[derive(Debug, PartialEq)]
    struct Point
    {
        x: i32,
        y: i32
    }

    impl FromStr for Point
    {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err>
        {
            let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                .split(',')
                .map(|x| x.trim())
                .collect();

            let x_fromstr = coords[0].parse::<i32>()?;
            let y_fromstr = coords[1].parse::<i32>()?;

            Ok(Point { x: x_fromstr, y: y_fromstr })
        }
    }

    fn main()
    {
        let p = Point::from_str("(3, 4)");

        assert_eq!(p.unwrap(), Point{ x: 3, y: 4} );
        println!("Success!");
    }
}

fn test4()
{
    fn foo() -> i32
    {
        0
    }

    fn main()
    {
        let pointer = foo as *const ();

        let function: fn() -> i32 = unsafe
            {
            std::mem::transmute(pointer)
        };

        assert_eq!(function(), 0);
        println!("Success!");
    }
}

fn test5()
{
    use std::sync::Arc;
    struct R<'a>(&'a i32);

    fn safe_extend_lifetime(value: &i32) -> Arc<R>
    {
        Arc::new(R(value))
    }

    fn main()
    {
        let value = 10;
        let r = safe_extend_lifetime(&value);
    }
}

fn test6()
{
    fn main()
    {
        let raw_bytes = [0x78, 0x56, 0x34, 0x12];

        let num = u32::from_ne_bytes(raw_bytes);
        assert_eq!(num, 0x12345678);

        let num = u32::from_be_bytes(raw_bytes);
        assert_eq!(num, 0x78563412);

        let ptr = &0;
        let ptr_num_cast = ptr as *const i32 as usize;

        let mut value: i32 = 0;
        let ptr = &mut value;
        let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32) };

        let slice = "Rust".as_bytes();
        assert_eq!(slice, &[82, 117, 115, 116]);

        assert_eq!(b"Rust", &[82, 117, 115, 116]);
        println!("Success!");
    }
}