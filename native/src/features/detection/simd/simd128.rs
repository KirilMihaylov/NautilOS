//! Provides interface over platform's 128-bit SIMD features.

use crate::{
    features::detection::{detection_mechanism_available, FeatureState},
    result::{
        Error::{self, FeatureDisabled},
        Result,
    },
};

/// Checks whether the minimal 128-bit SIMD instructions are supported.
///
/// Returns `Err` with [`FeatureDisabled`] when feature detection mechanism is required but is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when it returns an error.
pub fn min_available() -> Result<FeatureState> {
    use Error::*;
    use FeatureState::*;

    target_arch_else_unimplemented_error! {
        ["x86"] {
            /* SSE */

            match detection_mechanism_available() {
                Ok(Enabled) => {
                    /*
                    SSE: CPUID[1].D[25]
                    */

                    #[cfg(not(feature="kernel_mode"))]
                    let c: u32;

                    let d: u32;

                    unsafe {
                        #[cfg(not(feature="kernel_mode"))]
                        asm!(
                            "cpuid",
                            inlateout("eax") 1 => _,
                            lateout("ebx") _,
                            lateout("ecx") c,
                            lateout("edx") d,
                            options(nomem, nostack)
                        );

                        #[cfg(feature="kernel_mode")]
                        asm!(
                            "cpuid",
                            inlateout("eax") 1 => _,
                            lateout("ebx") _,
                            lateout("ecx") _,
                            lateout("edx") d,
                            options(nomem, nostack)
                        );
                    }

                    if d >> 25 & 1 == 1 {
                        #[cfg(not(feature="kernel_mode"))]
                        {
                            /*
                            OSXSAVE: C[27]
                            FPU/MMX: XCR0[0]
                            SSE: XCR0[1]
                            */

                            if c >> 27 & 1 == 1 {
                                let result: u32;

                                unsafe {
                                    asm!(
                                        "xgetbv",
                                        lateout("eax") result,
                                        in("ecx") 0,
                                        lateout("edx") _,
                                        options(nomem, nostack)
                                    );
                                }

                                if result & 3 == 3 {
                                    return Ok(Enabled);
                                }
                            }
                            Err(OsInteractionRequired)
                        }

                        #[cfg(feature="kernel_mode")]
                        {
                            let (cr0, cr4): (usize, usize);

                            unsafe {
                                asm!(
                                    "mov {cr0}, cr0
                                    mov {cr4}, cr4",
                                    cr0 = lateout(reg) cr0,
                                    cr4 = lateout(reg) cr4,
                                    options(nomem, nostack)
                                );
                            }

                            Ok(if cr0 & 6 == 6 && cr4 >> 9 & 3 == 3 { Enabled } else { Disabled })
                        }
                    } else { Err(Unavailable) }
                },
                Ok(Disabled) => Err(FeatureDisabled),
                error => error,
            }
        },
        ["x86_64"] {
            /* SSE */

            #[cfg(not(feature="kernel_mode"))]
            {
                /*
                OSXSAVE: C[27]
                FPU/MMX: XCR0[0]
                SSE: XCR0[1]
                */

                let c: u32;

                unsafe {
                    asm!(
                        "cpuid",
                        inlateout("eax") 1 => _,
                        lateout("ebx") _,
                        lateout("ecx") c,
                        lateout("edx") _,
                        options(nomem, nostack)
                    );
                }

                if c >> 27 & 1 == 1 {
                    let result: u32;

                    unsafe {
                        asm!(
                            "xgetbv",
                            lateout("eax") result,
                            in("ecx") 0,
                            lateout("edx") _,
                            options(nomem, nostack)
                        );
                    }

                    if result & 3 == 3 {
                        return Ok(Enabled);
                    }
                }
                Err(OsInteractionRequired)
            }

            #[cfg(feature="kernel_mode")]
            {
                let (cr0, cr4): (usize, usize);

                unsafe {
                    asm!(
                        "mov {cr0}, cr0
                        mov {cr4}, cr4",
                        cr0 = lateout(reg) cr0,
                        cr4 = lateout(reg) cr4,
                        options(nomem, nostack)
                    );
                }

                if cr0 & 6 == 6 && cr4 >> 9 & 3 == 3 {
                    Ok(Enabled)
                } else {
                    Ok(Disabled)
                }
            }
        }
    }
}

/// Checks whether the recommended 128-bit SIMD instructions are supported.
///
/// Returns `Err` with [`FeatureDisabled`] when feature detection mechanism is required but is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when it returns an error.
pub fn available() -> Result<FeatureState> {
    use Error::*;
    use FeatureState::*;

    target_arch_else_unimplemented_error! {
        ["x86"] {
            /* SSE2 */

            match detection_mechanism_available() {
                Ok(Enabled) => {
                    /*
                    SSE: CPUID[1].D[25]
                    SSE2: CPUID[1].D[26]
                    */

                    #[cfg(not(feature="kernel_mode"))]
                    let c: u32;

                    let d: u32;

                    unsafe {
                        #[cfg(not(feature="kernel_mode"))]
                        asm!(
                            "cpuid",
                            inlateout("eax") 1 => _,
                            lateout("ebx") _,
                            lateout("ecx") c,
                            lateout("edx") d,
                            options(nomem, nostack)
                        );

                        #[cfg(feature="kernel_mode")]
                        asm!(
                            "cpuid",
                            inlateout("eax") 1 => _,
                            lateout("ebx") _,
                            lateout("ecx") _,
                            lateout("edx") d,
                            options(nomem, nostack)
                        );
                    }

                    if d >> 25 & 3 == 3 {
                        #[cfg(not(feature="kernel_mode"))]
                        {
                            /*
                            OSXSAVE: C[27]
                            FPU/MMX: XCR0[0]
                            SSE: XCR0[1]
                            */

                            if c >> 27 & 1 == 1 {
                                let result: u32;

                                unsafe {
                                    asm!(
                                        "xgetbv",
                                        lateout("eax") result,
                                        in("ecx") 0,
                                        lateout("edx") _,
                                        options(nomem, nostack)
                                    );
                                }

                                if result & 3 == 3 {
                                    return Ok(Enabled);
                                }
                            }
                            Err(OsInteractionRequired)
                        }

                        #[cfg(feature="kernel_mode")]
                        {
                            let (cr0, cr4): (usize, usize);

                            unsafe {
                                asm!(
                                    "mov {cr0}, cr0
                                    mov {cr4}, cr4",
                                    cr0 = lateout(reg) cr0,
                                    cr4 = lateout(reg) cr4,
                                    options(nomem, nostack)
                                );
                            }

                            Ok(if cr0 & 6 == 6 && cr4 >> 9 & 3 == 3 { Enabled } else { Disabled })
                        }
                    } else { Err(Unavailable) }
                },
                Ok(Disabled) => Err(FeatureDisabled),
                error => error,
            }
        },
        ["x86_64"] {
            /* SSE2 */

            #[cfg(not(feature="kernel_mode"))]
            {
                /*
                OSXSAVE: CPUID.C[27]
                FPU/MMX: XCR0[0]
                SSE/SSE2: XCR0[1]
                */

                let c: u32;

                unsafe {
                    asm!(
                        "cpuid",
                        inlateout("eax") 1 => _,
                        lateout("ebx") _,
                        lateout("ecx") c,
                        lateout("edx") _,
                        options(nomem, nostack)
                    );
                }

                if c >> 27 & 1 == 1 {
                    let result: u32;

                    unsafe {
                        asm!(
                            "xgetbv",
                            lateout("eax") result,
                            in("ecx") 0,
                            lateout("edx") _,
                            options(nomem, nostack)
                        );
                    }

                    if result & 3 == 3 {
                        return Ok(Enabled);
                    }
                }
                
                Err(OsInteractionRequired)
            }

            #[cfg(feature="kernel_mode")]
            {
                let (cr0, cr4): (usize, usize);

                unsafe {
                    asm!(
                        "mov {cr0}, cr0
                        mov {cr4}, cr4",
                        cr0 = lateout(reg) cr0,
                        cr4 = lateout(reg) cr4,
                        options(nomem, nostack)
                    );
                }

                if cr0 & 6 == 6 && cr4 >> 9 & 3 == 3 {
                    Ok(Enabled)
                } else {
                    Ok(Disabled)
                }
            }
        }
    }
}

/// Checks whether the maximal 128-bit SIMD instructions are supported.
///
/// Returns `Err` with [`FeatureDisabled`] when feature detection mechanism is required but is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when it returns an error.
pub fn max_available() -> Result<FeatureState> {
    use Error::*;
    use FeatureState::*;

    target_arch_else_unimplemented_error! {
        ["x86"] {
            /* SSE4.2 */

            match detection_mechanism_available() {
                Ok(Enabled) => {
                    /*
                    SSE: CPUID[1].D[25]
                    SSE2: CPUID[1].D[26]
                    SSE3: CPUID[1].C[0]
                    SSSE3: CPUID[1].C[9]
                    SSE4.1: CPUID[1].C[19]
                    SSE4.2: CPUID[1].C[20]
                    */

                    let (c, d): (u32, u32);

                    unsafe {
                        asm!(
                            "cpuid",
                            inlateout("eax") 1 => _,
                            lateout("ebx") _,
                            lateout("ecx") c,
                            lateout("edx") d,
                            options(nomem, nostack)
                        );
                    }

                    if c & 0x180201 == 0x180201 && d >> 25 & 3 == 3 {
                        #[cfg(not(feature="kernel_mode"))]
                        {
                            /*
                            OSXSAVE: C[27]
                            FPU/MMX: XCR0[0]
                            SSE: XCR0[1]
                            */

                            if c >> 27 & 1 == 1 {
                                let result: u32;

                                unsafe {
                                    asm!(
                                        "xgetbv",
                                        lateout("eax") result,
                                        in("ecx") 0,
                                        lateout("edx") _,
                                        options(nomem, nostack)
                                    );
                                }

                                if result & 3 == 3 {
                                    return Ok(Enabled);
                                }
                            }
                            Err(OsInteractionRequired)
                        }

                        #[cfg(feature="kernel_mode")]
                        {
                            let (cr0, cr4): (usize, usize);

                            unsafe {
                                asm!(
                                    "mov {cr0}, cr0
                                    mov {cr4}, cr4",
                                    cr0 = lateout(reg) cr0,
                                    cr4 = lateout(reg) cr4,
                                    options(nomem, nostack)
                                );
                            }

                            Ok(if cr0 & 6 == 6 && cr4 >> 9 & 3 == 3 { Enabled } else { Disabled })
                        }
                    } else { Err(Unavailable) }
                },
                Ok(Disabled) => Err(FeatureDisabled),
                error => error,
            }
        },
        ["x86_64"] {
            /* SSE4.2 */

            /*
            SSE: CPUID[1].D[25]
            SSE2: CPUID[1].D[26]
            SSE3: CPUID[1].C[0]
            SSSE3: CPUID[1].C[9]
            SSE4.1: CPUID[1].C[19]
            SSE4.2: CPUID[1].C[20]
            */

            let (c, d): (u32, u32);

            unsafe {
                asm!(
                    "cpuid",
                    inlateout("eax") 1 => _,
                    lateout("ebx") _,
                    lateout("ecx") c,
                    lateout("edx") d,
                    options(nomem, nostack)
                );
            }

            if c & 0x180201 == 0x180201 && d >> 25 & 3 == 3 {
                #[cfg(not(feature="kernel_mode"))]
                {
                    /*
                    OSXSAVE: C[27]
                    FPU/MMX: XCR0[0]
                    SSE: XCR0[1]
                    */

                    if c >> 27 & 1 == 1 {
                        let result: u32;

                        unsafe {
                            asm!(
                                "xgetbv",
                                lateout("eax") result,
                                in("ecx") 0,
                                lateout("edx") _,
                                options(nomem, nostack)
                            );
                        }

                        if result & 3 == 3 {
                            return Ok(Enabled);
                        }
                    }
                    Err(OsInteractionRequired)
                }

                #[cfg(feature="kernel_mode")]
                {
                    let (cr0, cr4): (usize, usize);

                    unsafe {
                        asm!(
                            "mov {cr0}, cr0
                            mov {cr4}, cr4",
                            cr0 = lateout(reg) cr0,
                            cr4 = lateout(reg) cr4,
                            options(nomem, nostack)
                        );
                    }

                    Ok(if cr0 & 6 == 6 && cr4 >> 9 & 3 == 3 { Enabled } else { Disabled })
                }
            } else { Err(Unavailable) }
        }
    }
}
