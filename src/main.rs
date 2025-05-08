mod control_flow;
mod data_types;
mod enums;
mod functions;
mod ownership;
mod structs;
mod variables;

fn main() {
    println!("--- Variables ---");
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

    println!("--- Control Flow ---");
    control_flow::test_if_statement();
    control_flow::test_match_statement();
    control_flow::test_loops();

    println!("--- Structs ---");
    structs::test_structs();

    println!("--- Enums ---");
    enums::test_status_enum();
    enums::test_message_enum();

    println!("--- Ownership ---");
    ownership::ownership_example();
    ownership::borrowing_example();
    ownership::mutable_borrowing_example();
}
