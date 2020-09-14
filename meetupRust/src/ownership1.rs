fn great(name : Box<String>){
    println!("Hello, {}", name);
}

fn great_ref(name : &Box<String>){
    println!("Hello, {}", name);
}

fn main() {
    let alice = Box::<String>::new("Alice".to_string());

    great_ref(&alice); //Borrow
    great(alice.clone()); //Copy
    great(alice); //Move

   // great(alice); // error[E0382]: use of moved value: `alice`

}