#[percpu::def_percpu]
static CPU_ID: usize = usize::MAX;

#[inline(always)]
pub fn cpu_id() -> usize {
    CPU_ID.read_current()
}

pub fn init_percpu_primary(cpu_id: usize) {
    percpu::init_in_place().unwrap();
    percpu::init_percpu_reg(cpu_id);
    CPU_ID.write_current(cpu_id);
    println!("percpu use {:#x} mem", percpu::percpu_area_layout_expected(1).size());
}

pub fn init_percpu_secondary(cpu_id: usize) {
    percpu::init_percpu_reg(cpu_id);
    CPU_ID.write_current(cpu_id);
}