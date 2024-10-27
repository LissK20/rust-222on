mod front_of_house;
mod back_of_house;

pub use front_of_house::hosting;

pub fn eat_at_restaurant() -> &'static str
{
    hosting::add_to_waitlist();
    "yummy yummy!"
}
