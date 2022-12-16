use std::fs::{read_to_string, write};
use std::path::Path;
use std::error::Error;
use log::info;
use nix::{sys::signal::{kill, Signal}, unistd::Pid};

pub fn replace_old_gameserver(pidfile: &str) -> Result<(), Box<dyn Error>> {
    match read_to_string(pidfile) {
        Err(e) =>
            if e.kind() == std::io::ErrorKind::NotFound {
                info!("pidfile not found, assuming not already running");
                Ok(())
            } else {  
                info!("Error reading pidfile (other than NotFound): {}", e);
                Err(Box::new(e) as Box::<dyn Error>)
            }
        Ok(f) => {
            let pid: Pid = Pid::from_raw(f.parse().map_err(|e| Box::new(e) as Box::<dyn Error>)?);
            match read_to_string(format!("/proc/{}/cmdline", pid)) {
                Ok(content) =>
                    if content.contains("blastmud_game") {
                        info!("pid in pidfile references blastmud_game; starting cutover");
                        kill(pid, Signal::SIGUSR1)
                            .map_err(|e| Box::new(e) as Box<dyn Error>)
                    } else {
                        info!("Pid in pidfile is for process not including blastmud_game - ignoring pidfile");
                        Ok(())
                    }
                Err(_) => {
                    info!("Pid in pidfile is gone - ignoring pidfile");
                    Ok(())
                }
            }
        }
    }?;
    info!("Writing new pidfile");
    write(Path::new(pidfile), format!("{}", std::process::id()))
        .map_err(|e| Box::new(e) as Box::<dyn Error>)
}
