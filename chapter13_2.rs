#[test]
fn test1()
{
    use std::num::ParseIntError;
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError>
    {
        let n1 = n1_str.parse::<i32>()?;
        let n2 = n2_str.parse::<i32>()?;
        Ok(n1 * n2)
    }
    fn main()
    {
        let result = multiply("10", "2");
        assert_eq!(result, Ok(20));

        let result = multiply("t", "2");
        assert_eq!(result.is_err(), true);

        println!("Success!");
    }
}

fn test2()
{
    use std::num::ParseIntError;
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError>
    {
        let n1 = n1_str.parse::<i32>()?;
        let n2 = n2_str.parse::<i32>()?;
        Ok(n1 * n2)
    }

    fn main()
    {
        assert_eq!(multiply("3", "4").unwrap(), 12);
        println!("Success!");
    }
}

fn test3()
{
    use std::fs::File;
    use std::io::{self, Read};

    fn read_file1() -> Result<String, io::Error>
    {
        let f = File::open("hello.txt");
        let mut f = match f
        {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s)
        {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_file2() -> Result<String, io::Error>
    {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }

    fn main()
    {
        assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
        println!("Success!");
    }
}

fn test4()
{
    use std::num::ParseIntError;

    fn add_two(n_str: &str) -> Result<i32, ParseIntError>
    {
        n_str.parse::<i32>().map(|n| n + 2)
    }

    fn add_two_and_then(n_str: &str) -> Result<i32, ParseIntError>
    {
        n_str.parse::<i32>().and_then(|n| Ok(n + 2))
    }

    fn main()
    {
        assert_eq!(add_two("4").unwrap(), 6);
        assert_eq!(add_two_and_then("4").unwrap(), 6);

        println!("Success!");
    }
}

fn test5()
{
    use std::num::ParseIntError;

    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError>
    {
        match n1_str.parse::<i32>()
        {
            Ok(n1) =>
                {
                match n2_str.parse::<i32>()
                {
                    Ok(n2) => Ok(n1 * n2),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError>
    {
        n1_str.parse::<i32>().and_then(|n1|
            {
            n2_str.parse::<i32>().map(|n2| n1 * n2)
        })
    }

    fn print(result: Result<i32, ParseIntError>)
    {
        match result
        {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn main()
    {
        let twenty = multiply1("10", "2");
        print(twenty);

        let tt = multiply("t", "2");
        print(tt);

        println!("Success!");
    }
}

fn test6()
{
    use std::num::ParseIntError;

    type Res<T> = Result<T, ParseIntError>;

    fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32>
    {
        first_number_str.parse::<i32>().and_then(|first_number|
            {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
    }

    fn print(result: Res<i32>)
    {
        match result
        {
            Ok(n)  => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn main()
    {
        print(multiply("10", "2"));
        print(multiply("t", "2"));

        println!("Success!");
    }
}