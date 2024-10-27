#[test]
fn test1()
{
    fn main()
    {
        let s1 = "hello";
        let s = format!("{}, world!", s1);
        assert_eq!(s, "hello, world!");
    }

}

fn test2()
{
    fn main()
    {
        println!("Hello world, I am {}!", "Sunface");
    }
}
