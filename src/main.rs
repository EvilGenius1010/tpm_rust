mod tpm_comm;
use std::str::FromStr;
use tss_esapi::Context;
use tss_esapi::tcti_ldr::TctiNameConf;
use tpm_comm::get_initial_hash;

fn main() -> Result<(), String> {
    // Specify the TCTI configuration for swtpm
    let tcti = TctiNameConf::from_str("swtpm:port=2321")
        .map_err(|e| format!("Failed to initialize TCTI: {}", e))?;

    // Create the Context with the explicitly initialized TCTI
    let mut context = Context::new(tcti)
        .map_err(|e| format!("Failed to create TPM context: {}", e))?;

    // Get the initial hash
    let hash = get_initial_hash(&mut context)
        .map_err(|e| format!("Failed to get initial hash: {}", e))?;

    println!("Initial PCR hash: {:x?}", hash);
    Ok(())
}