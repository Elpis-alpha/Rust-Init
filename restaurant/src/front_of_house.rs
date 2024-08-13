pub mod hosting;
pub mod serving;

pub fn open_the_door() {
    serving::serve_order();
    println!("The doors are opening")
}