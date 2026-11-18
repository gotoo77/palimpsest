use palimpsest::compare;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    if args.next().as_deref() != Some("compare") || args.next().is_some() {
        eprintln!("Usage: palimpsest compare");
        return ExitCode::from(2);
    }

    match compare() {
        Ok(result) => {
            println!(
                "E001 - same initial world except claim visibility: {}",
                result.world_equal_except_mask
            );
            println!("Visible Claim: {:#?}", result.visible);
            println!("Masked Claim: {:#?}", result.masked);
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("E001 could not run: {error:?}");
            ExitCode::FAILURE
        }
    }
}
