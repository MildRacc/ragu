use sysinfo::{self, System};

pub struct Ragud
{


}

impl Ragud
{

    pub fn new() -> Self
    {
        
        let s = System::new_all();

        let processes = s.processes_by_name("ragud".as_ref());
    


        Ragud{}

    }

}
