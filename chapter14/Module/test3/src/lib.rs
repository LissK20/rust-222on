
mod front_of_house
{
    pub mod hosting
    {
        pub fn add_to_waitlist()
        {
            println!("Додано до списку очікування.");
        }

        pub fn seat_at_table()
        {
            println!("Саджаємо за стіл.");
        }
    }

    pub mod serving
    {
        pub fn take_order()
        {
            println!("Прийнято замовлення.");
        }

        pub fn serve_order()
        {
            println!("Замовлення подано.");
        }

        pub fn take_payment()
        {
            println!("Оплату прийнято.");
        }

        pub fn complain()
        {
            println!("Обробляємо скаргу.");
        }
    }
}

mod back_of_house
{
    fn fix_incorrect_order()
    {
        cook_order();

        super::front_of_house::serving::serve_order();

        front_of_house::serving::serve_order();

    }

    fn cook_order() {}
}

pub fn eat_at_restaurant()
{
    back_of_house::fix_incorrect_order();
}
