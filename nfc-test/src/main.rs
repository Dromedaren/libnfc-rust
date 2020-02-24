use libnfc::*;

fn main() -> Result<(), Error> {
    let mut context = Context::init().unwrap();
    
    println!("libnfc version: {:?}", context.lib_version());

    let mut device = context.open_default_device()?;
    device.initiator_init()?;

    println!("Device name: {}", device.name());
    println!("Device connection string: {}", device.connstring());
    println!("Device information:\n {:?}", device.information()?);
    let modulation = Modulation::new(ModulationType::NmtIso14443A, BaudRate::Nbr106);

    println!("Modulation set to: {:?}", modulation.str_modulation_type());
    println!("Baud rate set to: {:?}", modulation.str_baud_rate());

    let mut target = device.initiator_select_passive_target_default(modulation).expect("Issue waiting for card");

    let verbose = true;
    for target_information in target.str_target(verbose).unwrap().to_str().unwrap().split('\n') {
        println!("{}", target_information);
    }

    let iso14443a = target.get_iso14443a_info();
    println!("Tag UID: {}", iso14443a.hex_uid());

    Ok(())
}
