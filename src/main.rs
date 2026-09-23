// Takes ownership of a player's session ID (our "core resource").
// `String` does NOT implement `Copy`, so calling this function MOVES
// the string in. The caller's original binding is invalidated afterward.
fn process_session_id(session_id: String) -> usize {
    println!("  [process_session_id] processing: {session_id}");
    session_id.len()
}

// Works with a room's capacity limit (our "system parameter").
// `i32` DOES implement `Copy`, so calling this function copies the
// 4-byte value in. The caller's original binding stays valid afterward.
fn validate_capacity(capacity: i32) -> bool {
    println!("  [validate_capacity] validating: {capacity}");
    capacity > 0 && capacity <= 64
}

fn main() {
    println!("--- String (session_id) ---");
    let session_id = String::from("session-a1b2c3");
    let len = process_session_id(session_id);
    // `session_id` was moved into process_session_id above.
    // It is no longer valid here. Uncommenting the next line fails to compile because the ownership has been moved.
    // println!("still have: {session_id}");
    println!("  session id length was: {len}");
    println!("  -> OBSERVATION: after the call, the original `session_id`/ 
    \n\t variable can no longer be used. Ownership moved into/
    \n\t process_session_id, so the compiler invalidated it here.");

    // If we need to keep using the original, we must clone it explicitly
    // *before* the call — this is a visible, deliberate heap allocation.
    let session_id_2 = String::from("session-d4e5f6");
    let _len2 = process_session_id(session_id_2.clone());
    println!("  still valid after cloning: {session_id_2}");

    println!("\n--- i32 (capacity) ---");
    let capacity: i32 = 32;
    let is_valid = validate_capacity(capacity);
    println!("  capacity is still usable here: {capacity}");
    println!("  is_valid: {is_valid}");
    println!("  -> OBSERVATION: after the call, `capacity` is still valid/
     \n\t and unchanged. validate_capacity received a bitwise/ 
     \n\t copy, not the original binding, so nothing was moved.");

    // We can even reuse it again immediately, no clone needed.
    let is_valid_again = validate_capacity(capacity);
    println!("  called again with the same variable, no issue: {is_valid_again}");
}