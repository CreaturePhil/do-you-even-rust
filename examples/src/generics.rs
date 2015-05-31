// A concrete type `T`.
struct T;

// The first use of `T` was not preceded by `<T>` so `Single` must
// be a concrete type. `T` is defined at the top.
struct Single(T);
//            ^ Here is `Single`s first use of the type `T`.

// The first use of `T` is precede by `<T>`. `SingleGen` must be generic and has
// not yet been specialized. `T` could be anything including `T` at the top.
struct SingleGen<T>(T);

// Conrete type.
struct S(T);

// Generic type.
struct SGen<T>(T);

// These functions all take ownership of the variable passed into
// them and immediately go out of scope freeing the variable.
//
// This has no preceding `<T>` so this must be a regular function.
fn die_regular(s: S) {}

// Has a `<T>` but it isn't preceded by `<T>` to make it generic.
// This is a regular function which takes `SGen<T>` which has
// been specialized to type `T` defined at the top.
fn die_generic_specialized_t(s: SGen<T>) {}

// A regular function taking `SGen<T>`. This function is generic over `i32`.
fn die_generic_specialized_i32<T>(s: SGen<T>) {}

// `<T>` is preceded by `<T>`. This function is generic over `T`.
fn die_generic<T>(s: SGen<T>) {}

struct Tup(f64,);
struct GenTup<T>(T,);

// impl of Tup
impl Tup {
    fn value(&self) -> &f64 { &self.0 }
}

// impl of GenTup for a generic type `T`
impl <T> GenTup<T> {
    fn value(&self) -> &T { &self.0 }
}

// Instantiating the types can be implicit or explicit.
pub fn main() {
    // Regular `Single`.
    let _s = Single(T);

    // `SingleGen` explicity specialized.
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen`'s implicitly specialized.
    let _t = SingleGen(T); // Uses `T` at top.
    let _i32 = SingleGen(6); // Uses `i32`.

    // Use the regular functions like normal
    die_regular(S(T)); // Concrete type.
    die_generic_specialized_t(SGen(T)); // Specialized generic type.
    die_generic_specialized_i32(SGen(6)); // Specialized generic type.

    // Explicitly specialize `die_generic()` to `char`.
    die_generic::<char>(SGen('a'));

    // Implicitly specialize `die_generic()` to `char`.
    die_generic(SGen('c'));

    let x = Tup(3.0);
    let y = GenTup(3i32);

    println!("{}, {}", x.value(), y.value());
}
