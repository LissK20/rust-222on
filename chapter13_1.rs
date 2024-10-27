#[test]
fn test1()
{
    fn drink(beverage: &str)
    {
        if beverage == "lemonade"
        {
            println!("Success!");
            return;
        }

        panic!("Expected lemonade, but got {}", beverage);
    }

    fn main()
    {
        drink("water");

        println!("Exercise Failed if printing out this line!");
    }
}

fn test2()
{
    fn main()
    {
        assert_eq!("abc".as_bytes(), [97, 98, 99]);

        let v = vec![1, 2, 3];
        let ele = match v.get(3)
        {
            Some(&value) => value,
            None =>
                {
                println!("Index out of bounds! Defaulting to 0.");
                0
            }
        };

        let rate = production_rate_per_hour(2);

        divide(15, 1);

        println!("Success! The value is: {}", ele);
    }

    fn divide(x: u8, y: u8)
    {
        if y == 0
        {
            println!("Cannot divide by zero! Defaulting to 0.");
            return;
        }
        println!("{}", x / y);
    }

    fn production_rate_per_hour(speed: u8) -> f64
    {
        let cph: u8 = 221;
        match speed
        {
            1..=4 => (speed * cph) as f64,
            5..=8 => (speed * cph) as f64 * 0.9,
            9..=10 => (speed * cph) as f64 * 0.77,
            _ => 0.0,
        }
    }

    pub fn working_items_per_minute(speed: u8) -> u32
    {
        (production_rate_per_hour(speed) / 60.0) as u32
    }
}
