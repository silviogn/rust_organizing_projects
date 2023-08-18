/***************************
 * my_project binary crate *
 ***************************/

mod some_module;
use A_01_02_packages_and_crates; // library crate for my_project package

fn main() {
    println!("Running the my_project executable.");
    some_module::mod_func();
    A_01_02_packages_and_crates::lib_func();
}
