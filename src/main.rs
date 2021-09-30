mod print;
mod variables;
mod datatypes;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditions;
mod loops;
mod functions;
mod pointer_ref;
mod struct_;

fn main() {
    print::run();
    variables::run();
    datatypes::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditions::run();
    loops::run();
    functions::run();
    pointer_ref::run();
    struct_::run();
}
