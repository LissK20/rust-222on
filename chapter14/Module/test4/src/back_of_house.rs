
use super::front_of_house;

pub fn fix_incorrect_order()
{
    cook_order();

    super::front_of_house::serving::serve_order();

    front_of_house::serving::serve_order();

}

fn cook_order()
{
    println!("Приготування замовлення.");
}
