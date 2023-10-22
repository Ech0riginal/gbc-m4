// TODO remember that Rust's ranges are inclusive only at the head.


pub const CART: &[u8] = include_bytes!("red.gb");


pub fn entry<'c>() -> &'c [u8] { &CART[0x100..0x104] }


pub fn logo<'c>() -> &'c [u8] { &CART[0x104..0x134] }


pub fn title<'c>() -> &'c [u8] {
    let arr: &[u8] = &CART[0x134..0x144];
    let mut i = arr.len() - 1;
    // 'trim' off zeros
    while i > 0 {
        if arr[i.clone()] != 0 { break; }
        i -= 1;
    }

    &CART[0x134..0x135+i]
}


pub fn manufacturer_code<'c>() -> &'c [u8] { &CART[0x13f..0x143] }


pub fn cgb_flag<'c>() -> &'c u8 { &CART[0x143] }


pub fn new_licensee_code<'c>() -> &'c [u8] { &CART[0x144..0x146] }


pub fn sgb_flag<'c>() -> &'c u8 { &CART[0x146] }

// AKA MBC-type
pub fn cart_type<'c>() -> &'c u8 { &CART[0x147] }


pub fn rom_size<'c>() -> &'c u8 { &CART[0x148] }


pub fn ram_size<'c>() -> &'c u8 { &CART[0x149] }


pub fn dest_code<'c>() -> &'c u8 { &CART[0x14a] }


pub fn old_licensee_code<'c>() -> &'c u8 { &CART[0x14b] }


pub fn mask_rom_ver_number<'c>() -> &'c u8 { &CART[0x14c] }


pub fn header_checksum<'c>() -> &'c u8 { &CART[0x14d] }


pub fn global_checksum<'c>() -> &'c [u8] { &CART[0x14e..0x150] }


pub fn game_data<'c>() -> &'c [u8] { &CART[0x150..0x3fff] }


#[cfg(std)]
pub fn debug() {
    println!(
           "Cart {{
    entry: {:?}
    logo: {:?}
    title: {}
    manufacturer_code: {}
    cgb_flag: {}
    new_licensee_code: {}
    sgb_flag: {}
    cart_type: {}
    rom_size (machine): {}
    ram_size (machine): {}
    dest_code: {}
    old_licensee_code: {}
    mask_rom_ver_number: {}
    header_checksum: {}
    global_checksum: {:?}
}}",
            entry(),
            logo(),
            to_str(title()),
            to_str(manufacturer_code()),
            cgb_flag(),
            to_str(new_licensee_code()),
            sgb_flag(),
            cart_type(),
            rom_size(),
            ram_size(),
            dest_code(),
            old_licensee_code(),
            mask_rom_ver_number(),
            header_checksum(),
            global_checksum(),
    );
}

#[inline]
#[cfg(std)]
fn to_str(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).unwrap()
}




#[cfg(test)]
#[cfg(std)]
mod tests {
    use crate::cart;

    #[test]
    fn debuggery() {
        cart::debug();
    }


    #[test]
    fn logo() {
        let expected = [
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
            0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
            0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
            0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
            0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
            0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        let actual = cart::logo();

        assert_eq!(expected, actual);
    }
}