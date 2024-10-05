#[test]
fn test1()
{
    struct A;          // Concrete type `A`.
    struct S(A);       // Concrete type `S`.
    struct SGen<T>(T); // Generic type `SGen`.

    fn reg_fn(_s: S) {}

    fn gen_spec_t(_s: SGen<A>) {}

    fn gen_spec_i32(_s: SGen<i32>) {}

    fn generic<T>(_s: SGen<T>) {}

    fn main()
    {
        // Using the non-generic functions
        reg_fn(S(A));          // Concrete type.
        gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
        gen_spec_i32(SGen(32)); // Implicitly specified type parameter `i32`.

        // Explicitly specified type parameter `char` to `generic()`.
        generic::<char>(SGen('a'));

        // Implicitly specified type parameter `char` to `generic()`.
        generic(SGen('b'));

        println!("Success!");
    }
}

#[test]
fn test2()
{
    use std::ops::Add;

    fn sum<T: Add<Output = T>>(a: T, b: T) -> T
    {
        a + b
    }

    fn main()
    {
        assert_eq!(5, sum(2i8, 3i8));
        assert_eq!(50, sum(20, 30));
        assert_eq!(2.46, sum(1.23, 1.23));

        println!("Success!");
    }
}

#[test]
fn test3()
{
    struct Point<T>
    {
        x: T,
        y: T,
    }

    fn main()
    {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };

        println!("Success!");
    }
}

#[test]
fn test4()
{
    struct Point<T, U>
    {
        x: T,
        y: U,
    }

    fn main()
    {
        // DON'T modify this code.
        let p = Point { x: 5, y: "hello".to_string() };

        println!("Success!");
    }
}

#[test]
fn test5()
{
    struct Val<T>
    {
        val: T,
    }

    impl<T> Val<T>
    {
        fn value(&self) -> &T
        {
            &self.val
        }
    }

    fn main()
    {
        let x = Val { val: 3.0 };
        let y = Val { val: "hello".to_string() };
        println!("{}, {}", x.value(), y.value());
    }
}

#[test]
fn test6()
{
    struct Point<T, U>
    {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U>
    {
        // Метод mixup, який бере іншу точку і повертає нову точку з елементами від обох
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W>
        {
            Point
            {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn main()
    {
        let p1 = Point { x: 5, y: 10 };
        let p2 = Point { x: "Hello", y: '中' };

        let p3 = p1.mixup(p2);

        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, '中');

        println!("Success!");
    }
}

#[test]
fn test7()
{
    struct Point<T>
    {
        x: T,
        y: T,
    }

    impl Point<f32>
    {
        fn distance_from_origin(&self) -> f32
        {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    fn main()
    {
        let p = Point { x: 5.0, y: 10.0 }; // Замість цілих чисел використовуємо значення f32
        println!("{}", p.distance_from_origin());
    }
}

