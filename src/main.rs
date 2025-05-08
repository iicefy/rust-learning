mod data_types;
mod functions;
mod variables;

fn main() {
    variables::show_variables();
    println!("--- Scalar Types ---");
    data_types::show_scalar_types();
    println!("--- Compound Types ---");
    data_types::show_compound_types();
    println!("--- Functions ---");

    functions::greet("Ice");

    let result = functions::square(6);
    println!("Square of 6 is: {}", result);

    functions::describe_number(5);
}
