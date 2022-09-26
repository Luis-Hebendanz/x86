//! Provides functions to read and write segment registers.

use crate::structures::gdt::SegmentSelector;
use core::arch::asm;


/// Reload code segment register.
///
/// Note this is special since we can not directly move
/// to cs. Instead we push the new segment selector
/// and return value on the stack and use retf
/// to reload cs and continue at 1:.
///
/// ## Safety
///
/// This function is unsafe because the caller must ensure that `sel`
/// is a valid code segment descriptor.
#[inline]
pub unsafe fn set_cs(sel: SegmentSelector) {

    #[inline(always)]
    unsafe fn inner(sel: SegmentSelector) {
        asm!(
            "push {sel}",
            "lea {tmp}, 1f",
            "push {tmp}",
            "retf",
            "1:",
            sel = in(reg) u32::from(sel.0),
            tmp = lateout(reg) _,
        );
    }


    inner(sel)
}

/// Reload stack segment register.
///
/// ## Safety
///
/// This function is unsafe because the caller must ensure that `sel`
/// is a valid stack segment descriptor.
#[inline]
pub unsafe fn load_ss(sel: SegmentSelector) {

    asm!("mov ss, {0:x}", in(reg) sel.0, options(nostack));

}

/// Reload data segment register.
///
/// ## Safety
///
/// This function is unsafe because the caller must ensure that `sel`
/// is a valid data segment descriptor.
#[inline]
pub unsafe fn load_ds(sel: SegmentSelector) {

    asm!("mov ds, {0:x}", in(reg) sel.0, options(nostack));

}

/// Reload es segment register.
///
/// ## Safety
///
/// This function is unsafe because the caller must ensure that `sel`
/// is a valid extra segment descriptor.
#[inline]
pub unsafe fn load_es(sel: SegmentSelector) {

    asm!("mov es, {0:x}", in(reg) sel.0, options(nostack));

}

/// Reload fs segment register.
///
/// ## Safety
///
/// This function is unsafe because the caller must ensure that `sel`
/// is a valid fs segment descriptor.
#[inline]
pub unsafe fn load_fs(sel: SegmentSelector) {

    asm!("mov fs, {0:x}", in(reg) sel.0, options(nostack));
}

/// Reload gs segment register.
///
/// ## Safety
///
/// This function is unsafe because the caller must ensure that `sel`
/// is a valid gs segment descriptor.
#[inline]
pub unsafe fn load_gs(sel: SegmentSelector) {

    asm!("mov gs, {0:x}", in(reg) sel.0, options(nostack));
}

/// Swap `KernelGsBase` MSR and `GsBase` MSR.
///
/// ## Safety
///
/// This function is unsafe because the caller must ensure that the
/// swap operation cannot lead to undefined behavior.
#[inline]
pub unsafe fn swap_gs() {

    asm!("swapgs", options(nostack));
}

/// Returns the current value of the code segment register.
#[inline]
pub fn cs() -> SegmentSelector {
    let segment: u16;
    unsafe { asm!("mov {0:x}, cs", out(reg) segment, options(nostack, nomem)) };
    SegmentSelector(segment)
}

