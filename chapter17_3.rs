#[test]
fn test1()
{
    struct DoubleRef<'r, 's, T>
    where
        's: 'r
    {
        r: &'r T,
        s: &'s T,
    }

    fn main()
    {
        println!("Success!")
    }

}

fn test2()
{
    struct ImportantExcerpt<'a>
    {
        part: &'a str,
    }

    impl<'a, 'b> ImportantExcerpt<'a>
    {
        fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str
        {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    fn main()
    {
        println!("Success!")
    }
}

fn test3()
{
    fn f<'a, 'b>(x: &'a i32, mut y: &'b i32)
    {
        y = x;
        let r: &'b &'a i32 = &&0;
    }

    fn main()
    {
        println!("Success!")
    }

}

fn test4()
{
    fn call_on_ref_zero<'a, F>(f: F)
    where
        F: Fn(&'a i32)
    {
        let zero = 0;
        f(&zero);
    }

    fn main()
    {
        println!("Success!");
    }

}

fn test5()
{
    fn main() {
        let mut data = 10;
        {
            let ref1 = &mut data;
            let ref2 = &mut *ref1;

            *ref1 += 1;
            *ref2 += 2;
        }

        println!("{}", data);
    }

}

fn test6()
{
    struct Interface<'a>
    {
        manager: &'a mut Manager<'a>
    }

    impl<'a> Interface<'a>
    {
        pub fn noop(self)
        {
            println!("interface consumed");
        }
    }

    struct Manager<'a>
    {
        text: &'a str
    }

    struct List<'a>
    {
        manager: Manager<'a>,
    }

    impl<'a> List<'a>
    {
        pub fn get_interface(&'a mut self) -> Interface<'a>
        {
            Interface
            {
                manager: &mut self.manager
            }
        }
    }

    fn main()
    {
        let mut list = List
        {
            manager: Manager
            {
                text: "hello"
            }
        };

        list.get_interface().noop();

        println!("Interface should be dropped here and the borrow released");

        use_list(&list);
    }

    fn use_list(list: &List)
    {
        println!("{}", list.manager.text);
    }

}
