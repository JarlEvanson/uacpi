#![allow(missing_docs)]

use uacpi::status::Status;

pub fn main() {
    println!("{:?}", Status::AmlBadEncoding);
    println!("{}", Status::AmlBadEncoding);
}
