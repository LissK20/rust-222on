

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
