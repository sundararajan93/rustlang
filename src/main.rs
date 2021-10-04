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
mod enums;
//mod cli_args;
//mod writing_file;
mod guess_game;

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
    enums::run();
    // cli_args::run();
    // writing_file::run();
    guess_game::run();
}
