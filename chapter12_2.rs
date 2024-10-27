#[test]
fn test1()
{
    fn main()
    {
        let i1: i32 = false.into();
        let i2: i32 = i32::from(false);
        assert_eq!(i1, i2);
        assert_eq!(i1, 0);

        let i3: i32 = 'a' as u32 as i32;

        let s: String = 'a'.to_string();

        println!("Success!");
    }
}

fn test2()
{
    #[derive(Debug)]
    struct Number
    {
        value: i32,
    }

    impl From<i32> for Number
    {
        fn from(value: i32) -> Self
        {
            Number { value }
        }
    }

    fn main()
    {
        let num = Number::from(30);
        assert_eq!(num.value, 30);

        let num: Number = 30.into();
        assert_eq!(num.value, 30);

        println!("Success!");
    }
}

fn test3()
{
    use std::fs;
    use std::io;
    use std::num;

    enum CliError
    {
        IoError(io::Error),
        ParseError(num::ParseIntError),
    }

    impl From<io::Error> for CliError
    {
        fn from(error: io::Error) -> CliError
        {
            CliError::IoError(error)
        }
    }

    impl From<num::ParseIntError> for CliError
    {
        fn from(error: num::ParseIntError) -> CliError
        {
            CliError::ParseError(error)
        }
    }

    fn open_and_parse_file(file_name: &str) -> Result<i32, CliError>
    {
        let contents = fs::read_to_string(&file_name)?;
        let num: i32 = contents.trim().parse()?;
        Ok(num)
    }

    fn main()
    {
        println!("Success!");
    }
}



fn test4()
{
    use std::convert::TryInto;
    let n: i16 = 256;

    let n: u8 = match n.try_into()
    {
        Ok(n) => n,
        Err(e) =>
            {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };

    assert_eq!(n, 0);
    println!("Success!");
}

fn test5()
{
    use std::convert::TryInto;

    #[derive(Debug, PartialEq)]
    struct EvenNum(i32);

    impl TryFrom<i32> for EvenNum
    {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error>
        {
            if value % 2 == 0
            {
                Ok(EvenNum(value))
            } else
            {
                Err(())
            }
        }
    }

    fn main()
    {
        assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
        assert_eq!(EvenNum::try_from(5), Err(()));

        let result: Result<EvenNum, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNum(8)));
        let result: Result<EvenNum, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));

        println!("Success!");
    }
}
