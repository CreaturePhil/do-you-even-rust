fn function() {
    println!("called `function()`");
}

// A module named `my`
mod my {
    // A module can contain items like functions
    #[allow(dead_code)]
    pub fn function() {
        println!("called `my::function()`");
    }

    // Modules can be nested
    mod nested {
        #[allow(dead_code)]
        fn function() {
            println!("called `my::nested::function()`");
        }
    }
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}


pub fn main() {
    function();    
    my::function();
}
