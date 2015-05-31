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

mod maya {
    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope
        print!("called `maya::indirect_called()` that\n>");

        function();

        {
            // This will bind to the `cool::function` in the *crate*s cope
            // In this case the crate scope is the outermost scope
            //use cool::function as root_cool_function;

            print!("> ");
            //root_cool_function();
        }

        {
            // `self` refers to the current module scope, in this case 'maya'
            //use self::cool::function as my_cool_function;

            print!("> ");
            //my_cool_function();
        }

        {
            // `super` refers to the parent scope, i.e. outside of `maya`
            //use super::function as root_function;

            print!("> ");
            //root_function();
        }
    }

    fn function() {
        println!("called `maya:function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `maya::cool::function()`");
        }
    }
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}


pub fn main() {
    function();    
    my::function();
    //maya::indirect_call();
}
