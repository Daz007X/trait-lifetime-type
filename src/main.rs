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