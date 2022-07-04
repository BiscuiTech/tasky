mod todo;
mod io;
use io::read_data;


fn main() {
    println!("Tasky!");
    println!("-----");

    // call read_data
    let content = read_data();
    // iterate through the vector of Todos
    for todo in content {
        println!("{}", todo.print());
    }
}
