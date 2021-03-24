https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=cf8de5ede0984fe951793bf93f2fdc9c
fn main() {
    
    let my_vec : Vec<i32> = (0..100).collect();
    let result = my_vec.iter().filter(|&x| *x % 3 != 0).map(|x| x * x );
    for item in result {
        println!("Result: {}", item);
    }
}