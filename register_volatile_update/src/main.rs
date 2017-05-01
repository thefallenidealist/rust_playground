// example how to update some register bits
// created 170501

extern crate volatile;
extern crate bit_field;

use volatile::*;
use bit_field::BitField;

// aliases for Volatile types:
type RW<T> = Volatile<T>;
type RO<T> = ReadOnly<T>;
type WO<T> = WriteOnly<T>;

struct Register
{
	r1: RW<u32>,
	r2: RO<u32>,
	r3: WO<u32>,
}

impl Register
{
	// on bare metal:
	// pub unsafe fn new() -> &'static register
	// {
		// &mut *(0x40047000 as *mut register)
	// }
	pub fn new() -> Register
	{
		Register
		{
			r1: RW::new(0u32),
			r2: RO::new(888u32),
			r3: WO::new(0u32),
		}
	}
}

fn main()
{
	// -----------------------------------
	// volatile bit field on a variable:
	// -----------------------------------
	let mut var = Volatile::new(0x0000u32);

	println!("var: {:032b} dec: {}", var.read(), var.read());

	var.update(|new| { new.set_bit(1, true); });
	var.update(|new| { new.set_bit(3, true); });
	var.update(|new| { new.set_bits(5..8, 0b111); });
	var.update(|new| { new.set_bits(20..32, 0b111111111111); });
	var.update(|new| { new.set_bits(22..24, 0b00); });

	println!("var: {:032b} dec: {}", var.read(), var.read());
	println!("     {:032} ", "10987654321098765432109876543210");

	// -----------------------------------
	// volatile bit field on a struct:
	// -----------------------------------

	println!("----------------------------------------");
	let mut reg = Register::new();
	println!("reg r1: {:032b} dec: {}", reg.r1.read(), reg.r1.read());
	println!("reg r2: {:032b} dec: {}", reg.r2.read(), reg.r2.read());
	// R3 is WriteOnly:
	// println!("reg r3: {:032b} dec: {}", reg.r3.read(), reg.r3.read());

	println!("----------------------------------------");
	reg.r1.update(|new| { new.set_bit(8, true); } );
	reg.r1.update(|new| { new.set_bits(10..12, 0b11); } );
	// R2 is ReadOnly:
	// reg.r2.update(|new| { new.set_bit(9, true); } );
	// R3 is WriteOnly:
	reg.r3.write(0xABCD);

	println!("reg r1: {:032b} dec: {}", reg.r1.read(), reg.r1.read());
	println!("reg r2: {:032b} dec: {}", reg.r2.read(), reg.r2.read());

	println!("main() END");
}
