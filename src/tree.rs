use crate::parser::Log;

pub fn display(logs: Vec<Log>) -> Result<(), String>
{
    println!("{:#?}", logs);
    Ok(())
}
