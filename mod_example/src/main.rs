// created 170808
// example how to use submodules and sub-submodule
mod arm_v7m;	// declare master submodule in main

/*
 * main
 * - arm_v7m					- declared above
 * - arm_v7m::core_registers	- declared in arm_v7m/mod.rs
 */

fn main()
{
	println!("----------------------------------------");
	arm_v7m::test();
	arm_v7m::core_registers::test();

	println!("main() end");
}
