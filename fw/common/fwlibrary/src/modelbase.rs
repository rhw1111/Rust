use std::collections::HashMap;
use std::any::Any;
use std::default;
use std::mem;

pub struct ModelBase<'a>{
    datas:HashMap<&'a str,Box<dyn Any>>
}

impl<'a> ModelBase<'a>{
    pub fn new()->ModelBase<'a>{
        ModelBase{
            datas:HashMap::new()
        }
    }
    pub fn getattribute<T: 'static>(&self,attname:&'a str)->Option<&T>
    {
        if let Some(v) = self.datas.get(&attname){
            if let Some(x) =(*v).downcast_ref::<T>()
            {

                return Some(x);
            }
            else
            {
                let v:Option<&T>=None;
                return  v;            
            }
        }
       else{
        let v:Option<&T>=None;
        return  v; 
       }

    }

    pub fn setattribute<T: 'static>(&mut self,attname:&'a str,value:T)
    {
        self.datas.insert(attname, Box::new(value));
    }

}