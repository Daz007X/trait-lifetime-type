use std::fmt::{Debug, Display};

trait Example {
    type A;
    type B<T> where T: ToString + Display + Debug;
    type C<'a>;
    type D<'a> 
    where 
        Self:'a;  
    type E<'a, T> 
        where T: ToString + Display + Debug + 'a;
    type F<'a, T> where 
        T: ToString + Display + Debug + 'a,
        Self: 'a;


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
}

struct Ex1;

impl Example for Ex1 {
    type A = i32;
    type B<Tb> = Tb where Tb: ToString + Display + Debug;
    type C<'c1> = &'c1 Self::A where Self:'c1 ;
    type D<'d1, Td> = &'d1 Td where 
        Td: ToString + Display + Debug + 'd1,
        Self: 'd1;
    
    fn one_a(&self, a1: Self::A) -> Self::A {
        a1
    }
    
    fn one_b<Tb1>(&self, b1: Self::B<Tb1>) -> Self::B<Tb1>
    where
        Tb1: ToString + Display + Debug,
    {
        b1
    }
    
    fn one_c<'oc>(&'oc self, c1: Self::C<'oc>) -> Self::C<'oc> {
        c1
    }
    
    fn one_d<'od, Td1>(&'od self, d1: Self::D<'od, Td1>) -> Self::D<'od, Td1>
    where
        Td1: ToString + Display + Debug + 'od,
    {
        d1
    }
}

fn main() {
    let example = Ex1;
    
    // ใช้เมธอด one_a ที่ทำงานกับ type A (i32)
    let num = example.one_a(42);
    println!("one_a: {}", num);  // ผลลัพธ์: one_a: 42
    
    // ใช้เมธอด one_b ที่ทำงานกับ type B<T> (T ที่ต้องเป็น ToString + Display + Debug)
    let text = example.one_b("Hello");
    println!("one_b: {}", text);  // ผลลัพธ์: one_b: Hello
    
    // ใช้เมธอด one_c ที่ทำงานกับ type C<'a> (&'a i32)
    let value = 100;
    let ref_value = example.one_c(&value);
    println!("one_c: {}", ref_value);  // ผลลัพธ์: one_c: 100
    
    // ใช้เมธอด one_d ที่ทำงานกับ type D<'a, T> (&'a T)
    let message = "Rust";
    let ref_message = example.one_d(&message);
    println!("one_d: {}", ref_message);  // ผลลัพธ์: one_d: Rust
}