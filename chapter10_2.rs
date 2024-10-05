#[test]
fn test1()
{
    struct Array<T, const N: usize>
    {
        data: [T; N],
    }

    fn main()
    {
        let arrays =
            [
            Array
            {
                data: [1, 2, 3],
            },
            Array
            {
                data: [4, 5, 6],
            },
            Array
            {
                data: [7, 8, 9],
            }
        ];
        println!("Success!");
    }
}

#[test]
fn test2()
{
    fn print_array<T, const N: usize>(arr: [T; N])
    {
        println!("{:?}", arr);
    }

    fn main()
    {
        let arr = [1, 2, 3];
        print_array(arr);

        let arr = ["hello", "world"];
        print_array(arr);
    }
}

#[test]
fn test3()
{
    #![allow(incomplete_features)]
    #![feature(generic_const_exprs)]

    fn check_size<T>(val: T)
    where
        Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
    {
        //...
    }

    fn main()
    {
        check_size([0u8; 767]);
        check_size([0i32; 191]);
        check_size(["hello你好"; 95]); // Size of &str is 8 bytes, so 95 * 8 = 760 bytes
        check_size([(); 31].map(|_| "hello你好".to_string()));  // Size of String is 24 bytes, so 31 * 24 = 744 bytes
        check_size(['中'; 191]); // Size of char is 4 bytes, so 191 * 4 = 764 bytes

        println!("Success!");
    }

    pub enum Assert<const CHECK: bool> {}

    pub trait IsTrue {}

    impl IsTrue for Assert<true> {}
}

