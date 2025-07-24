use std::marker::PhantomData;

trait Template {
    type TemplateData<'t,T> where T: 't; 
    
    
    fn render<'r,R>(&self, data: Self::TemplateData<'r,R>) -> Self::TemplateData<'r,R>
    where R: 'r;
}

struct MyTmpData01;
struct TmpData<'t,T> 
where T: 't
{
    a:&'t T,
    b: &'t T,
}


impl Template for MyTmpData01 
{
    type TemplateData<'t,T> = TmpData<'t,T> where T: 't;
    
    fn render<'r,R>(&self, data: Self::TemplateData<'r,R>) -> Self::TemplateData<'r,R>
    where R: 'r,
    {
        data
    }
}



struct TmpDataWithCtx<'t, 'd, T, D>
where
    T: 't,
    D: 'd,
{
    a: &'t T,
    b: &'t T,
    ctx: &'d D,
}


struct MyTmpData02<'d, D> {
    _marker: PhantomData<&'d D>, //help: consider removing `'d`, referring to it in a field, or using a marker such as `PhantomData`
}



impl<'d, D> Template for MyTmpData02<'d, D> 
where
    D: 'd,
{
    type TemplateData<'t, T> = TmpDataWithCtx<'t, 'd, T, D>  where T: 't;
   

    fn render<'r, R>(&self, data: Self::TemplateData<'r, R>) -> Self::TemplateData<'r, R>
    where
        R: 'r,
    {
        data
    }
}



/*

*************** Stack Overflow Code Base*******************

URL : https://stackoverflow.com/questions/72469644/how-do-i-specify-an-associated-type-with-a-lifetime-parameter

impl Template for MyTmpData {
    type TemplateData<'t,T> where T: 't = TmpData<'t,T>;
    
    fn render<'r,T>(&self, data: &Self::TemplateData<'r,T>) -> &Self::TemplateData<'r,T>
    {
        data
    }
}


struct MyTemplateData<'a,T> {
    title: &'a T,
    author: &'a T,
    body: &'a str,
}

struct MyTemplate<'a,T>;

impl Template for MyTemplate {
    type TemplateData = MyTemplateData<'a>;

    fn render(&self, data: &MyTemplateData<'_>) -> String {
        /* snip */
    }
}


impl Template for MyTemplate {
    type TemplateData<'a> = MyTemplateData<'a>;

    fn render(&self, data: &MyTemplateData<'_>) -> String {
        /* snip */
    }
}
*/


