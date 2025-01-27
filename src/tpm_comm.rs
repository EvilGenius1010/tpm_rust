use tss_esapi::{
     structures::{ DigestList, PcrSelectionListBuilder, PcrSlot}, Context
};

use tss_esapi::interface_types::algorithm::HashingAlgorithm;
// pub trait TpmContext {
//     fn read_pcr(&self, pcr_index: u32) -> Result<PcrData, Tss2ResponseCode>;
// }

// impl TpmContext for Context {
//     fn read_pcr(&self, pcr_index: u32) -> Result<PcrData, Tss2ResponseCode> {
//         let pcr_selection_list = PcrSelectionListBuilder::new()
//             .with_selection(TPM2_ALG_SHA256, &[pcr_index])
//             .build();
        
//         let (update_counter, pcr_data) = self
//             .pcr_read(pcr_selection_list)
//             .map_err(|e| e.into())?; // Convert to `Tss2ResponseCode`
        
//         println!("PCR update counter: {}", update_counter);
//         Ok(pcr_data)
//     }
// }

// Function to get the initial hash
pub fn get_initial_hash(context: &mut Context) -> Result<DigestList, String> {
    // let pcr_data = context.read_pcr(pcr_index).map_err(|e| format!("Failed to read PCR: {}", e))?;
    let pcr_selection_list = PcrSelectionListBuilder::new()
    .with_selection(HashingAlgorithm::Sha256, &[PcrSlot::Slot0, PcrSlot::Slot1])
    .build()
    .expect("Failed to build PcrSelectionList");
    let pcr_op = context.pcr_read(pcr_selection_list);
    
    let digest_list = pcr_op.map(|(_, _, digest_list)| digest_list);

    match digest_list {
        Ok(digest_list) => Ok(digest_list),
        Err(e) => Err(format!("Failed to read initial hash. Error is {}",e))
    }
     
}
