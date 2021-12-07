use crate::{
    Error,
    Errno,
    NixPath,
    Result,
    sys::uio::IoVec
};
use libc::{c_char, c_int, c_uint, c_void};
use std::{
    borrow::Cow,
    ffi::{CString, CStr},
    fmt,
    io,
    ptr
};


libc_bitflags!(
    /// Used with [`Nmount::nmount`].
    pub struct MntFlags: c_int {
        /// ACL support enabled.
        #[cfg(any(target_os = "netbsd", target_os = "freebsd"))]
        MNT_ACLS;
        /// All I/O to the file system should be done asynchronously.
        MNT_ASYNC;
        /// dir should instead be a file system ID encoded as “FSID:val0:val1”.
        #[cfg(target_os = "freebsd")]
        MNT_BYFSID;
        /// Force a read-write mount even if the file system appears to be
        /// unclean.
        MNT_FORCE;
        /// GEOM journal support enabled.
        #[cfg(target_os = "freebsd")]
        MNT_GJOURNAL;
        /// MAC support for objects.
        #[cfg(any(target_os = "macos", target_os = "freebsd"))]
        MNT_MULTILABEL;
        /// Disable read clustering.
        #[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
        MNT_NOCLUSTERR;
        /// Disable write clustering.
        #[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
        MNT_NOCLUSTERW;
        /// Enable NFS version 4 ACLs.
        #[cfg(target_os = "freebsd")]
        MNT_NFS4ACLS;
        /// Do not update access times.
        MNT_NOATIME;
        /// Disallow program execution.
        MNT_NOEXEC;
        /// Do not honor setuid or setgid bits on files when executing them.
        MNT_NOSUID;
        /// Do not follow symlinks.
        #[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
        MNT_NOSYMFOLLOW;
        /// Mount read-only.
        MNT_RDONLY;
        /// Causes the vfs subsystem to update its data structures pertaining to
        /// the specified already mounted file system.
        MNT_RELOAD;
        /// Create a snapshot of the file system.
        ///
        /// See [mksnap_ffs(8)](https://www.freebsd.org/cgi/man.cgi?query=mksnap_ffs)
        #[cfg(any(target_os = "macos", target_os = "freebsd"))]
        MNT_SNAPSHOT;
        /// Using soft updates.
        #[cfg(any(
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
        ))]
        MNT_SOFTDEP;
        /// Directories with the SUID bit set chown new files to their own
        /// owner.
        #[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
        MNT_SUIDDIR;
        /// All I/O to the file system should be done synchronously.
        MNT_SYNCHRONOUS;
        /// Union with underlying fs.
        #[cfg(any(
                target_os = "macos",
                target_os = "freebsd",
                target_os = "netbsd"
        ))]
        MNT_UNION;
        /// Indicates that the mount command is being applied to an already
        /// mounted file system.
        MNT_UPDATE;
        /// Check vnode use counts.
        #[cfg(target_os = "freebsd")]
        MNT_NONBUSY;
    }
);


/// The Error type of [`Nmount::nmount`].
///
/// It wraps an [`Errno`], but also may contain an additional message returned
/// by `nmount(2)`.
#[derive(Debug)]
pub struct NmountError {
    errno: Error,
    errmsg: Option<String>
}

impl NmountError {
    /// Returns the additional error string sometimes generated by `nmount(2)`.
    pub fn errmsg(&self) -> Option<&str> {
        self.errmsg.as_deref()
    }

    /// Returns the inner [`Error`]
    pub const fn error(&self) -> Error {
        self.errno
    }

    fn new(error: Error, errmsg: Option<&CStr>) -> Self {
        Self {
            errno: error,
            errmsg: errmsg.map(CStr::to_string_lossy).map(Cow::into_owned)
        }
    }
}

impl std::error::Error for NmountError {}

impl fmt::Display for NmountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(errmsg) = &self.errmsg {
            write!(f, "{:?}: {}: {}", self.errno, errmsg, self.errno.desc())
        } else {
            write!(f, "{:?}: {}", self.errno, self.errno.desc())
        }
    }
}

impl From<NmountError> for io::Error {
    fn from(err: NmountError) -> Self {
        err.errno.into()
    }
}

/// Result type of [`Nmount::nmount`].
pub type NmountResult = std::result::Result<(), NmountError>;

/// Mount a FreeBSD file system.
///
/// The `nmount(2)` system call works similarly to the `mount(8)` program; it
/// takes its options as a series of name-value pairs.  Most of the values are
/// strings, as are all of the names.  The `Nmount` structure builds up an
/// argument list and then executes the syscall.
///
/// # Examples
///
/// To mount `target` onto `mountpoint` with `nullfs`:
/// ```
/// # use nix::unistd::Uid;
/// # use ::sysctl::CtlValue;
/// # if !Uid::current().is_root() && CtlValue::Int(0) == ::sysctl::value("vfs.usermount").unwrap() {
/// #     return;
/// # };
/// use nix::mount::{MntFlags, Nmount, unmount};
/// use std::ffi::CString;
/// use tempfile::tempdir;
///
/// let mountpoint = tempdir().unwrap();
/// let target = tempdir().unwrap();
///
/// let fstype = CString::new("fstype").unwrap();
/// let nullfs = CString::new("nullfs").unwrap();
/// Nmount::new()
///     .str_opt(&fstype, &nullfs)
///     .str_opt_owned("fspath", mountpoint.path().to_str().unwrap())
///     .str_opt_owned("target", target.path().to_str().unwrap())
///     .nmount(MntFlags::empty()).unwrap();
/// 
/// unmount(mountpoint.path(), MntFlags::empty()).unwrap();
/// ```
///
/// # See Also
/// * [`nmount(2)`](https://www.freebsd.org/cgi/man.cgi?query=nmount)
/// * [`nullfs(5)`](https://www.freebsd.org/cgi/man.cgi?query=nullfs)
#[cfg(target_os = "freebsd")]
#[derive(Debug, Default)]
pub struct Nmount<'a>{
    iov: Vec<IoVec<&'a [u8]>>,
    is_owned: Vec<bool>,
}

#[cfg(target_os = "freebsd")]
impl<'a> Nmount<'a> {
    /// Add an opaque mount option.
    ///
    /// Some file systems take binary-valued mount options.  They can be set
    /// with this method.
    ///
    /// # Safety
    ///
    /// Unsafe because it will cause `Nmount::nmount` to dereference a raw
    /// pointer.  The user is responsible for ensuring that `val` is valid and
    /// its lifetime outlives `self`!  An easy way to do that is to give the
    /// value a larger scope than `name`
    ///
    /// # Examples
    /// ```
    /// use libc::c_void;
    /// use nix::mount::Nmount;
    /// use std::ffi::CString;
    /// use std::mem;
    ///
    /// // Note that flags outlives name
    /// let mut flags: u32 = 0xdeadbeef;
    /// let name = CString::new("flags").unwrap();
    /// let p = &mut flags as *mut u32 as *mut c_void;
    /// let len = mem::size_of_val(&flags);
    /// let mut nmount = Nmount::new();
    /// unsafe { nmount.mut_ptr_opt(&name, p, len) };
    /// ```
    pub unsafe fn mut_ptr_opt(
        &mut self,
        name: &'a CStr,
        val: *mut c_void,
        len: usize
    ) -> &mut Self
    {
        self.iov.push(IoVec::from_slice(name.to_bytes_with_nul()));
        self.is_owned.push(false);
        self.iov.push(IoVec::from_raw_parts(val, len));
        self.is_owned.push(false);
        self
    }

    /// Add a mount option that does not take a value.
    ///
    /// # Examples
    /// ```
    /// use nix::mount::Nmount;
    /// use std::ffi::CString;
    ///
    /// let read_only = CString::new("ro").unwrap();
    /// Nmount::new()
    ///     .null_opt(&read_only);
    /// ```
    pub fn null_opt(&mut self, name: &'a CStr) -> &mut Self {
        self.iov.push(IoVec::from_slice(name.to_bytes_with_nul()));
        self.is_owned.push(false);
        self.iov.push(IoVec::from_raw_parts(ptr::null_mut(), 0));
        self.is_owned.push(false);
        self
    }

    /// Add a mount option that does not take a value, but whose name must be
    /// owned.
    ///
    ///
    /// This has higher runtime cost than [`Nmount::null_opt`], but is useful
    /// when the name's lifetime doesn't outlive the `Nmount`, or it's a
    /// different string type than `CStr`.
    ///
    /// # Examples
    /// ```
    /// use nix::mount::Nmount;
    ///
    /// let read_only = "ro";
    /// let mut nmount: Nmount<'static> = Nmount::new();
    /// nmount.null_opt_owned(read_only);
    /// ```
    pub fn null_opt_owned<P: ?Sized + NixPath>(&mut self, name: &P) -> &mut Self
    {
        name.with_nix_path(|s| {
            let len = s.to_bytes_with_nul().len();
            self.iov.push(IoVec::from_raw_parts(
                // Must free it later
                s.to_owned().into_raw() as *mut c_void,
                len
            ));
            self.is_owned.push(true);
        }).unwrap();
        self.iov.push(IoVec::from_raw_parts(ptr::null_mut(), 0));
        self.is_owned.push(false);
        self
    }

    /// Add a mount option as a [`CStr`].
    ///
    /// # Examples
    /// ```
    /// use nix::mount::Nmount;
    /// use std::ffi::CString;
    ///
    /// let fstype = CString::new("fstype").unwrap();
    /// let nullfs = CString::new("nullfs").unwrap();
    /// Nmount::new()
    ///     .str_opt(&fstype, &nullfs);
    /// ```
    pub fn str_opt(
        &mut self,
        name: &'a CStr,
        val: &'a CStr
    ) -> &mut Self
    {
        self.iov.push(IoVec::from_slice(name.to_bytes_with_nul()));
        self.is_owned.push(false);
        self.iov.push(IoVec::from_slice(val.to_bytes_with_nul()));
        self.is_owned.push(false);
        self
    }

    /// Add a mount option as an owned string.
    ///
    /// This has higher runtime cost than [`Nmount::str_opt`], but is useful
    /// when the value's lifetime doesn't outlive the `Nmount`, or it's a
    /// different string type than `CStr`.
    ///
    /// # Examples
    /// ```
    /// use nix::mount::Nmount;
    /// use std::path::Path;
    ///
    /// let mountpoint = Path::new("/mnt");
    /// Nmount::new()
    ///     .str_opt_owned("fspath", mountpoint.to_str().unwrap());
    /// ```
    pub fn str_opt_owned<P1, P2>(&mut self, name: &P1, val: &P2) -> &mut Self
        where P1: ?Sized + NixPath,
              P2: ?Sized + NixPath
    {
        name.with_nix_path(|s| {
            let len = s.to_bytes_with_nul().len();
            self.iov.push(IoVec::from_raw_parts(
                // Must free it later
                s.to_owned().into_raw() as *mut c_void,
                len
            ));
            self.is_owned.push(true);
        }).unwrap();
        val.with_nix_path(|s| {
            let len = s.to_bytes_with_nul().len();
            self.iov.push(IoVec::from_raw_parts(
                // Must free it later
                s.to_owned().into_raw() as *mut c_void,
                len
            ));
            self.is_owned.push(true);
        }).unwrap();
        self
    }

    /// Create a new `Nmount` struct with no options
    pub fn new() -> Self {
        Self::default()
    }

    /// Actually mount the file system.
    pub fn nmount(&mut self, flags: MntFlags) -> NmountResult {
        // nmount can return extra error information via a "errmsg" return
        // argument.
        const ERRMSG_NAME: &[u8] = b"errmsg\0";
        let mut errmsg = vec![0u8; 255];
        self.iov.push(IoVec::from_raw_parts(
                ERRMSG_NAME.as_ptr() as *mut c_void,
                ERRMSG_NAME.len()
        ));
        self.iov.push(IoVec::from_raw_parts(
                errmsg.as_mut_ptr() as *mut c_void,
                errmsg.len()
        ));

        let niov = self.iov.len() as c_uint;
        let iovp = self.iov.as_mut_ptr() as *mut libc::iovec;
        let res = unsafe {
            libc::nmount(iovp, niov, flags.bits)
        };
        match Errno::result(res) {
            Ok(_) => Ok(()),
            Err(error) => {
                let errmsg = match errmsg.iter().position(|&x| x == 0) {
                    None => None,
                    Some(0) => None,
                    Some(n) => {
                        let sl = &errmsg[0..n + 1];
                        Some(CStr::from_bytes_with_nul(sl).unwrap())
                    }
                };
                Err(NmountError::new(error, errmsg))
            }
        }
    }
}

#[cfg(target_os = "freebsd")]
impl<'a> Drop for Nmount<'a> {
    fn drop(&mut self) {
        for (iov, is_owned) in self.iov.iter().zip(self.is_owned.iter()) {
            if *is_owned {
                // Free the owned string.  Safe because we recorded ownership,
                // and Nmount does not implement Clone.
                unsafe {
                    CString::from_raw(iov.0.iov_base as *mut c_char);
                }
            }
        }
    }
}

/// Unmount the file system mounted at `mountpoint`.
///
/// Useful flags include
/// * `MNT_FORCE` -     Unmount even if still in use.
/// * `MNT_BYFSID` -    `mountpoint` is not a path, but a file system ID
///                     encoded as `FSID:val0:val1`, where `val0` and `val1`
///                     are the contents of the `fsid_t val[]` array in decimal.
///                     The file system that has the specified file system ID
///                     will be unmounted.  See
///                     [`statfs`](crate::sys::statfs::statfs) to determine the
///                     `fsid`.
pub fn unmount<P>(mountpoint: &P, flags: MntFlags) -> Result<()>
    where P: ?Sized + NixPath
{
    let res = mountpoint.with_nix_path(|cstr| {
        unsafe { libc::unmount(cstr.as_ptr(), flags.bits) }
    })?;

    Errno::result(res).map(drop)
}
