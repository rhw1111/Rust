#[macro_use]
extern crate lazy_static; 
extern crate fwlibrary;
use std::ops::Deref;
use std::ops::DerefMut;
use std::default;
use std::sync::Arc;
use std::collections::HashMap;
use std::sync::Once;

struct A{
    name:String
}

struct B
{
    a:A,
    role:String
}

struct C 
{
    b:B,
    age:i32
}

impl A{
    fn new()->A
    {
        A
        {
            name:"w".to_string()
        }
    }
    fn a1(&self)->&str
    {
        &*self.name
    }
}

impl C{
    fn new()->C{
        C{
            b:B::new(),
            age:12
        }
    }
    fn c1(&self)->i32{
        self.age
    }
}

impl B{
    fn new()->B{
        B{
            a:A::new(),
            role:"r".to_string()
        }
    }
    fn b1(&self)->&str
    {
        &*self.role
    }
}

impl Deref for B{
    type Target=A;
    fn deref<'a>(&'a self)->&'a A{
        &self.a
    }
}
impl DerefMut for B{
    fn deref_mut<'a>(&'a mut self)->&'a mut A{
        &mut self.a
    }
}

impl Deref for C{
    type Target=B;
    fn deref<'a>(&'a self)->&'a B{
        &self.b
    }
}

impl DerefMut for C{
    fn deref_mut<'a>(&'a mut self)->&'a mut B{
        &mut self.b
    }
}

fn a1(c:&C)
{

}
fn a2(c:C)->C
{
    c
}

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}


struct V();

fn vtest(v:& mut V)
{

}

static INIT: Once = Once::new();
static mut EntityAIMPStaticFactory:Option<Arc<dyn Fn()->Box<dyn EntityAIMP>+Send+Sync>>=None;



/*lazy_static! {
    static ref EntityAIMPStaticFactory:HashMap<&'static str,Arc<dyn Fn()->Box<dyn EntityAIMP>+Send+Sync>> =
    {

        let mut map=HashMap::new();

        let x:Arc<dyn Fn()->Box<dyn EntityAIMP>+Send+Sync>=Arc::new(||->Box<dyn EntityAIMP>
            {
    
                Box::new(EntityARealIMP::new())
               
            });
        map.insert("imp", x);

        map
    };
    
}*/


struct EntityA<'a>{
    base:fwlibrary::ModelBase<'a>,
    imp:Box<dyn EntityAIMP>

}


trait EntityAIMP
{
    fn test(&self, entity:&EntityA)->&str;
} 






impl<'a> EntityA<'a>{
    fn new()->EntityA<'a>{
        
        let realimp:Box<dyn EntityAIMP>;
        unsafe {
            INIT.call_once(|| {
                let x:Arc<dyn Fn()->Box<dyn EntityAIMP>+Send+Sync>=Arc::new(||->Box<dyn EntityAIMP>
                    {
            
                        Box::new(EntityARealIMP::new())
                       
                    });
                EntityAIMPStaticFactory = Some(x);
                
            });


            if let Some(v) = &EntityAIMPStaticFactory{
                realimp=v();
            }
            else
            {
                realimp=Box::new(EntityARealIMP::new());
            }

        }

        EntityA{
            base:fwlibrary::ModelBase::new(),
            imp:realimp
        }
    
    }

    fn getid(&self)->&str{
        let value=self.base.getattribute::<String>("id");
        let mut result:&str =Default::default();
        if let Some(v) = value {
            result=v;
        } 
        
        result
    }

    fn setid(&mut self,value:String){
        self.base.setattribute::<String>("id",value);
    }

    fn getremark(&self)->Option<&str>{
        let value=self.base.getattribute::<Option<String>>("remark");
        let mut result:Option<&str> = Option::None;
        if let Some(v) = value {
            if let Some(iv)=v{
                result=Option::Some(iv);     
            }
            
        } 

        result
    }

    fn setremark(&mut self,value:Option<String>){
        self.base.setattribute::<Option<String>>("remark",value);
    }

    fn test(&self)->&str{
        self.imp.test(self)
    }

}

struct EntityARealIMP();
impl EntityARealIMP{
    fn new()->EntityARealIMP
    {
        EntityARealIMP{}
    }
}
impl EntityAIMP for EntityARealIMP 
{
    fn test(&self,entity:&EntityA)->&str{
        "eeee"
    }
}


struct EntityAReal1IMP();
impl EntityAReal1IMP{
    fn new()->EntityAReal1IMP
    {
        EntityAReal1IMP{}
    }
}
impl EntityAIMP for EntityAReal1IMP 
{
    fn test(&self,entity:&EntityA)->&str{
        "fffff"
    }
}



fn main() {



    let mut ii: V=V{};
    let  iia=&mut ii;
    vtest(iia);

    let ii1=ii;
   

    let mut entitya=EntityA::new();

    let ma=entitya.test();

    entitya.setid(String::from("11"));
    entitya.setremark(Some(String::from("xxxx")));
    let aid=entitya.getid();
    entitya.setremark(Option::Some(String::from("cccc")));
    let aremark=entitya.getremark();

    let remark= aremark.unwrap();


    let mut e1=1;
    let mut e2=&e1;

    let mut e3=e2+4;
   
    let mut model=fwlibrary::ModelBase::new();
    model.setattribute::<String>("aa",String::from("11"));
    let modelV=model.getattribute::<String>("aa");
    let v=modelV.unwrap();
    let c=C::new();

    let l=&c;

 
    //a2(c);
    //let str1=c.a1();
    println!("{}","");
}
