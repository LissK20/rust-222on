

use hello_package::front_of_house::hosting::seat_at_table;
use hello_package::front_of_house::serving::take_order;

fn main()
{
    assert_eq!(seat_at_table(), "sit down please");
    assert_eq!(take_order(), "yummy yummy!");

    println!("Success!");
}
