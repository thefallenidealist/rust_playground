// INFO no need to use "mod" keywork when module is separe file

pub mod core_registers; // declare submodule (relative to this submodule)

// functions in this (arm_v7m) submodule
pub fn test()
{
	println!("arm_v7m::test() \t\t\t from src/arm_v7m/mod.rs");
}
