fn great(name : String){
    println!("Hello, {}", name);
}

fn great_ref(name : &String){
    println!("Hello, {}", name);
}

fn main() {
    let alice = "Alice".to_string();

    great_ref(&alice); //Borrow
    great(alice.clone()); //Copy
    great(alice); //Move

    // great(alice); // error[E0382]: use of moved value: `alice`

}