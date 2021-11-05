extern crate llvm_sys;

use llvm_sys::disassembler::*;
use llvm_sys::target::*;
use std::ffi::CStr;
use std::io::{stdin, stdout, Read, Result as IoResult, Write};
use std::ptr;

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
}

fn main() -> IoResult<()> {
    unsafe {
        LLVM_InitializeAllAsmPrinters();
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllDisassemblers();
    }
    let disasm = unsafe {
        LLVMCreateDisasmCPUFeatures(
            c_str!("aarch64"),
            c_str!("generic"),
            // c_str!("+v8.3a"),
            c_str!("+v8.5a+memtag"),
            ptr::null_mut(),
            0,
            None,
            None,
        )
    };
    if disasm.is_null() {
        eprintln!("Failed to create disassembler");
        return Ok(());
    }

    unsafe {
        // LLVMSetDisasmOptions(disasm, 1 as u64);
        LLVMSetDisasmOptions(disasm, 2 as u64);
        // LLVMSetDisasmOptions(disasm, 4 as u64);
    }

    let mut data = Vec::<u8>::new();
    stdin().read_to_end(&mut data)?;
    let r = disassemble_bytes(&mut data, disasm, stdout());

    unsafe {
        LLVMDisasmDispose(disasm);
    }

    r
}

const PC_BASE_ADDR: u64 = 0x100007e58;

fn disassemble_bytes<W: Write>(
    mut x: &mut [u8],
    disasm: LLVMDisasmContextRef,
    mut out: W,
) -> IoResult<()> {
    let mut pc = PC_BASE_ADDR;

    loop {
        let mut sbuf = [0i8; 255];
        let sz = unsafe {
            LLVMDisasmInstruction(
                disasm,
                x.as_mut_ptr(),
                x.len() as u64,
                pc as u64,
                sbuf.as_mut_ptr() as *mut i8,
                sbuf.len(),
            )
        };
        // if sz == 0 && x.len() < 4 {
        if sz == 0 {
            break;
        }

        let instr_str = unsafe { CStr::from_ptr(sbuf.as_ptr()) };
        // write!(out, "{:#x?}: {}\n", pc, instr_str.to_string_lossy())?;
        println!("{:#x?}: {}", pc as u64, instr_str.to_string_lossy());

        // pc += 4 as u64;
        pc += sz as u64;
        // x = &mut x[4..];
        x = &mut x[sz..];
    }

    Ok(())
}
