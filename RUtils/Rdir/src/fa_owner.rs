// fa_owner.rs - File analysis for ownership
//
// 2025-10-28   PV      First version

use std::ffi::OsStr;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use windows::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, ERROR_NONE_MAPPED, GetLastError, HLOCAL, LocalFree, WIN32_ERROR};
use windows::Win32::Security::Authorization::{ConvertSidToStringSidW, GetNamedSecurityInfoW, SE_FILE_OBJECT};
use windows::Win32::Security::LookupAccountSidW;
use windows::Win32::Security::OWNER_SECURITY_INFORMATION;
use windows::Win32::Security::PSECURITY_DESCRIPTOR;
use windows::Win32::Security::PSID;
use windows::Win32::Security::SID_NAME_USE;
use windows::core::{PCWSTR, PWSTR};

use crate::Options;

#[derive(Debug)]
#[allow(unused)]
pub struct OwnerInfo {
    pub sid_string: String,
    pub mapped_owner: Option<String>,
}

pub fn get_owner_information(path: &Path, _options: &Options) -> core::result::Result<OwnerInfo, String> {
    match get_file_owner(path) {
        Ok((sid_string, mapped_owner)) => Ok(OwnerInfo { sid_string, mapped_owner }),
        Err(e) => Err(format!("{}", e)),
    }
}

/// Gets the owner of a file at a given path, and whether this is
///
/// Returns the owner in `DOMAIN\User` format.
/// If the name cannot be resolved, it returns the owner's SID string
/// (e.g., "S-1-5-21-...").
///
/// # Errors
///
/// Returns an `io::Error` if any Windows API call fails.
pub fn get_file_owner(path: &Path) -> io::Result<(String, Option<String>)> {
    // 1. Convert the Rust path to a null-terminated wide string (UTF-16)
    //    that the Windows API expects.
    let path_wide: Vec<u16> = OsStr::new(path).encode_wide().chain(std::iter::once(0)).collect();

    let mut psid_owner: PSID = PSID::default();
    let mut p_security_descriptor: PSECURITY_DESCRIPTOR = PSECURITY_DESCRIPTOR::default();

    // 2. Call GetNamedSecurityInfoW to get the security descriptor and owner SID.
    //    We request only the OWNER_SECURITY_INFORMATION.
    let error_code: WIN32_ERROR = unsafe {
        GetNamedSecurityInfoW(
            PCWSTR(path_wide.as_ptr()),
            SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION,
            Some(&mut psid_owner),
            None,
            None,
            None,
            &mut p_security_descriptor,
        )
    };

    if error_code.0 != 0 {
        return Err(io::Error::from_raw_os_error(error_code.0 as i32));
    }

    // Ensure the security descriptor is freed when we're done.
    // This is a RAII guard to prevent memory leaks.
    let _guard = SecurityDescriptorGuard(p_security_descriptor);

    if psid_owner.is_invalid() {
        return Err(io::Error::other("GetNamedSecurityInfoW returned an invalid SID"));
    }

    // 3. Call LookupAccountSidW to resolve the SID to a name.
    //    This is a two-call process:
    //    - First call (with 0-length buffers) gets the required buffer sizes.
    //    - Second call (with allocated buffers) gets the names.

    let mut name_size: u32 = 0;
    let mut domain_size: u32 = 0;
    let mut sid_use: SID_NAME_USE = SID_NAME_USE(0);

    let _ = unsafe {
        LookupAccountSidW(
            PCWSTR::null(), // Local computer
            psid_owner,
            None,
            &mut name_size,
            None,
            &mut domain_size,
            &mut sid_use,
        )
    };

    match unsafe { GetLastError() } {
        // This error is expected because our buffers are 0-sized.
        ERROR_INSUFFICIENT_BUFFER => {
            let mut name_buf: Vec<u16> = vec![0; name_size as usize];
            let mut domain_buf: Vec<u16> = vec![0; domain_size as usize];

            let success = unsafe {
                LookupAccountSidW(
                    PCWSTR::null(),
                    psid_owner,
                    Some(PWSTR(name_buf.as_mut_ptr())),
                    &mut name_size,
                    Some(PWSTR(domain_buf.as_mut_ptr())),
                    &mut domain_size,
                    &mut sid_use,
                )
            };

            if success.is_err() {
                // Name resolution failed. This can happen for SIDs that
                // don't map to an account (e.g., deleted user).
                // Fall back to returning the SID string.
                if unsafe { GetLastError() } == ERROR_NONE_MAPPED {
                    let sid_string = get_sid_string(psid_owner)?;
                    return Ok((sid_string, None));
                } else {
                    return Err(io::Error::last_os_error());
                }
            }

            // Successfully got the names. Combine them.
            let name = String::from_utf16_lossy(&name_buf[..name_size as usize]);
            let domain = String::from_utf16_lossy(&domain_buf[..domain_size as usize]);

            let sid_string = get_sid_string(psid_owner)?;

            // For local accounts (like "SYSTEM"), the domain is "NT AUTHORITY"
            // For built-in accounts, the domain is "BUILTIN"
            // For user accounts, it's the computer or domain name.
            if domain.is_empty() {
                Ok((sid_string, Some(name)))
            } else {
                Ok((sid_string, Some(format!("{}\\{}", domain, name))))
            }
        }

        // This can happen for well-known SIDs that don't have a name.
        ERROR_NONE_MAPPED => {
            let sid_string = get_sid_string(psid_owner)?;
            Ok((sid_string, None))
        }
        // Any other error.
        err => Err(io::Error::from_raw_os_error(err.0 as i32)),
    }
}

/// Fallback function to convert a PSID to its string representation.
fn get_sid_string(psid: PSID) -> io::Result<String> {
    let mut sid_string_ptr: PWSTR = PWSTR::null();
    let success = unsafe { ConvertSidToStringSidW(psid, &mut sid_string_ptr) };

    if success.is_err() {
        return Err(io::Error::last_os_error());
    }

    // Ensure the buffer allocated by the API is freed.
    let _guard = LocalFreeGuard(sid_string_ptr.0 as _);

    let sid_string = unsafe {
        // Find the null terminator to determine the string length
        let mut len = 0;
        while *sid_string_ptr.0.add(len) != 0 {
            len += 1;
        }
        let slice = std::slice::from_raw_parts(sid_string_ptr.0, len);
        String::from_utf16_lossy(slice)
    };

    Ok(sid_string)
}

// --- Helper structs for RAII (to auto-free memory) ---

/// RAII guard to call LocalFree on PSECURITY_DESCRIPTOR.
struct SecurityDescriptorGuard(PSECURITY_DESCRIPTOR);

impl Drop for SecurityDescriptorGuard {
    fn drop(&mut self) {
        if !self.0.0.is_null() {
            unsafe { LocalFree(Some(HLOCAL(self.0.0))) };
        }
    }
}

/// RAII guard to call LocalFree on a generic handle.
struct LocalFreeGuard(isize);

impl Drop for LocalFreeGuard {
    fn drop(&mut self) {
        if self.0 != 0 {
            let zer = Some(HLOCAL(self.0 as *mut _));
            unsafe { LocalFree(zer) };
        }
    }
}
