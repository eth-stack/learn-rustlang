use crate::garden::vegertables::Asparagus;
pub mod garden;

fn main() {
    let plant = Asparagus {};

    println!("I'm growing {:?}", plant);
}
