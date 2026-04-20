use std::process::exit;

#[derive(PartialEq, Debug)]
pub enum RunningAs
{
    User,
    Root
}


#[cfg(unix)]
pub fn check() -> RunningAs
{
    let id = match std::env::var("USER")
    {
        Err(e) => {
            println!("Something went wrong{:?}", e);
            exit(2);
        }
        Ok(name) => name

    };
    
    match id.as_str()
    {
        "root" => {return RunningAs::Root;},
        _ => {return RunningAs::User}
    }

}
