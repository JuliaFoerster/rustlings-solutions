// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.


struct Basket<T> { 
    items: Vec<T> 
} 

fn main() {

    let mut basket = Basket { items: vec![] };

    basket.items.push("banana");
    basket.items.push("apples");


    let mut basket_julia = Basket { items: vec![] };
    basket_julia.items.push((1,23));

    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
