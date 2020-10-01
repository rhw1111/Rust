extern crate lazy_static; 
extern crate fwlibrary;
use std::ops::Deref;
use std::ops::DerefMut;


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



struct EntityAReal1IMP();
impl EntityAReal1IMP{
    fn new()->EntityAReal1IMP
    {
        EntityAReal1IMP{}
    }
}
impl fwlibrary::EntityAIMP for EntityAReal1IMP 
{
    fn test(&self,entity:&fwlibrary::EntityA)->&str{
        "fffff"
    }
}



fn main() {


    /*unsafe{
    let x:Arc<dyn Fn()->Box<dyn fwlibrary::EntityAIMP>+Send+Sync>=Arc::new(||->Box<dyn fwlibrary::EntityAIMP>
        {

            Box::new(EntityAReal1IMP::new())
           
        });
        fwlibrary::ENTITYAIMPSTATICFACTORY = Some(x);
    }*/

    let mut entitya=fwlibrary::EntityA::new();

    let rentitya=&entitya;
    let ma=entitya.test();

    entitya.setid(String::from("11"));
    entitya.setremark(Some(String::from("xxxx")));
    let aid=entitya.getid();
    entitya.setremark(Option::Some(String::from("cccc")));
    let aremark=entitya.getremark();

    let remark= aremark.unwrap();


    /*let mut ii: V=V{};
    let  iia=&mut ii;
    vtest(iia);

    let ii1=ii;
   




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
    println!("{}","");*/
}
