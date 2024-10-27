#[test]
fn test1()
{
    fn main()
    {
        let mut s: String = "hello, ".to_owned();
        s.push_str("world");
        s.push('!');

        move_ownership(s);

        assert_eq!(s, "hello, world!");

        println!("Success!");
    }

    fn move_ownership(s: String)
    {
        println!("ownership of \"{}\" is moved here!", s)
    }
}

fn test2()
{
    fn main()
    {
        let mut s = String::from("hello, world");

        let slice1: &str = &s;
        assert_eq!(slice1, "hello, world");

        let slice2 = &s[..5];
        assert_eq!(slice2, "hello");

        let slice3: &mut String = &mut s;
        slice3.push('!');
        assert_eq!(slice3, "hello, world!");

        println!("Success!");
    }
}

fn test3()
{
    fn main()
    {
        let s: String = String::from("hello, world!");
        let slice: &str = &s;
        let s: String = slice.to_string();

        assert_eq!(s, "hello, world!");
        println!("Success!");
    }
}

fn test4()
{
    fn main()
    {
        let s = String::from("hello, 世界");

        let slice1 = s.chars().next().unwrap();
        assert_eq!(slice1, 'h');

        let slice2 = &s[7..10];
        assert_eq!(slice2, "世");

        for (i, c) in s.char_indices()
        {
            if i == 7
            {
                assert_eq!(c, '世')
            }
        }
        println!("Success!");
    }
}

fn test5()
{
    fn main()
    {
        let mut s = String::new();
        s.push_str("hello");

        let v = vec![104, 101, 108, 108, 111];
        let s1 = String::from_utf8(v).expect("Invalid UTF-8 sequence");

        assert_eq!(s, s1);
        println!("Success!");
    }
}

fn test6()
{
    fn main()
    {
        let mut s = String::with_capacity(25);

        println!("{}", s.capacity());

        for _ in 0..2
        {
            s.push_str("hello");
            println!("{}", s.capacity());
        }
        println!("Success!");
    }
}

fn test7()
{
    use std::mem;
    fn main()
    {
        let story = String::from("Rust By Practice");
        let mut story = mem::ManuallyDrop::new(story);

        let ptr = story.as_ptr();
        let len = story.len();
        let capacity = story.capacity();

        assert_eq!(16, len);

        let s = unsafe { String::from_raw_parts(ptr as *mut _, len, capacity) };

        assert_eq!(*story, s);
        println!("Success!");
    }
}
