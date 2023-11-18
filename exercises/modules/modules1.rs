// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

mod sausage_factory {
    // By default, a private function
    // No one outside of this module can see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // Use `pub` to make a public function.
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
