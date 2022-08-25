const STARTING_MISSILE:i32 = 8;
const READY_AMOUNT:i32 = 2;
fn main() {
    let missiles:i32 = 8;
    let ready:i32 = 2;
    println!("Firing {} of my {} missiles...",ready,missiles);
    println!("{} missiles left",missiles-ready);
    println!("Firing {} of my {} missiles",STARTING_MISSILE,READY_AMOUNT);
    println!("{} missiles left",STARTING_MISSILE);
}
