mod ModA {
    pub fn function1(temp:f32)->f32{
        super::ModB::function2(temp*9.0)
    }
}
mod ModB {
    pub fn function2(temp:f32)->f32{
        super::ModC::function3(temp/5.0)
    }
}
mod ModC {
    pub fn function3(temp:f32)->f32{
        temp +32.0 
    }
}
mod ModD {
    pub fn function4(temp:f32)->f32{
        super::ModE::function5(temp-32.0) 
    }
}
mod ModE {
    pub fn function5(temp:f32)->f32{
        super::ModF::function6(temp*5.0)
    }
}
mod ModF {
    pub fn function6(temp:f32)->f32{
        temp/9.0
    }
}


fn main() {
    let data = ModA::function1(37.0);
    println!("{}",data);
    println!("{}",ModD::function4(data));
}
