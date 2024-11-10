pub mod puml {
    pub mod common {
        pub mod constants;
    }

    pub mod code_generators {
        pub mod code_generator;
        pub mod java;
    }

    pub mod core_parser {
        pub mod class;
        pub mod parser;
        pub mod regex;
        pub mod regex_constants;
    }
}