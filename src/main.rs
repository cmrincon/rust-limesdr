mod LimeSuite;

use crate::LimeSuite::limeSuite;

fn main() {
    println!("{:?}", limeSuite::get_device_list().unwrap());
}

