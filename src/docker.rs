use std::process::Command;

pub fn run_command(args: Box<[&str]>) -> Result<String, ()>
{
    match Command::new("sudo").arg("docker").args(args.iter()).output() {
        Ok(output) => Ok(
            format!("{}", String::from_utf8_lossy(&output.stdout))
        ),
        Err(_) => Err(())
    }
}
