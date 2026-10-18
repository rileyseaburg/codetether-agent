//! OKR report okr render behavior.
use codetether_okr::Okr;
pub(super) fn print(okr: &Okr) -> () {
    println!("\n=== OKR Report: {} ===\n", okr.title);
    println!("Description: {}", okr.description);
    println!("Status: {:?}", okr.status);
    println!("Overall Progress: {:.1}%\n", okr.progress() * 100.0);
}
