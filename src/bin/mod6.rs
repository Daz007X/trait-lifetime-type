use std::fmt::{Debug, Display};

// Define the Example trait
trait Example {
    type A;
    type B<T> where T: ToString + Display + Debug;
    type C<'a>;
    type D<'a> where Self: 'a;  
    type E<'a, T> where T: ToString + Display + Debug + 'a;
    type F<'a, T> where T: ToString + Display + Debug + 'a, Self: 'a;
    type G<T>;

    fn one_a(&self, a1: Self::A) -> Self::A;

    fn one_b<Tb>(&self, b1: Self::B<Tb>) -> Self::B<Tb>
    where
        Tb: ToString + Display + Debug;

    fn one_c<'ac>(&self, c1: Self::C<'ac>) -> Self::C<'ac>;

    fn one_d<'ad>(&'ad self, d1: Self::D<'ad>) -> Self::D<'ad>;

    fn one_e<'ae, Te>(&self, e1: Self::E<'ae, Te>) -> Self::E<'ae, Te>
    where
        Te: ToString + Display + Debug + 'ae;

    fn one_f<'af, Tf>(&'af self, d1: Self::F<'af, Tf>) -> Self::F<'af, Tf>
    where
        Tf: ToString + Display + Debug + 'af;
    
    fn one_g<Tg>(&self, g1: Self::G<Tg>) -> Self::G<Tg>; 
}

// Struct implementation
#[derive(Debug)]
struct MyStruct {
    value: i32,
}

impl Example for MyStruct {
    // Simple type for A
    type A = i32;
    
    // Wrapper type for B<T>, where T must satisfy ToString + Display + Debug
    type B<T> = T where T: ToString + Display + Debug;
    
    // Reference to a string for C<'a>
    type C<'a> = &'a str;
    
    // Reference to self's value for D<'a>, satisfying Self: 'a
    type D<'a> = &'a i32 where Self: 'a;
    
    // Wrapper type for E<'a, T>, with lifetime and trait bounds
    type E<'a, T> = &'a T where T: ToString + Display + Debug + 'a;
    
    // Wrapper type for F<'a, T>, with lifetime and Self: 'a
    type F<'a, T> = (T, &'a i32) where T: ToString + Display + Debug + 'a, Self: 'a;
    
    type G<T> = T;

    fn one_a(&self, a1: Self::A) -> Self::A {
        a1 // Return input unchanged
    }

    fn one_b<Tb>(&self, b1: Self::B<Tb>) -> Self::B<Tb>
    where
        Tb: ToString + Display + Debug,
    {
        b1 // Return input unchanged
    }

    fn one_c<'ac>(&self, c1: Self::C<'ac>) -> Self::C<'ac> {
        c1 // Return input unchanged
    }

    fn one_d<'ad>(&'ad self, d1: Self::D<'ad>) -> Self::D<'ad> {
        d1 // Return input unchanged
    }

    fn one_e<'ae, Te>(&self, e1: Self::E<'ae, Te>) -> Self::E<'ae, Te>
    where
        Te: ToString + Display + Debug + 'ae,
    {
        e1 // Return input unchanged
    }

    fn one_f<'af, Tf>(&'af self, d1: Self::F<'af, Tf>) -> Self::F<'af, Tf>
    where
        Tf: ToString + Display + Debug + 'af,
    {
        d1 // Return input unchanged
    }
    
    fn one_g<Tg>(&self, g1: Self::G<Tg>) -> Self::G<Tg>
    {
        g1
    }
}

// Enum implementation
#[derive(Debug)]
enum MyEnum {
    Number(i32),
    Text(String),
}

impl Example for MyEnum {
    // Simple type for A
    type A = String;
    
    // Wrapper type for B<T>
    type B<T> = Vec<T> where T: ToString + Display + Debug;
    
    // Reference to a string for C<'a>
    type C<'a> = &'a str;
    
    // Reference to self for D<'a>, satisfying Self: 'a
    type D<'a> = &'a Self where Self: 'a;
    
    // Wrapper type for E<'a, T>, with lifetime and trait bounds
    type E<'a, T> = &'a [T] where T: ToString + Display + Debug + 'a;
    
    // Wrapper type for F<'a, T>, with lifetime and Self: 'a
    type F<'a, T> = Option<T> where T: ToString + Display + Debug + 'a, Self: 'a;
    
    type G<T> = T;

    fn one_a(&self, a1: Self::A) -> Self::A {
        a1 // Return input unchanged
    }

    fn one_b<Tb>(&self, b1: Self::B<Tb>) -> Self::B<Tb>
    where
        Tb: ToString + Display + Debug,
    {
        b1 // Return input unchanged
    }

    fn one_c<'ac>(&self, c1: Self::C<'ac>) -> Self::C<'ac> {
        c1 // Return input unchanged
    }

    fn one_d<'ad>(&'ad self, d1: Self::D<'ad>) -> Self::D<'ad> {
        d1 // Return input unchanged
    }

    fn one_e<'ae, Te>(&self, e1: Self::E<'ae, Te>) -> Self::E<'ae, Te>
    where
        Te: ToString + Display + Debug + 'ae,
    {
        e1 // Return input unchanged
    }

    fn one_f<'af, Tf>(&'af self, d1: Self::F<'af, Tf>) -> Self::F<'af, Tf>
    where
        Tf: ToString + Display + Debug + 'af,
    {
        d1 // Return input unchanged
    }
    
    fn one_g<Tg>(&self, g1: Self::G<Tg>) -> Self::G<Tg>
    {
        g1
    }
}

// Example usage of the use_ex function
fn use_ex<'c, 'd, 'e, 'f,'ow, Exam, Af, Bf, Cf, Df, Ef, Ff, Gf, Tb, Te, Tf, Tg>(
    exam :&'ow Exam,
    a: Af,
    b: Bf,
    c: Cf,
    d: Df,
    e: Ef,
    f: Ff,
    g: Gf,
) where
    Exam: Example<A = Af, B<Tb> = Bf, C<'c> = Cf, D<'d> = Df, E<'e, Te> = Ef, F<'f, Tf> = Ff, G<Tg> = Gf> + 'c + 'd +'e +'f,
    Tb: ToString + Display + Debug ,
    Te: ToString + Display + Debug + 'e,
    Tf: ToString + Display + Debug + 'f,
    Af: Debug,
    Bf: Debug,
    Cf: Debug,
    Df: Debug,
    Ef: Debug,
    Ff: Debug,
    Gf: Debug,
    'ow: 'd,
    'ow: 'f,
{
    // Using one_a with type A
    let a_result = exam.one_a(a);
    println!("one_a result: {:?}", a_result);
    
    // Using one_b with type B<Tb>
    let b_result = exam.one_b::<Tb>(b);
    println!("one_b result: {:?}", b_result);
    
    // Using one_c with type C<'c>
    let c_result = exam.one_c::<'c>(c);
    println!("one_c result: {:?}", c_result);
    
    // Using one_d with type D<'d>
    let d_result = exam.one_d::<'d>(d);
    println!("one_d result: {:?}", d_result);
    
    // Using one_e with type E<'e, Te>
    let e_result = exam.one_e::<'e,Te>(e);
    println!("one_e result: {:?}", e_result);
    
    // Using one_f with type F<'f, Tf>
    let f_result = exam.one_f::<'f,Tf>(f);
    println!("one_f result: {:?}", f_result);
    
    // Using one_g with type G<Tg>
    let g_result = exam.one_g::<Tg>(g);
    println!("one_g result: {:?}", g_result);
}

fn main() {
    // Example usage with MyStruct
    let my_struct = MyStruct { value: 42 };
    use_ex(
        &my_struct,
        10,                      // Af (i32)
        5,                       // Bf (i32)
        "hello",                 // Cf (&str)
        &42,                     // Df (&i32)
        &5,                      // Ef (&i32)
        (10, &42),              // Ff ((i32, &i32))
        10,                      // Gf (i32)
    );

    // Example usage with MyEnum
    let my_enum = MyEnum::Number(100);
    use_ex(
        &my_enum,
        String::from("test"),    // Af (String)
        vec![1, 2, 3],          // Bf (Vec<i32>)
        "world",                // Cf (&str)
        &MyEnum::Number(100),   // Df (&MyEnum)
        &[1, 2, 3],            // Ef (&[i32])
        Some(42),               // Ff (Option<i32>)
        10,                     // Gf (i32)
    );
}