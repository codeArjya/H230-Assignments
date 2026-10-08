const BIT_IS_ACTIVE: u32 = 0;
const BIT_IS_FULL: u32 = 1;
const BIT_IS_PRIVATE: u32 = 2;
const BIT_ENDIANNESS: u32 = 3; // the designated endianness bit

const GROUP_ID_SHIFT: u32 = 4;
const GROUP_ID_MASK: u32 = 0xFF; // 8 bits

const CAPACITY_SHIFT: u32 = 12;
const CAPACITY_MASK: u32 = 0xFFFF; // 16 bits

const STATUS_SHIFT: u32 = 28;
const STATUS_MASK: u32 = 0xF; // 4 bits

/// Generic single-bit flag extractor.
fn get_flag(state: u32, bit: u32) -> bool {
    (state >> bit) & 1 == 1
}

fn is_active(state: u32) -> bool {
    get_flag(state, BIT_IS_ACTIVE)
}

fn is_full(state: u32) -> bool {
    get_flag(state, BIT_IS_FULL)
}

fn is_private(state: u32) -> bool {
    get_flag(state, BIT_IS_PRIVATE)
}

fn is_big_endian(state: u32) -> bool {
    get_flag(state, BIT_ENDIANNESS)
}

fn get_group_id(state: u32) -> u8 {
    ((state >> GROUP_ID_SHIFT) & GROUP_ID_MASK) as u8
}

fn get_status_code(state: u32) -> u8 {
    ((state >> STATUS_SHIFT) & STATUS_MASK) as u8
}

fn get_capacity_raw(state: u32) -> u16 {
    ((state >> CAPACITY_SHIFT) & CAPACITY_MASK) as u16
}

fn get_capacity(state: u32) -> u16 {
    let raw = get_capacity_raw(state);
    if is_big_endian(state) {
        raw.to_be() // reorder bytes into big-endian representation
    } else {
        raw
    }
}

fn build_state(active: bool, full: bool, private: bool, big_endian: bool,
                group_id: u8, capacity: u16, status: u8) -> u32 {
    let mut state: u32 = 0;
    if active { state |= 1 << BIT_IS_ACTIVE; }
    if full { state |= 1 << BIT_IS_FULL; }
    if private { state |= 1 << BIT_IS_PRIVATE; }
    if big_endian { state |= 1 << BIT_ENDIANNESS; }
    state |= ((group_id as u32) & GROUP_ID_MASK) << GROUP_ID_SHIFT;
    state |= ((capacity as u32) & CAPACITY_MASK) << CAPACITY_SHIFT;
    state |= ((status as u32) & STATUS_MASK) << STATUS_SHIFT;
    state
}

fn main() {
    let capacity_value: u16 = 300; // 0x012C

    let state_le = build_state(true, false, true, false, 7, capacity_value, 2);
    let state_be = build_state(true, false, true, true, 7, capacity_value, 2);

    for (label, state) in [("little-endian flag", state_le), ("big-endian flag", state_be)] {
        println!("--- {label} ---");
        println!("  raw state bits : {state:#034b}");
        println!("  is_active      : {}", is_active(state));
        println!("  is_full        : {}", is_full(state));
        println!("  is_private     : {}", is_private(state));
        println!("  is_big_endian  : {}", is_big_endian(state));
        println!("  group_id       : {}", get_group_id(state));
        println!("  status_code    : {}", get_status_code(state));
        println!("  capacity_raw   : {} (bytes: {:02X?})", get_capacity_raw(state), get_capacity_raw(state).to_ne_bytes());
        let cap = get_capacity(state);
        println!("  capacity_out   : {} (bytes: {:02X?})", cap, cap.to_ne_bytes());
        println!();
    }
}
