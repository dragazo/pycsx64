use std::sync::{Arc, Mutex};
use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use pyo3::types::PyBytes;

#[derive(Clone)]
#[pyclass]
struct ObjectFile(csx64::asm::ObjectFile);
#[pyclass]
struct Executable(csx64::common::Executable);
#[pyclass]
struct Emulator(csx64::exec::Emulator);
#[pyclass]
struct MemoryFile(Arc<Mutex<csx64::exec::fs::MemoryFile>>);

#[pymethods]
impl MemoryFile {
    fn get_content(&self, py: Python) -> PyObject {
        PyBytes::new(py, self.0.lock().unwrap().content.get_ref()).into()
    }
}

#[pymethods]
impl Emulator {
    #[new]
    fn new() -> Self {
        Emulator(csx64::exec::Emulator::new())
    }
    #[args(exe, "*", max_memory, stack_size, max_files, command_line_args = "vec![]")]
    fn init(&mut self, exe: Py<Executable>, max_memory: Option<usize>, stack_size: Option<usize>, max_files: Option<usize>, command_line_args: Vec<String>) {
        let args = csx64::exec::EmulatorArgs {
            max_memory,
            stack_size,
            max_files,
            command_line_args,
        };
        Python::with_gil(|py| self.0.init(&exe.borrow(py).0, &args));
    }
    #[args(cycles = "u64::MAX")]
    fn execute_cycles(&mut self, cycles: u64) -> (u64, &'static str) {
        let res = self.0.execute_cycles(cycles);
        let reason = match res.1 {
            csx64::exec::StopReason::NotRunning => "NotRunning",
            csx64::exec::StopReason::MaxCycles => "MaxCycles",
            csx64::exec::StopReason::ForfeitTimeslot => "ForfeitTimeslot",
            csx64::exec::StopReason::Terminated(_) => "Terminated",
            csx64::exec::StopReason::Error(_) => "Error",
        };
        (res.0, reason)
    }
    fn get_state(&self) -> &'static str {
        match self.0.get_state() {
            csx64::exec::State::Uninitialized => "Uninitialized",
            csx64::exec::State::Running => "Running",
            csx64::exec::State::Terminated(_) => "Terminated",
            csx64::exec::State::Error(_) => "Error",
        }
    }
    fn get_error(&self) -> Option<String> {
        match self.0.get_state() {
            csx64::exec::State::Error(err) => Some(format!("{:?}", err)),
            _ => None,
        }
    }
    fn get_return_value(&self) -> Option<i32> {
        match self.0.get_state() {
            csx64::exec::State::Terminated(res) => Some(res),
            _ => None,
        }
    }

    fn setup_stdio(&mut self) -> (MemoryFile, MemoryFile, MemoryFile) {
        let stdin = Arc::new(Mutex::new(csx64::exec::fs::MemoryFile { content: Default::default(), readable: true, writable: false, seekable: false, appendonly: false, interactive: true }));
        let stdout = Arc::new(Mutex::new(csx64::exec::fs::MemoryFile { content: Default::default(), readable: false, writable: true, seekable: false, appendonly: true, interactive: false }));
        let stderr = Arc::new(Mutex::new(csx64::exec::fs::MemoryFile { content: Default::default(), readable: false, writable: true, seekable: false, appendonly: true, interactive: false }));
        self.0.files.handles[0] = Some(stdin.clone());
        self.0.files.handles[1] = Some(stdout.clone());
        self.0.files.handles[2] = Some(stderr.clone());
        (MemoryFile(stdin), MemoryFile(stdout), MemoryFile(stderr))
    }

    #[getter] fn get_flags(&self) -> u64 { self.0.flags.0 }
    #[setter] fn set_flags(&mut self, value: u64) { self.0.flags.0 = value }

    #[getter] fn get_cf(&self) -> bool { self.0.flags.get_cf() }
    #[setter] fn set_cf(&mut self, value: bool) { self.0.flags.assign_cf(value) }
    #[getter] fn get_pf(&self) -> bool { self.0.flags.get_pf() }
    #[setter] fn set_pf(&mut self, value: bool) { self.0.flags.assign_pf(value) }
    #[getter] fn get_af(&self) -> bool { self.0.flags.get_af() }
    #[setter] fn set_af(&mut self, value: bool) { self.0.flags.assign_af(value) }
    #[getter] fn get_zf(&self) -> bool { self.0.flags.get_zf() }
    #[setter] fn set_zf(&mut self, value: bool) { self.0.flags.assign_zf(value) }
    #[getter] fn get_sf(&self) -> bool { self.0.flags.get_sf() }
    #[setter] fn set_sf(&mut self, value: bool) { self.0.flags.assign_sf(value) }
    #[getter] fn get_tf(&self) -> bool { self.0.flags.get_tf() }
    #[setter] fn set_tf(&mut self, value: bool) { self.0.flags.assign_tf(value) }
    #[getter] fn get_if(&self) -> bool { self.0.flags.get_if() }
    #[setter] fn set_if(&mut self, value: bool) { self.0.flags.assign_if(value) }
    #[getter] fn get_df(&self) -> bool { self.0.flags.get_df() }
    #[setter] fn set_df(&mut self, value: bool) { self.0.flags.assign_df(value) }
    #[getter] fn get_of(&self) -> bool { self.0.flags.get_of() }
    #[setter] fn set_of(&mut self, value: bool) { self.0.flags.assign_of(value) }
    #[getter] fn get_nt(&self) -> bool { self.0.flags.get_nt() }
    #[setter] fn set_nt(&mut self, value: bool) { self.0.flags.assign_nt(value) }
    #[getter] fn get_rf(&self) -> bool { self.0.flags.get_rf() }
    #[setter] fn set_rf(&mut self, value: bool) { self.0.flags.assign_rf(value) }
    #[getter] fn get_vm(&self) -> bool { self.0.flags.get_vm() }
    #[setter] fn set_vm(&mut self, value: bool) { self.0.flags.assign_vm(value) }
    #[getter] fn get_ac(&self) -> bool { self.0.flags.get_ac() }
    #[setter] fn set_ac(&mut self, value: bool) { self.0.flags.assign_ac(value) }
    #[getter] fn get_vif(&self) -> bool { self.0.flags.get_vif() }
    #[setter] fn set_vif(&mut self, value: bool) { self.0.flags.assign_vif(value) }
    #[getter] fn get_vip(&self) -> bool { self.0.flags.get_vip() }
    #[setter] fn set_vip(&mut self, value: bool) { self.0.flags.assign_vip(value) }
    #[getter] fn get_id(&self) -> bool { self.0.flags.get_id() }
    #[setter] fn set_id(&mut self, value: bool) { self.0.flags.assign_id(value) }
    #[getter] fn get_iopl(&self) -> u8 { self.0.flags.get_iopl() }
    #[setter] fn set_iopl(&mut self, value: u8) -> PyResult<()> { self.0.flags.assign_iopl(value).map_err(|v| PyRuntimeError::new_err(format!("IOPL out of bounds ({})", v))) }
    #[getter] fn get_ots(&self) -> bool { self.0.flags.get_ots() }
    #[setter] fn set_ots(&mut self, value: bool) { self.0.flags.assign_ots(value) }

    #[getter] fn get_cc_b(&self) -> bool { self.0.flags.condition_b() }
    #[getter] fn get_cc_be(&self) -> bool { self.0.flags.condition_be() }
    #[getter] fn get_cc_a(&self) -> bool { self.0.flags.condition_a() }
    #[getter] fn get_cc_ae(&self) -> bool { self.0.flags.condition_ae() }
    #[getter] fn get_cc_l(&self) -> bool { self.0.flags.condition_l() }
    #[getter] fn get_cc_le(&self) -> bool { self.0.flags.condition_le() }
    #[getter] fn get_cc_g(&self) -> bool { self.0.flags.condition_g() }
    #[getter] fn get_cc_ge(&self) -> bool { self.0.flags.condition_ge() }

    #[getter] fn get_rax(&self) -> u64 { self.0.cpu.get_rax() }
    #[setter] fn set_rax(&mut self, value: u64) { self.0.cpu.set_rax(value) }
    #[getter] fn get_rbx(&self) -> u64 { self.0.cpu.get_rbx() }
    #[setter] fn set_rbx(&mut self, value: u64) { self.0.cpu.set_rbx(value) }
    #[getter] fn get_rcx(&self) -> u64 { self.0.cpu.get_rcx() }
    #[setter] fn set_rcx(&mut self, value: u64) { self.0.cpu.set_rcx(value) }
    #[getter] fn get_rdx(&self) -> u64 { self.0.cpu.get_rdx() }
    #[setter] fn set_rdx(&mut self, value: u64) { self.0.cpu.set_rdx(value) }
    #[getter] fn get_rsi(&self) -> u64 { self.0.cpu.get_rsi() }
    #[setter] fn set_rsi(&mut self, value: u64) { self.0.cpu.set_rsi(value) }
    #[getter] fn get_rdi(&self) -> u64 { self.0.cpu.get_rdi() }
    #[setter] fn set_rdi(&mut self, value: u64) { self.0.cpu.set_rdi(value) }
    #[getter] fn get_r8(&self) -> u64 { self.0.cpu.get_r8() }
    #[setter] fn set_r8(&mut self, value: u64) { self.0.cpu.set_r8(value) }
    #[getter] fn get_r9(&self) -> u64 { self.0.cpu.get_r9() }
    #[setter] fn set_r9(&mut self, value: u64) { self.0.cpu.set_r9(value) }
    #[getter] fn get_r10(&self) -> u64 { self.0.cpu.get_r10() }
    #[setter] fn set_r10(&mut self, value: u64) { self.0.cpu.set_r10(value) }
    #[getter] fn get_r11(&self) -> u64 { self.0.cpu.get_r11() }
    #[setter] fn set_r11(&mut self, value: u64) { self.0.cpu.set_r11(value) }
    #[getter] fn get_r12(&self) -> u64 { self.0.cpu.get_r12() }
    #[setter] fn set_r12(&mut self, value: u64) { self.0.cpu.set_r12(value) }
    #[getter] fn get_r13(&self) -> u64 { self.0.cpu.get_r13() }
    #[setter] fn set_r13(&mut self, value: u64) { self.0.cpu.set_r13(value) }
    #[getter] fn get_r14(&self) -> u64 { self.0.cpu.get_r14() }
    #[setter] fn set_r14(&mut self, value: u64) { self.0.cpu.set_r14(value) }
    #[getter] fn get_r15(&self) -> u64 { self.0.cpu.get_r15() }
    #[setter] fn set_r15(&mut self, value: u64) { self.0.cpu.set_r15(value) }
    #[getter] fn get_raxi(&self) -> i64 { self.0.cpu.get_rax() as i64 }
    #[setter] fn set_raxi(&mut self, value: i64) { self.0.cpu.set_rax(value as u64) }
    #[getter] fn get_rbxi(&self) -> i64 { self.0.cpu.get_rbx() as i64 }
    #[setter] fn set_rbxi(&mut self, value: i64) { self.0.cpu.set_rbx(value as u64) }
    #[getter] fn get_rcxi(&self) -> i64 { self.0.cpu.get_rcx() as i64 }
    #[setter] fn set_rcxi(&mut self, value: i64) { self.0.cpu.set_rcx(value as u64) }
    #[getter] fn get_rdxi(&self) -> i64 { self.0.cpu.get_rdx() as i64 }
    #[setter] fn set_rdxi(&mut self, value: i64) { self.0.cpu.set_rdx(value as u64) }
    #[getter] fn get_rsii(&self) -> i64 { self.0.cpu.get_rsi() as i64 }
    #[setter] fn set_rsii(&mut self, value: i64) { self.0.cpu.set_rsi(value as u64) }
    #[getter] fn get_rdii(&self) -> i64 { self.0.cpu.get_rdi() as i64 }
    #[setter] fn set_rdii(&mut self, value: i64) { self.0.cpu.set_rdi(value as u64) }
    #[getter] fn get_r8i(&self) -> i64 { self.0.cpu.get_r8() as i64 }
    #[setter] fn set_r8i(&mut self, value: i64) { self.0.cpu.set_r8(value as u64) }
    #[getter] fn get_r9i(&self) -> i64 { self.0.cpu.get_r9() as i64 }
    #[setter] fn set_r9i(&mut self, value: i64) { self.0.cpu.set_r9(value as u64) }
    #[getter] fn get_r10i(&self) -> i64 { self.0.cpu.get_r10() as i64 }
    #[setter] fn set_r10i(&mut self, value: i64) { self.0.cpu.set_r10(value as u64) }
    #[getter] fn get_r11i(&self) -> i64 { self.0.cpu.get_r11() as i64 }
    #[setter] fn set_r11i(&mut self, value: i64) { self.0.cpu.set_r11(value as u64) }
    #[getter] fn get_r12i(&self) -> i64 { self.0.cpu.get_r12() as i64 }
    #[setter] fn set_r12i(&mut self, value: i64) { self.0.cpu.set_r12(value as u64) }
    #[getter] fn get_r13i(&self) -> i64 { self.0.cpu.get_r13() as i64 }
    #[setter] fn set_r13i(&mut self, value: i64) { self.0.cpu.set_r13(value as u64) }
    #[getter] fn get_r14i(&self) -> i64 { self.0.cpu.get_r14() as i64 }
    #[setter] fn set_r14i(&mut self, value: i64) { self.0.cpu.set_r14(value as u64) }
    #[getter] fn get_r15i(&self) -> i64 { self.0.cpu.get_r15() as i64 }
    #[setter] fn set_r15i(&mut self, value: i64) { self.0.cpu.set_r15(value as u64) }
    #[getter] fn get_raxf(&self) -> f64 { f64::from_bits(self.0.cpu.get_rax()) }
    #[setter] fn set_raxf(&mut self, value: f64) { self.0.cpu.set_rax(value.to_bits()) }
    #[getter] fn get_rbxf(&self) -> f64 { f64::from_bits(self.0.cpu.get_rbx()) }
    #[setter] fn set_rbxf(&mut self, value: f64) { self.0.cpu.set_rbx(value.to_bits()) }
    #[getter] fn get_rcxf(&self) -> f64 { f64::from_bits(self.0.cpu.get_rcx()) }
    #[setter] fn set_rcxf(&mut self, value: f64) { self.0.cpu.set_rcx(value.to_bits()) }
    #[getter] fn get_rdxf(&self) -> f64 { f64::from_bits(self.0.cpu.get_rdx()) }
    #[setter] fn set_rdxf(&mut self, value: f64) { self.0.cpu.set_rdx(value.to_bits()) }
    #[getter] fn get_rsif(&self) -> f64 { f64::from_bits(self.0.cpu.get_rsi()) }
    #[setter] fn set_rsif(&mut self, value: f64) { self.0.cpu.set_rsi(value.to_bits()) }
    #[getter] fn get_rdif(&self) -> f64 { f64::from_bits(self.0.cpu.get_rdi()) }
    #[setter] fn set_rdif(&mut self, value: f64) { self.0.cpu.set_rdi(value.to_bits()) }
    #[getter] fn get_r8f(&self) -> f64 { f64::from_bits(self.0.cpu.get_r8()) }
    #[setter] fn set_r8f(&mut self, value: f64) { self.0.cpu.set_r8(value.to_bits()) }
    #[getter] fn get_r9f(&self) -> f64 { f64::from_bits(self.0.cpu.get_r9()) }
    #[setter] fn set_r9f(&mut self, value: f64) { self.0.cpu.set_r9(value.to_bits()) }
    #[getter] fn get_r10f(&self) -> f64 { f64::from_bits(self.0.cpu.get_r10()) }
    #[setter] fn set_r10f(&mut self, value: f64) { self.0.cpu.set_r10(value.to_bits()) }
    #[getter] fn get_r11f(&self) -> f64 { f64::from_bits(self.0.cpu.get_r11()) }
    #[setter] fn set_r11f(&mut self, value: f64) { self.0.cpu.set_r11(value.to_bits()) }
    #[getter] fn get_r12f(&self) -> f64 { f64::from_bits(self.0.cpu.get_r12()) }
    #[setter] fn set_r12f(&mut self, value: f64) { self.0.cpu.set_r12(value.to_bits()) }
    #[getter] fn get_r13f(&self) -> f64 { f64::from_bits(self.0.cpu.get_r13()) }
    #[setter] fn set_r13f(&mut self, value: f64) { self.0.cpu.set_r13(value.to_bits()) }
    #[getter] fn get_r14f(&self) -> f64 { f64::from_bits(self.0.cpu.get_r14()) }
    #[setter] fn set_r14f(&mut self, value: f64) { self.0.cpu.set_r14(value.to_bits()) }
    #[getter] fn get_r15f(&self) -> f64 { f64::from_bits(self.0.cpu.get_r15()) }
    #[setter] fn set_r15f(&mut self, value: f64) { self.0.cpu.set_r15(value.to_bits()) }

    #[getter] fn get_eax(&self) -> u32 { self.0.cpu.get_eax() }
    #[setter] fn set_eax(&mut self, value: u32) { self.0.cpu.set_eax(value) }
    #[getter] fn get_ebx(&self) -> u32 { self.0.cpu.get_ebx() }
    #[setter] fn set_ebx(&mut self, value: u32) { self.0.cpu.set_ebx(value) }
    #[getter] fn get_ecx(&self) -> u32 { self.0.cpu.get_ecx() }
    #[setter] fn set_ecx(&mut self, value: u32) { self.0.cpu.set_ecx(value) }
    #[getter] fn get_edx(&self) -> u32 { self.0.cpu.get_edx() }
    #[setter] fn set_edx(&mut self, value: u32) { self.0.cpu.set_edx(value) }
    #[getter] fn get_esi(&self) -> u32 { self.0.cpu.get_esi() }
    #[setter] fn set_esi(&mut self, value: u32) { self.0.cpu.set_esi(value) }
    #[getter] fn get_edi(&self) -> u32 { self.0.cpu.get_edi() }
    #[setter] fn set_edi(&mut self, value: u32) { self.0.cpu.set_edi(value) }
    #[getter] fn get_r8d(&self) -> u32 { self.0.cpu.get_r8d() }
    #[setter] fn set_r8d(&mut self, value: u32) { self.0.cpu.set_r8d(value) }
    #[getter] fn get_r9d(&self) -> u32 { self.0.cpu.get_r9d() }
    #[setter] fn set_r9d(&mut self, value: u32) { self.0.cpu.set_r9d(value) }
    #[getter] fn get_r10d(&self) -> u32 { self.0.cpu.get_r10d() }
    #[setter] fn set_r10d(&mut self, value: u32) { self.0.cpu.set_r10d(value) }
    #[getter] fn get_r11d(&self) -> u32 { self.0.cpu.get_r11d() }
    #[setter] fn set_r11d(&mut self, value: u32) { self.0.cpu.set_r11d(value) }
    #[getter] fn get_r12d(&self) -> u32 { self.0.cpu.get_r12d() }
    #[setter] fn set_r12d(&mut self, value: u32) { self.0.cpu.set_r12d(value) }
    #[getter] fn get_r13d(&self) -> u32 { self.0.cpu.get_r13d() }
    #[setter] fn set_r13d(&mut self, value: u32) { self.0.cpu.set_r13d(value) }
    #[getter] fn get_r14d(&self) -> u32 { self.0.cpu.get_r14d() }
    #[setter] fn set_r14d(&mut self, value: u32) { self.0.cpu.set_r14d(value) }
    #[getter] fn get_r15d(&self) -> u32 { self.0.cpu.get_r15d() }
    #[setter] fn set_r15d(&mut self, value: u32) { self.0.cpu.set_r15d(value) }
    #[getter] fn get_eaxi(&self) -> i32 { self.0.cpu.get_eax() as i32 }
    #[setter] fn set_eaxi(&mut self, value: i32) { self.0.cpu.set_eax(value as u32) }
    #[getter] fn get_ebxi(&self) -> i32 { self.0.cpu.get_ebx() as i32 }
    #[setter] fn set_ebxi(&mut self, value: i32) { self.0.cpu.set_ebx(value as u32) }
    #[getter] fn get_ecxi(&self) -> i32 { self.0.cpu.get_ecx() as i32 }
    #[setter] fn set_ecxi(&mut self, value: i32) { self.0.cpu.set_ecx(value as u32) }
    #[getter] fn get_edxi(&self) -> i32 { self.0.cpu.get_edx() as i32 }
    #[setter] fn set_edxi(&mut self, value: i32) { self.0.cpu.set_edx(value as u32) }
    #[getter] fn get_esii(&self) -> i32 { self.0.cpu.get_esi() as i32 }
    #[setter] fn set_esii(&mut self, value: i32) { self.0.cpu.set_esi(value as u32) }
    #[getter] fn get_edii(&self) -> i32 { self.0.cpu.get_edi() as i32 }
    #[setter] fn set_edii(&mut self, value: i32) { self.0.cpu.set_edi(value as u32) }
    #[getter] fn get_r8di(&self) -> i32 { self.0.cpu.get_r8d() as i32 }
    #[setter] fn set_r8di(&mut self, value: i32) { self.0.cpu.set_r8d(value as u32) }
    #[getter] fn get_r9di(&self) -> i32 { self.0.cpu.get_r9d() as i32 }
    #[setter] fn set_r9di(&mut self, value: i32) { self.0.cpu.set_r9d(value as u32) }
    #[getter] fn get_r10di(&self) -> i32 { self.0.cpu.get_r10d() as i32 }
    #[setter] fn set_r10di(&mut self, value: i32) { self.0.cpu.set_r10d(value as u32) }
    #[getter] fn get_r11di(&self) -> i32 { self.0.cpu.get_r11d() as i32 }
    #[setter] fn set_r11di(&mut self, value: i32) { self.0.cpu.set_r11d(value as u32) }
    #[getter] fn get_r12di(&self) -> i32 { self.0.cpu.get_r12d() as i32 }
    #[setter] fn set_r12di(&mut self, value: i32) { self.0.cpu.set_r12d(value as u32) }
    #[getter] fn get_r13di(&self) -> i32 { self.0.cpu.get_r13d() as i32 }
    #[setter] fn set_r13di(&mut self, value: i32) { self.0.cpu.set_r13d(value as u32) }
    #[getter] fn get_r14di(&self) -> i32 { self.0.cpu.get_r14d() as i32 }
    #[setter] fn set_r14di(&mut self, value: i32) { self.0.cpu.set_r14d(value as u32) }
    #[getter] fn get_r15di(&self) -> i32 { self.0.cpu.get_r15d() as i32 }
    #[setter] fn set_r15di(&mut self, value: i32) { self.0.cpu.set_r15d(value as u32) }
    #[getter] fn get_eaxf(&self) -> f32 { f32::from_bits(self.0.cpu.get_eax()) }
    #[setter] fn set_eaxf(&mut self, value: f32) { self.0.cpu.set_eax(value.to_bits()) }
    #[getter] fn get_ebxf(&self) -> f32 { f32::from_bits(self.0.cpu.get_ebx()) }
    #[setter] fn set_ebxf(&mut self, value: f32) { self.0.cpu.set_ebx(value.to_bits()) }
    #[getter] fn get_ecxf(&self) -> f32 { f32::from_bits(self.0.cpu.get_ecx()) }
    #[setter] fn set_ecxf(&mut self, value: f32) { self.0.cpu.set_ecx(value.to_bits()) }
    #[getter] fn get_edxf(&self) -> f32 { f32::from_bits(self.0.cpu.get_edx()) }
    #[setter] fn set_edxf(&mut self, value: f32) { self.0.cpu.set_edx(value.to_bits()) }
    #[getter] fn get_esif(&self) -> f32 { f32::from_bits(self.0.cpu.get_esi()) }
    #[setter] fn set_esif(&mut self, value: f32) { self.0.cpu.set_esi(value.to_bits()) }
    #[getter] fn get_edif(&self) -> f32 { f32::from_bits(self.0.cpu.get_edi()) }
    #[setter] fn set_edif(&mut self, value: f32) { self.0.cpu.set_edi(value.to_bits()) }
    #[getter] fn get_r8df(&self) -> f32 { f32::from_bits(self.0.cpu.get_r8d()) }
    #[setter] fn set_r8df(&mut self, value: f32) { self.0.cpu.set_r8d(value.to_bits()) }
    #[getter] fn get_r9df(&self) -> f32 { f32::from_bits(self.0.cpu.get_r9d()) }
    #[setter] fn set_r9df(&mut self, value: f32) { self.0.cpu.set_r9d(value.to_bits()) }
    #[getter] fn get_r10df(&self) -> f32 { f32::from_bits(self.0.cpu.get_r10d()) }
    #[setter] fn set_r10df(&mut self, value: f32) { self.0.cpu.set_r10d(value.to_bits()) }
    #[getter] fn get_r11df(&self) -> f32 { f32::from_bits(self.0.cpu.get_r11d()) }
    #[setter] fn set_r11df(&mut self, value: f32) { self.0.cpu.set_r11d(value.to_bits()) }
    #[getter] fn get_r12df(&self) -> f32 { f32::from_bits(self.0.cpu.get_r12d()) }
    #[setter] fn set_r12df(&mut self, value: f32) { self.0.cpu.set_r12d(value.to_bits()) }
    #[getter] fn get_r13df(&self) -> f32 { f32::from_bits(self.0.cpu.get_r13d()) }
    #[setter] fn set_r13df(&mut self, value: f32) { self.0.cpu.set_r13d(value.to_bits()) }
    #[getter] fn get_r14df(&self) -> f32 { f32::from_bits(self.0.cpu.get_r14d()) }
    #[setter] fn set_r14df(&mut self, value: f32) { self.0.cpu.set_r14d(value.to_bits()) }
    #[getter] fn get_r15df(&self) -> f32 { f32::from_bits(self.0.cpu.get_r15d()) }
    #[setter] fn set_r15df(&mut self, value: f32) { self.0.cpu.set_r15d(value.to_bits()) }

    #[getter] fn get_ax(&self) -> u16 { self.0.cpu.get_ax() }
    #[setter] fn set_ax(&mut self, value: u16) { self.0.cpu.set_ax(value) }
    #[getter] fn get_bx(&self) -> u16 { self.0.cpu.get_bx() }
    #[setter] fn set_bx(&mut self, value: u16) { self.0.cpu.set_bx(value) }
    #[getter] fn get_cx(&self) -> u16 { self.0.cpu.get_cx() }
    #[setter] fn set_cx(&mut self, value: u16) { self.0.cpu.set_cx(value) }
    #[getter] fn get_dx(&self) -> u16 { self.0.cpu.get_dx() }
    #[setter] fn set_dx(&mut self, value: u16) { self.0.cpu.set_dx(value) }
    #[getter] fn get_si(&self) -> u16 { self.0.cpu.get_si() }
    #[setter] fn set_si(&mut self, value: u16) { self.0.cpu.set_si(value) }
    #[getter] fn get_di(&self) -> u16 { self.0.cpu.get_di() }
    #[setter] fn set_di(&mut self, value: u16) { self.0.cpu.set_di(value) }
    #[getter] fn get_r8w(&self) -> u16 { self.0.cpu.get_r8w() }
    #[setter] fn set_r8w(&mut self, value: u16) { self.0.cpu.set_r8w(value) }
    #[getter] fn get_r9w(&self) -> u16 { self.0.cpu.get_r9w() }
    #[setter] fn set_r9w(&mut self, value: u16) { self.0.cpu.set_r9w(value) }
    #[getter] fn get_r10w(&self) -> u16 { self.0.cpu.get_r10w() }
    #[setter] fn set_r10w(&mut self, value: u16) { self.0.cpu.set_r10w(value) }
    #[getter] fn get_r11w(&self) -> u16 { self.0.cpu.get_r11w() }
    #[setter] fn set_r11w(&mut self, value: u16) { self.0.cpu.set_r11w(value) }
    #[getter] fn get_r12w(&self) -> u16 { self.0.cpu.get_r12w() }
    #[setter] fn set_r12w(&mut self, value: u16) { self.0.cpu.set_r12w(value) }
    #[getter] fn get_r13w(&self) -> u16 { self.0.cpu.get_r13w() }
    #[setter] fn set_r13w(&mut self, value: u16) { self.0.cpu.set_r13w(value) }
    #[getter] fn get_r14w(&self) -> u16 { self.0.cpu.get_r14w() }
    #[setter] fn set_r14w(&mut self, value: u16) { self.0.cpu.set_r14w(value) }
    #[getter] fn get_r15w(&self) -> u16 { self.0.cpu.get_r15w() }
    #[setter] fn set_r15w(&mut self, value: u16) { self.0.cpu.set_r15w(value) }
    #[getter] fn get_axi(&self) -> i16 { self.0.cpu.get_ax() as i16 }
    #[setter] fn set_axi(&mut self, value: i16) { self.0.cpu.set_ax(value as u16) }
    #[getter] fn get_bxi(&self) -> i16 { self.0.cpu.get_bx() as i16 }
    #[setter] fn set_bxi(&mut self, value: i16) { self.0.cpu.set_bx(value as u16) }
    #[getter] fn get_cxi(&self) -> i16 { self.0.cpu.get_cx() as i16 }
    #[setter] fn set_cxi(&mut self, value: i16) { self.0.cpu.set_cx(value as u16) }
    #[getter] fn get_dxi(&self) -> i16 { self.0.cpu.get_dx() as i16 }
    #[setter] fn set_dxi(&mut self, value: i16) { self.0.cpu.set_dx(value as u16) }
    #[getter] fn get_sii(&self) -> i16 { self.0.cpu.get_si() as i16 }
    #[setter] fn set_sii(&mut self, value: i16) { self.0.cpu.set_si(value as u16) }
    #[getter] fn get_dii(&self) -> i16 { self.0.cpu.get_di() as i16 }
    #[setter] fn set_dii(&mut self, value: i16) { self.0.cpu.set_di(value as u16) }
    #[getter] fn get_r8wi(&self) -> i16 { self.0.cpu.get_r8w() as i16 }
    #[setter] fn set_r8wi(&mut self, value: i16) { self.0.cpu.set_r8w(value as u16) }
    #[getter] fn get_r9wi(&self) -> i16 { self.0.cpu.get_r9w() as i16 }
    #[setter] fn set_r9wi(&mut self, value: i16) { self.0.cpu.set_r9w(value as u16) }
    #[getter] fn get_r10wi(&self) -> i16 { self.0.cpu.get_r10w() as i16 }
    #[setter] fn set_r10wi(&mut self, value: i16) { self.0.cpu.set_r10w(value as u16) }
    #[getter] fn get_r11wi(&self) -> i16 { self.0.cpu.get_r11w() as i16 }
    #[setter] fn set_r11wi(&mut self, value: i16) { self.0.cpu.set_r11w(value as u16) }
    #[getter] fn get_r12wi(&self) -> i16 { self.0.cpu.get_r12w() as i16 }
    #[setter] fn set_r12wi(&mut self, value: i16) { self.0.cpu.set_r12w(value as u16) }
    #[getter] fn get_r13wi(&self) -> i16 { self.0.cpu.get_r13w() as i16 }
    #[setter] fn set_r13wi(&mut self, value: i16) { self.0.cpu.set_r13w(value as u16) }
    #[getter] fn get_r14wi(&self) -> i16 { self.0.cpu.get_r14w() as i16 }
    #[setter] fn set_r14wi(&mut self, value: i16) { self.0.cpu.set_r14w(value as u16) }
    #[getter] fn get_r15wi(&self) -> i16 { self.0.cpu.get_r15w() as i16 }
    #[setter] fn set_r15wi(&mut self, value: i16) { self.0.cpu.set_r15w(value as u16) }

    #[getter] fn get_al(&self) -> u8 { self.0.cpu.get_al() }
    #[setter] fn set_al(&mut self, value: u8) { self.0.cpu.set_al(value) }
    #[getter] fn get_bl(&self) -> u8 { self.0.cpu.get_bl() }
    #[setter] fn set_bl(&mut self, value: u8) { self.0.cpu.set_bl(value) }
    #[getter] fn get_cl(&self) -> u8 { self.0.cpu.get_cl() }
    #[setter] fn set_cl(&mut self, value: u8) { self.0.cpu.set_cl(value) }
    #[getter] fn get_dl(&self) -> u8 { self.0.cpu.get_dl() }
    #[setter] fn set_dl(&mut self, value: u8) { self.0.cpu.set_dl(value) }
    #[getter] fn get_sil(&self) -> u8 { self.0.cpu.get_sil() }
    #[setter] fn set_sil(&mut self, value: u8) { self.0.cpu.set_sil(value) }
    #[getter] fn get_dil(&self) -> u8 { self.0.cpu.get_dil() }
    #[setter] fn set_dil(&mut self, value: u8) { self.0.cpu.set_dil(value) }
    #[getter] fn get_r8b(&self) -> u8 { self.0.cpu.get_r8b() }
    #[setter] fn set_r8b(&mut self, value: u8) { self.0.cpu.set_r8b(value) }
    #[getter] fn get_r9b(&self) -> u8 { self.0.cpu.get_r9b() }
    #[setter] fn set_r9b(&mut self, value: u8) { self.0.cpu.set_r9b(value) }
    #[getter] fn get_r10b(&self) -> u8 { self.0.cpu.get_r10b() }
    #[setter] fn set_r10b(&mut self, value: u8) { self.0.cpu.set_r10b(value) }
    #[getter] fn get_r11b(&self) -> u8 { self.0.cpu.get_r11b() }
    #[setter] fn set_r11b(&mut self, value: u8) { self.0.cpu.set_r11b(value) }
    #[getter] fn get_r12b(&self) -> u8 { self.0.cpu.get_r12b() }
    #[setter] fn set_r12b(&mut self, value: u8) { self.0.cpu.set_r12b(value) }
    #[getter] fn get_r13b(&self) -> u8 { self.0.cpu.get_r13b() }
    #[setter] fn set_r13b(&mut self, value: u8) { self.0.cpu.set_r13b(value) }
    #[getter] fn get_r14b(&self) -> u8 { self.0.cpu.get_r14b() }
    #[setter] fn set_r14b(&mut self, value: u8) { self.0.cpu.set_r14b(value) }
    #[getter] fn get_r15b(&self) -> u8 { self.0.cpu.get_r15b() }
    #[setter] fn set_r15b(&mut self, value: u8) { self.0.cpu.set_r15b(value) }
    #[getter] fn get_ali(&self) -> i8 { self.0.cpu.get_al() as i8 }
    #[setter] fn set_ali(&mut self, value: i8) { self.0.cpu.set_al(value as u8) }
    #[getter] fn get_bli(&self) -> i8 { self.0.cpu.get_bl() as i8 }
    #[setter] fn set_bli(&mut self, value: i8) { self.0.cpu.set_bl(value as u8) }
    #[getter] fn get_cli(&self) -> i8 { self.0.cpu.get_cl() as i8 }
    #[setter] fn set_cli(&mut self, value: i8) { self.0.cpu.set_cl(value as u8) }
    #[getter] fn get_dli(&self) -> i8 { self.0.cpu.get_dl() as i8 }
    #[setter] fn set_dli(&mut self, value: i8) { self.0.cpu.set_dl(value as u8) }
    #[getter] fn get_sili(&self) -> i8 { self.0.cpu.get_sil() as i8 }
    #[setter] fn set_sili(&mut self, value: i8) { self.0.cpu.set_sil(value as u8) }
    #[getter] fn get_dili(&self) -> i8 { self.0.cpu.get_dil() as i8 }
    #[setter] fn set_dili(&mut self, value: i8) { self.0.cpu.set_dil(value as u8) }
    #[getter] fn get_r8bi(&self) -> i8 { self.0.cpu.get_r8b() as i8 }
    #[setter] fn set_r8bi(&mut self, value: i8) { self.0.cpu.set_r8b(value as u8) }
    #[getter] fn get_r9bi(&self) -> i8 { self.0.cpu.get_r9b() as i8 }
    #[setter] fn set_r9bi(&mut self, value: i8) { self.0.cpu.set_r9b(value as u8) }
    #[getter] fn get_r10bi(&self) -> i8 { self.0.cpu.get_r10b() as i8 }
    #[setter] fn set_r10bi(&mut self, value: i8) { self.0.cpu.set_r10b(value as u8) }
    #[getter] fn get_r11bi(&self) -> i8 { self.0.cpu.get_r11b() as i8 }
    #[setter] fn set_r11bi(&mut self, value: i8) { self.0.cpu.set_r11b(value as u8) }
    #[getter] fn get_r12bi(&self) -> i8 { self.0.cpu.get_r12b() as i8 }
    #[setter] fn set_r12bi(&mut self, value: i8) { self.0.cpu.set_r12b(value as u8) }
    #[getter] fn get_r13bi(&self) -> i8 { self.0.cpu.get_r13b() as i8 }
    #[setter] fn set_r13bi(&mut self, value: i8) { self.0.cpu.set_r13b(value as u8) }
    #[getter] fn get_r14bi(&self) -> i8 { self.0.cpu.get_r14b() as i8 }
    #[setter] fn set_r14bi(&mut self, value: i8) { self.0.cpu.set_r14b(value as u8) }
    #[getter] fn get_r15bi(&self) -> i8 { self.0.cpu.get_r15b() as i8 }
    #[setter] fn set_r15bi(&mut self, value: i8) { self.0.cpu.set_r15b(value as u8) }

    #[getter] fn get_ah(&self) -> u8 { self.0.cpu.get_ah() }
    #[setter] fn set_ah(&mut self, value: u8) { self.0.cpu.set_ah(value) }
    #[getter] fn get_bh(&self) -> u8 { self.0.cpu.get_bh() }
    #[setter] fn set_bh(&mut self, value: u8) { self.0.cpu.set_bh(value) }
    #[getter] fn get_ch(&self) -> u8 { self.0.cpu.get_ch() }
    #[setter] fn set_ch(&mut self, value: u8) { self.0.cpu.set_ch(value) }
    #[getter] fn get_dh(&self) -> u8 { self.0.cpu.get_dh() }
    #[setter] fn set_dh(&mut self, value: u8) { self.0.cpu.set_dh(value) }
    #[getter] fn get_ahi(&self) -> i8 { self.0.cpu.get_ah() as i8 }
    #[setter] fn set_ahi(&mut self, value: i8) { self.0.cpu.set_ah(value as u8) }
    #[getter] fn get_bhi(&self) -> i8 { self.0.cpu.get_bh() as i8 }
    #[setter] fn set_bhi(&mut self, value: i8) { self.0.cpu.set_bh(value as u8) }
    #[getter] fn get_chi(&self) -> i8 { self.0.cpu.get_ch() as i8 }
    #[setter] fn set_chi(&mut self, value: i8) { self.0.cpu.set_ch(value as u8) }
    #[getter] fn get_dhi(&self) -> i8 { self.0.cpu.get_dh() as i8 }
    #[setter] fn set_dhi(&mut self, value: i8) { self.0.cpu.set_dh(value as u8) }
}

#[pyfunction]
fn assemble(asm_name: &str, src: &str) -> PyResult<ObjectFile> {
    match csx64::asm::assemble(asm_name, &mut src.as_bytes(), Default::default()) {
        Ok(v) => Ok(ObjectFile(v)),
        Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
    }
}
#[pyfunction]
fn link(objs: Vec<(String, Py<ObjectFile>)>, entry_point: Option<(&str, &str)>) -> PyResult<Executable> {
    let objs = Python::with_gil(|py| -> PyResult<Vec<(String, csx64::asm::ObjectFile)>> {
        let mut robjs = Vec::with_capacity(objs.len());
        for obj in objs.into_iter() {
            robjs.push((obj.0, obj.1.extract::<ObjectFile>(py)?.0));
        }
        Ok(robjs)
    })?;
    match csx64::asm::link(objs, entry_point) {
        Ok(v) => Ok(Executable(v)),
        Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
    }
}
#[pyfunction]
fn stdlib() -> Vec<(String, ObjectFile)> {
    csx64::asm::stdlib().into_iter().map(|v| (v.0, ObjectFile(v.1))).collect()
}

#[pymodule]
fn csx64(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ObjectFile>()?;
    m.add_class::<Executable>()?;
    m.add_class::<Emulator>()?;
    m.add_class::<MemoryFile>()?;

    m.add_function(wrap_pyfunction!(assemble, m)?)?;
    m.add_function(wrap_pyfunction!(link, m)?)?;
    m.add_function(wrap_pyfunction!(stdlib, m)?)?;
    Ok(())
}
