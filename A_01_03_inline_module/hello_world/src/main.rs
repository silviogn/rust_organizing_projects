fn main() {
    // full path
    crate::hello::english();
    crate::hello::spanish();
    crate::hello::casual::english();
    // relative path
    hello::spanish();
    hello::casual::english();
}

mod hello {
    pub fn english() {
        println!("hello");
    }

    pub fn spanish() {
        println!("hola");
    }

    // sub module
    pub mod casual {
        pub fn english() {
            println!("hey!!");
        }
    }
}
