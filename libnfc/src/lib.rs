#![macro_use]
use hex;
extern crate libnfc_sys as ffi;

use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::{forget, transmute, MaybeUninit};
use std::ops::Deref;
use std::os::raw::{c_char, c_void};
use std::ptr::{null, null_mut};
use std::sync::Arc;

use ffi::{nfc_baud_rate, nfc_dep_mode, nfc_mode, nfc_modulation_type, nfc_property};

// Extra failure constants
const NFC_INITFAIL: i32 = -100;
const NFC_DEVOPENFAIL: i32 = -101;
const NFC_CONTEXTALREADYINUSE: i32 = -102;
const NFC_NULLPTR: i32 = -103;
const NFC_CANT_DISPOSE: i32 = -104;

#[doc = "Possible errors for the library"]
#[doc = "See [libnfc C API errors][1]"]
#[doc = " [1]: http://www.libnfc.org/api/group__error.html"]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    #[doc = "Custom added library errors."]
    NfcInitFail = NFC_INITFAIL as i32,
    NfcDevOpenFail = NFC_DEVOPENFAIL as i32,
    NfcContextAlreadyInUse = NFC_CONTEXTALREADYINUSE as i32,
    NfcNullPtr = NFC_NULLPTR as i32,
    NfcCantDispose = NFC_CANT_DISPOSE as i32,
    #[doc = "Library defined errors."]
    NfcSuccess = ffi::NFC_SUCCESS as i32,
    NfcEio = ffi::NFC_EIO as i32,
    NfcEinvArg = ffi::NFC_EINVARG as i32,
    NfcEDevNotSupp = ffi::NFC_EDEVNOTSUPP as i32,
    NfcENotSuchDev = ffi::NFC_ENOTSUCHDEV as i32,
    NfcEOvFlow = ffi::NFC_EOVFLOW as i32,
    NfcETimeout = ffi::NFC_ETIMEOUT as i32,
    NfcEOpAborted = ffi::NFC_EOPABORTED as i32,
    NfcENotImpl = ffi::NFC_ENOTIMPL as i32,
    NfcETgReleased = ffi::NFC_ETGRELEASED as i32,
    NfcERfTrans = ffi::NFC_ERFTRANS as i32,
    NfcEMfcAuthFail = ffi::NFC_EMFCAUTHFAIL as i32,
    NfcESoft = ffi::NFC_ESOFT as i32,
    NfcEChip = ffi::NFC_ECHIP as i32,
}

impl Error {
    fn from_raw(raw: i32) -> Error {
        unsafe {
            if raw >= ffi::NFC_ECHIP && raw <= ffi::NFC_SUCCESS {
                transmute::<i32, Error>(raw as i32)
            } else {
                if cfg!(debug_assertions) {
                    panic!("unknown libnfc error code: {:#x}", raw);
                }

                Error::NfcENotImpl
            }
        }
    }

    #[allow(dead_code)]
    fn device_last_error(device: &Device) {
        let err = unsafe { ffi::nfc_device_get_last_error(device.handle) };
        Error::from_raw(err);
    }
    // in case of other platforms, might wanna use self as i32 as X
    #[allow(dead_code)]
    fn into_raw(self) -> i32 {
        self as i32
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        // Descriptions from libnfc nfc.c
        match *self {
            // Custom added errors
            Error::NfcInitFail => "Unable to init libnfc",
            Error::NfcDevOpenFail => "Unable to open NFC device",
            Error::NfcContextAlreadyInUse => "libnfc context already in use",
            Error::NfcNullPtr => "Tried to access a null pointer",
            Error::NfcCantDispose => "Can't dispose of object",
            // Library defined errors
            Error::NfcSuccess => "Success",
            Error::NfcEio => {
                "Input / output error, device may not be usable anymore without re-open it"
            }
            Error::NfcEinvArg => "Invalid argument(s)",
            Error::NfcEDevNotSupp => "Operation not supported by Device",
            Error::NfcENotSuchDev => "No such device found",
            Error::NfcEOvFlow => "Buffer overflow",
            Error::NfcETimeout => "Operation timed out",
            Error::NfcEOpAborted => "Operation Aborted",
            Error::NfcENotImpl => "Not (yet) Implemented",
            Error::NfcETgReleased => "Target released",
            Error::NfcEMfcAuthFail => "Mifare authentication Failed",
            Error::NfcERfTrans => "RF transmission Error",
            Error::NfcESoft => "Software error (allocation, file/pipe creation, etc)",
            Error::NfcEChip => "Device internal Chip Error",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(std::error::Error::description(self))
    }
}

macro_rules! try_libnfc {
    ($e:expr) => {
        match $e {
            ffi::NFC_SUCCESS => (),
            err => return Err(Error::from_raw(err)),
        }
    };
}

#[doc = " Maximum connection string size"]
pub const NFC_BUFSIZE_CONNSTRING: u32 = ffi::NFC_BUFSIZE_CONNSTRING;

#[doc = " Connection string"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NfcConnstring {
    #[doc = "[c_char; 1024usize]"]
    connection_string: ffi::nfc_connstring,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Available modulation types
pub enum ModulationType {
    /// ISO-14443 A
    NmtIso14443A = ffi::NMT_ISO14443A as i32,
    /// ISO-14443 B
    NmtIso14443B = ffi::NMT_ISO14443B as i32,
    /// ISO-14443 BI
    NmtIso14443Bi = ffi::NMT_ISO14443BI as i32,
    /// ISO-14443B 2SR
    NmtIso14443B2SR = ffi::NMT_ISO14443B2SR as i32,
    /// ISO-14443B 2CT
    NmtIso14443B2CT = ffi::NMT_ISO14443B2CT as i32,
    /// Felicia
    NmtFelica = ffi::NMT_FELICA as i32,
    /// Jewel
    NmtJewel = ffi::NMT_JEWEL as i32,
    /// Barcode
    NmtBarcode = ffi::NMT_BARCODE as i32,
    /// Dep
    NmtDep = ffi::NMT_DEP as i32,
}

impl ModulationType {
    fn into_raw(self) -> nfc_modulation_type {
        nfc_modulation_type::from(self as nfc_modulation_type)
    }

    fn from_raw(raw: nfc_modulation_type) -> ModulationType {
        match raw {
            ffi::NMT_ISO14443A => ModulationType::NmtIso14443A,
            ffi::NMT_ISO14443B => ModulationType::NmtIso14443B,
            ffi::NMT_ISO14443BI => ModulationType::NmtIso14443Bi,
            ffi::NMT_ISO14443B2SR => ModulationType::NmtIso14443B2SR,
            ffi::NMT_ISO14443B2CT => ModulationType::NmtIso14443B2CT,
            ffi::NMT_FELICA => ModulationType::NmtFelica,
            ffi::NMT_JEWEL => ModulationType::NmtJewel,
            ffi::NMT_BARCODE => ModulationType::NmtBarcode,
            ffi::NMT_DEP => ModulationType::NmtDep,
            _ => panic!("Impossible modulation: {:#x}", raw),
        }
    }

    #[doc = " Returns a human readable description of the inner `ModulationType`."]
    pub fn str_modulation_type(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(ffi::str_nfc_modulation_type(
                *self as ffi::nfc_modulation_type,
            ))
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// Available baud rates
pub enum BaudRate {
    /// Undefined
    NbrUndefined = ffi::NBR_UNDEFINED,
    /// 106 kbits/s
    Nbr106 = ffi::NBR_106,
    /// 212 kbits/s
    Nbr212 = ffi::NBR_212,
    /// 424 kbits/s
    Nbr424 = ffi::NBR_424,
    /// 847 kbits/s
    Nbr847 = ffi::NBR_847,
}

impl BaudRate {
    fn into_raw(self) -> nfc_baud_rate {
        nfc_baud_rate::from(self as nfc_baud_rate)
    }

    fn from_raw(raw: nfc_baud_rate) -> BaudRate {
        match raw {
            ffi::NBR_UNDEFINED => BaudRate::NbrUndefined,
            ffi::NBR_106 => BaudRate::Nbr106,
            ffi::NBR_212 => BaudRate::Nbr212,
            ffi::NBR_424 => BaudRate::Nbr424,
            ffi::NBR_847 => BaudRate::Nbr847,
            _ => panic!("Impossible baud rate: {:#x}", raw),
        }
    }

    #[doc = " Returns a human readable description of the inner `Baudrate`."]
    pub fn str_baud_rate(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::str_nfc_baud_rate(*self as ffi::nfc_baud_rate)) }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[doc = " Mode"]
#[doc = " NFC mode type enumeration"]
pub enum Mode {
    #[doc = " As target"]
    NTarget = ffi::N_TARGET,
    #[doc = " As initiator"]
    NInitiator = ffi::N_INITIATOR,
}

impl Mode {
    fn into_raw(self) -> nfc_mode {
        nfc_mode::from(self as nfc_mode)
    }

    fn from_raw(raw: nfc_mode) -> Mode {
        match raw {
            ffi::N_TARGET => Mode::NTarget,
            ffi::N_INITIATOR => Mode::NInitiator,
            _ => panic!("Impossible mode: {:#x}", raw),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[doc = " DepMode"]
#[doc = " NFC D.E.P. (Data Exchange Protocol) active/passive mode"]
pub enum DepMode {
    NdmUndefined = ffi::NDM_UNDEFINED,
    NdmPassive = ffi::NDM_PASSIVE,
    NdmActive = ffi::NDM_ACTIVE,
}

impl DepMode {
    fn into_raw(self) -> nfc_dep_mode {
        nfc_dep_mode::from(self as nfc_dep_mode)
    }

    fn from_raw(raw: nfc_dep_mode) -> DepMode {
        match raw {
            ffi::NDM_UNDEFINED => DepMode::NdmUndefined,
            ffi::NDM_ACTIVE => DepMode::NdmActive,
            ffi::NDM_PASSIVE => DepMode::NdmPassive,
            _ => panic!("Impossible dep mode: {:#x}", raw),
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
/// Property
///
/// Used to configure parameters and registers that control for example timing,
/// modulation, frame and error handling.
pub enum Property {
    #[doc = " Default command processing timeout"]
    #[doc = " Property value\'s (duration) unit is ms and 0 means no timeout (infinite)."]
    #[doc = " Default value is set by driver layer"]
    NpTimeoutCommand = ffi::NP_TIMEOUT_COMMAND,
    #[doc = " Timeout between ATR_REQ and ATR_RES"]
    #[doc = " When the device is in initiator mode, a target is considered as mute if no"]
    #[doc = " valid ATR_RES is received within this timeout value."]
    #[doc = " Default value for this property is 103 ms on PN53x based devices."]
    NpTimeoutAtr = ffi::NP_TIMEOUT_ATR,
    #[doc = " Timeout value to give up reception from the target in case of no answer."]
    #[doc = " Default value for this property is 52 ms)."]
    NpTimeoutCom = ffi::NP_TIMEOUT_COM,
    #[doc = " Let the PN53X chip handle the CRC bytes. This means that the chip appends"]
    #[doc = " the CRC bytes to the frames that are transmitted. It will parse the last"]
    #[doc = " bytes from received frames as incoming CRC bytes. They will be verified"]
    #[doc = " against the used modulation and protocol. If an frame is expected with"]
    #[doc = " incorrect CRC bytes this option should be disabled. Example frames where"]
    #[doc = " this is useful are the ATQA and UID+BCC that are transmitted without CRC"]
    #[doc = " bytes during the anti-collision phase of the ISO14443-A protocol."]
    NpHandleCrc = ffi::NP_HANDLE_CRC,
    #[doc = " Parity bits in the network layer of ISO14443-A are by default generated and"]
    #[doc = " validated in the PN53X chip. This is a very convenient feature. On certain"]
    #[doc = " times though it is useful to get full control of the transmitted data. The"]
    #[doc = " proprietary MIFARE Classic protocol uses for example custom (encrypted)"]
    #[doc = " parity bits. For interoperability it is required to be completely"]
    #[doc = " compatible, including the arbitrary parity bits. When this option is"]
    #[doc = " disabled, the functions to communicating bits should be used."]
    NpHandleParity = ffi::NP_HANDLE_PARITY,
    #[doc = " This option can be used to enable or disable the electronic field of the"]
    #[doc = " NFC device."]
    NpActivateField = ffi::NP_ACTIVATE_FIELD,
    #[doc = " The internal CRYPTO1 co-processor can be used to transmit messages"]
    #[doc = " encrypted. This option is automatically activated after a successful MIFARE"]
    #[doc = " Classic authentication."]
    NpActivateCrypto01 = ffi::NP_ACTIVATE_CRYPTO1,
    #[doc = " The default configuration defines that the PN53X chip will try indefinitely"]
    #[doc = " to invite a tag in the field to respond. This could be desired when it is"]
    #[doc = " certain a tag will enter the field. On the other hand, when this is"]
    #[doc = " uncertain, it will block the application. This option could best be compared"]
    #[doc = " to the (NON)BLOCKING option used by (socket)network programming."]
    NpInfiniteSelect = ffi::NP_INFINITE_SELECT,
    #[doc = " If this option is enabled, frames that carry less than 4 bits are allowed."]
    #[doc = " According to the standards these frames should normally be handles as"]
    #[doc = " invalid frames."]
    NpAcceptInvalidFrames = ffi::NP_ACCEPT_INVALID_FRAMES,
    #[doc = " If the NFC device should only listen to frames, it could be useful to let"]
    #[doc = " it gather multiple frames in a sequence. They will be stored in the internal"]
    #[doc = " FIFO of the PN53X chip. This could be retrieved by using the receive data"]
    #[doc = " functions. Note that if the chip runs out of bytes (FIFO = 64 bytes long),"]
    #[doc = " it will overwrite the first received frames, so quick retrieving of the"]
    #[doc = " received data is desirable."]
    NpAcceptMultipleFrames = ffi::NP_ACCEPT_MULTIPLE_FRAMES,
    #[doc = " This option can be used to enable or disable the auto-switching mode to"]
    #[doc = " ISO14443-4 if device is compliant. In initiator mode, it means that "]
    #[doc = " NFC chip will send RATS automatically when in select and it will automatically"]
    #[doc = " poll for ISO14443-4 card when ISO14443A is requested."]
    #[doc = " In target mode, with a NFC chip compliant (ie. PN532), the chip will emulate"]
    #[doc = " a 14443-4 PICC using hardware capability."]
    NpAutoIso14443_4 = ffi::NP_AUTO_ISO14443_4,
    #[doc = " Use automatic frames encapsulation and chaining."]
    NpEasyFraming = ffi::NP_EASY_FRAMING,
    #[doc = "Force the chip to switch in ISO14443-A."]
    NpForceIso14443A = ffi::NP_FORCE_ISO14443_A,
    #[doc = "Force the chip to switch in ISO14443-B."]
    NpForceIso14443B = ffi::NP_FORCE_ISO14443_B,
    #[doc = "Force the chip to run at 106 kbps."]
    NpForceSpeed_106 = ffi::NP_FORCE_SPEED_106,
}

impl Property {
    fn into_raw(self) -> nfc_property {
        nfc_property::from(self as nfc_property)
    }

    fn from_raw(raw: nfc_property) -> Property {
        match raw {
            ffi::NP_TIMEOUT_COMMAND => Property::NpTimeoutCommand,
            ffi::NP_TIMEOUT_ATR => Property::NpTimeoutAtr,
            ffi::NP_TIMEOUT_COM => Property::NpTimeoutCom,
            ffi::NP_HANDLE_CRC => Property::NpHandleCrc,
            ffi::NP_HANDLE_PARITY => Property::NpHandleParity,
            ffi::NP_ACTIVATE_FIELD => Property::NpActivateField,
            ffi::NP_ACTIVATE_CRYPTO1 => Property::NpActivateCrypto01,
            ffi::NP_INFINITE_SELECT => Property::NpInfiniteSelect,
            ffi::NP_ACCEPT_INVALID_FRAMES => Property::NpAcceptInvalidFrames,
            ffi::NP_ACCEPT_MULTIPLE_FRAMES => Property::NpAcceptMultipleFrames,
            ffi::NP_AUTO_ISO14443_4 => Property::NpAutoIso14443_4,
            ffi::NP_EASY_FRAMING => Property::NpEasyFraming,
            ffi::NP_FORCE_ISO14443_A => Property::NpForceIso14443A,
            ffi::NP_FORCE_ISO14443_B => Property::NpForceIso14443B,
            ffi::NP_FORCE_SPEED_106 => Property::NpForceSpeed_106,
            _ => panic!("Impossible property: {:#x}", raw),
        }
    }
}

#[allow(dead_code)]
struct Driver {
    inner: Arc<DriverInner>,
}

struct DriverInner {
    handle: *const ffi::nfc_driver,
}

impl Driver {
    #[allow(dead_code)]
    pub fn register_driver(drv: Driver) {
        unsafe { ffi::nfc_register_driver(drv.inner.handle) };
    }
}

struct ContextInner {
    // this pointer must never be allowed to leave the struct
    handle: *mut ffi::nfc_context,
}

#[doc = " Library context to libnfc."]
#[doc = " This structure wraps `nfc_context`"]
pub struct Context {
    inner: Arc<ContextInner>,
}

impl Context {
    #[doc = " Initializes libnfc."]
    #[doc = " This function must be called before calling any other library functions."]
    #[doc = " On success returns a new library context,"]
    #[doc = " otherwise an error signifying library initialization is returned."]
    pub fn init() -> Result<Self, Error> {
        unsafe {
            // first try raw
            let mut handle: *mut ffi::nfc_context = null_mut();
            ffi::nfc_init(&mut handle);

            if handle.is_null() {
                return Err(Error::NfcInitFail);
            }

            Ok(Context {
                inner: Arc::new(ContextInner { handle }),
            })
        }
    }

    #[doc = " Release the context."]
    #[doc = " In case of an error, ownership of the context is returned to the caller."]
    #[doc = " This function wraps `nfc_exit`"]
    #[doc = " ## Note "]
    #[doc = " `Context` implements `Drop` that automatically releases the context."]
    #[doc = " This function manually closes the context."]
    pub fn release(self) -> Result<(), (Context, Error)> {
        match Arc::try_unwrap(self.inner) {
            Ok(inner) => {
                unsafe {
                    ffi::nfc_exit(inner.handle);
                    // Skip the drop, as we do it manually later
                    forget(inner);

                    Ok(())
                }
            }
            Err(arc_inner) => {
                let context = Context { inner: arc_inner };
                Err((context, Error::NfcCantDispose))
            }
        }
    }

    #[doc = " Open a NFC device with a specified connection string."]
    pub fn open_device(&mut self, connstring: &CStr) -> Result<Device, Error> {
        unsafe {
            let handle = ffi::nfc_open(self.inner.handle, connstring.as_ptr());

            if handle.is_null() {
                return Err(Error::NfcENotSuchDev);
            }

            let ptr: *mut c_char = null_mut();
            let len = 0;

            Ok(Device {
                _context: self.clone(),
                handle,
                name: CStr::from_ptr(ffi::nfc_device_get_name(handle))
                    .to_string_lossy()
                    .into_owned(),
                connection_string: CStr::from_ptr(ffi::nfc_device_get_connstring(handle))
                    .to_string_lossy()
                    .into_owned(),
                information: DeviceAbout { ptr, len },
            })
        }
    }

    #[doc = " Open the first NFC device found."]
    #[doc = " On success returns the opened `Device` back to the caller."]
    #[doc = "otherwise a library Error variant is returned."]
    pub fn open_default_device(&mut self) -> Result<Device, Error> {
        unsafe {
            let handle = ffi::nfc_open(self.inner.handle, null());

            if handle.is_null() {
                return Err(Error::NfcENotSuchDev);
            }

            let ptr: *mut c_char = null_mut();

            Ok(Device {
                _context: self.clone(),
                handle,
                name: CStr::from_ptr(ffi::nfc_device_get_name(handle))
                    .to_string_lossy()
                    .into_owned(),
                connection_string: CStr::from_ptr(ffi::nfc_device_get_connstring(handle))
                    .to_string_lossy()
                    .into_owned(),
                information: DeviceAbout { ptr, len: 0 },
            })
        }
    }

    #[doc = " Return the current libnfc library version used."]
    pub fn lib_version(&self) -> &CStr {
        unsafe { CStr::from_ptr(ffi::nfc_version()) }
    }
    /*pub fn list_devices(&self, connstrings: [nfc_connstring; nConstrings]) -> i32 {} */
}

/// Represents a connected device.
///
/// This structure wraps `nfc_device`.
/// A device can be set as initiator meaning it acts as a reader
/// or as target meaning the device acts as an emulated tag.
///
/// The device is closed automatically on `Drop`
pub struct Device {
    // Keep it alive
    _context: Context,
    handle: *mut ffi::nfc_device,
    name: String,
    connection_string: String,
    information: DeviceAbout,
}

/// Device information
struct DeviceAbout {
    ptr: *mut c_char,
    len: usize,
}

impl Device {
    /// Initialize the device as initiator (reader)
    ///
    /// The NFC device is configured to function as an RFID reader.
    /// After initialization it can be used to communicate to passive RFID tags and active NFC devices.
    /// The reader will act as initiator to communicate peer 2 peer (NFCIP) to other active NFC devices.
    ///
    /// ## Note
    /// Crc is handled by the device (NP_HANDLE_CRC = true)
    /// Parity is handled the device (NP_HANDLE_PARITY = true)
    /// Cryto1 cipher is disabled (NP_ACTIVATE_CRYPTO1 = false)
    /// Easy framing is enabled (NP_EASY_FRAMING = true)
    /// Auto-switching in ISO14443-4 mode is enabled (NP_AUTO_ISO14443_4 = true)
    /// Invalid frames are not accepted (NP_ACCEPT_INVALID_FRAMES = false)
    /// Multiple frames are not accepted (NP_ACCEPT_MULTIPLE_FRAMES = false)
    /// 14443-A mode is activated (NP_FORCE_ISO14443_A = true)
    /// speed is set to 106 kbps (NP_FORCE_SPEED_106 = true)
    /// Let the device try forever to find a target (NP_INFINITE_SELECT = true)
    /// RF field is shortly dropped (if it was enabled) then activated again
    pub fn initiator_init(&mut self) -> Result<(), Error> {
        unsafe {
            try_libnfc!(
                ffi::nfc_initiator_init(self.handle)
            );

            Ok(())
        }
    }

    #[doc = " Initialize NFC device as initiator with its secure element initiator (reader)."]
    #[doc = " The NFC device is configured to function as secure element reader. After initialization it can be used to communicate with the secure element."]
    #[doc = " ## Note"]
    #[doc = " RF field is de-activated in order to some power."]
    pub fn initiator_secure_element(&self) -> Result<(), Error> {
        unsafe {
            try_libnfc!(
                ffi::nfc_initiator_init_secure_element(self.handle)
            );

            Ok(())
        }
    }

    pub fn transaction(&mut self) -> Result<Transaction, Error> {
        Ok(Transaction { device: self })
    }

    pub fn close(&mut self) {
        unsafe {
            ffi::nfc_close(self.handle);

            // Skip the drop, we did it manually.
            std::ptr::drop_in_place(&mut self._context);
            forget(self);
        }
    }

    /// Abort current running command on the device
    ///
    /// Some commands (ie. target_init()) are blocking functions and will return only in particular conditions (ie. external initiator request). This function attempt to abort the current running command.
    ///
    /// ## Note
    /// The blocking function (ie. target_init()) will fail with NfcEOpAborted error.
    pub fn abort_command(&mut self) -> Result<(), Error> {
        unsafe {
            try_libnfc!(ffi::nfc_abort_command(self.handle));

            Ok(())
        }
    }

    /// Turn the NFC device into idle mode
    ///
    /// This function switches the device into idle mode.
    /// In initiator mode, the RF field is turned off and the device is set to low power mode (if avaible).
    /// In target mode, the emulation is stopped (no target available from external initiator) and the device is set to low power mode (if available).
    pub fn idle(&mut self) -> Result<(), Error> {
        unsafe {
            try_libnfc!(ffi::nfc_idle(self.handle));

            Ok(())
        }
    }

    /// Returns the device name.
    pub fn name(&self) -> &str {
        // Lifetime elision assigns this the same lifetime as &self; this
        // is what we want, and is safe.
        //unsafe { CStr::from_ptr(ffi::nfc_device_get_name(self.handle)) }
        self.name.as_ref()
    }

    /// Returns the device connection string.
    pub fn connstring(&mut self) -> &str {
        // Lifetime elision assigns this the same lifetime as &self; this
        // is what we want, and is safe.
        //unsafe { CStr::from_ptr(ffi::nfc_device_get_connstring(self.handle)) }
        self.connection_string.as_ref()
    }

    pub fn supported_baud_rate(
        &self,
        nmt: ModulationType,
        supported_br: &[BaudRate],
    ) -> Result<bool, Error> {
        unsafe {
            let baud_rates = supported_br
                .iter()
                .map(|br| BaudRate::into_raw(*br))
                .collect::<Vec<nfc_baud_rate>>();

            try_libnfc!(ffi::nfc_device_get_supported_baud_rate(
                self.handle,
                ModulationType::into_raw(nmt),
                &mut baud_rates.as_ptr(),
            ));

            Ok(true)
        }
    }

    pub fn supported_modulation(
        &self,
        mode: Mode,
        supported_mt: &[ModulationType],
    ) -> Result<bool, Error> {
        unsafe {
            let modulations = supported_mt
                .iter()
                .map(|modulation| ModulationType::into_raw(*modulation))
                .collect::<Vec<nfc_modulation_type>>();

            try_libnfc!(ffi::nfc_device_get_supported_modulation(
                self.handle,
                Mode::into_raw(mode),
                &mut modulations.as_ptr(),
            ));

            Ok(true)
        }
    }

    /// Get information about the NFC device.
    ///
    /// Upon success returns the device information.
    /// If the buffer returned internally is invalid an Error::NfcNullPtr is returned.
    /// Any other errors are signified by a library error variant.
    pub fn information(&mut self) -> Result<&CStr, Error> {
        unsafe {
            let device_info_len: i32 =
                ffi::nfc_device_get_information_about(self.handle, &mut self.information.ptr) + 1;

            if self.information.ptr.is_null() {
                return Err(Error::NfcNullPtr);
            }

            if device_info_len < 0 {
                return Err(Error::from_raw(device_info_len));
            }

            self.information.len = device_info_len as usize;

            Ok(CStr::from_ptr(self.information.ptr))
        }
    }

    /// Deselect a selected passive or emulated tag
    /// Returns 0 on success, otherwise returns libnfc's error code (negative value).
    /// After selecting and communicating with a passive tag, this function could be
    /// used to deactivate and release the tag.
    /// This is very useful when there are multiple tags available in the field.
    /// It is possible to use the function initiator_select_passive_target() 
    /// function to select the first available tag, test it for the available features 
    /// and support, deselect it and skip to the next tag until the correct tag is found.
    pub fn initiator_deselect_target(&self) -> Result<(), Error> {
        unsafe {
            try_libnfc!(
                ffi::nfc_initiator_deselect_target(self.handle)
            );

            Ok(())
        }
    }
    /// Select a passive or emulated tag with default initiator data
    pub fn initiator_select_passive_target_default<'a>(
        &'a self,
        nm: Modulation,
    ) -> Result<Target<'a>, Error> {
        let modulation = ffi::nfc_modulation {
            nmt: nm.nmt as ffi::nfc_modulation_type,
            nbr: nm.nbr as ffi::nfc_baud_rate,
        };

        let mut handle: MaybeUninit<ffi::nfc_target> = MaybeUninit::uninit();

        unsafe {
            let result = ffi::nfc_initiator_select_passive_target(
                self.handle,
                modulation,
                null(),
                0,
                handle.as_mut_ptr(),
            );

            if result < 0 {
                return Err(Error::from_raw(result));
            }

            let handle = handle.assume_init();

            let ptr: *mut c_char = null_mut();

            Ok(Target::from(&self, handle, TargetAbout { ptr, len: 0 }))
        }
    }

    /// Select a passive or emulated tag with supplied initiator data.
    /// On success returns a `Target` and if any errors occured, returns a library Error variant.
    pub fn initiator_select_passive_target<'a>(
        &'a self,
        nm: Modulation,
        pbt_init_data: &[u8],
    ) -> Result<Target<'a>, Error> {
        let pbt_init_data_len = if pbt_init_data.len() > 0 {
            pbt_init_data.len()
        } else {
            0 as usize
        };
        let pbt_init_data_ptr = if pbt_init_data.len() > 0 {
            pbt_init_data.as_ptr()
        } else {
            null()
        };

        let lib_modulation = ffi::nfc_modulation {
            nmt: ModulationType::into_raw(nm.nmt),
            nbr: BaudRate::into_raw(nm.nbr),
        };

        // Our [out] ptr for nfc_target
        let mut handle: MaybeUninit<ffi::nfc_target> = MaybeUninit::uninit();

        unsafe {
            let result = ffi::nfc_initiator_select_passive_target(
                self.handle,
                lib_modulation,
                pbt_init_data_ptr,
                pbt_init_data_len,
                handle.as_mut_ptr(),
            );

            if result < 0 {
                return Err(Error::from_raw(result));
            }

            let handle = handle.assume_init();

            let ptr: *mut c_char = null_mut();
            Ok(Target::from(&self, handle, TargetAbout { ptr, len: 0 }))
        }
    }

    pub fn initiator_target_is_present(&self, target: &Target) -> Result<(), Error> {
        unsafe {
            try_libnfc!(ffi::nfc_initiator_target_is_present(
                self.handle,
                &target.handle,
            ));

            Ok(())
        }
    }

    pub fn initiator_last_tag_is_present(&self) -> Result<(), Error> {
        unsafe {
            try_libnfc!(ffi::nfc_initiator_target_is_present(self.handle, null(),));

            Ok(())
        }
    }

    pub fn set_property_int(&self, property: Property, value: i32) -> Result<(), Error> {
        unsafe {
            try_libnfc!(ffi::nfc_device_set_property_int(
                self.handle,
                Property::into_raw(property),
                value
            ));

            Ok(())
        }
    }

    pub fn set_property_bool(&self, property: Property, enable: bool) -> Result<(), Error> {
        unsafe {
            try_libnfc!(ffi::nfc_device_set_property_bool(
                self.handle,
                Property::into_raw(property),
                enable,
            ));

            Ok(())
        }
    }
}

/// Target represents a target for nfc operations. It can be set to one of two different states.
/// initiator: device acts as a reader.
/// target: device acts as an emulated tag.
pub struct Target<'a> {
    handle: ffi::nfc_target,
    information: TargetAbout,
    phantom: PhantomData<&'a Device>,
}

struct TargetAbout {
    ptr: *mut c_char,
    len: usize,
}

impl<'a> Target<'a> {
    fn from<'dev>(
        _: &'dev Device,
        handle: ffi::nfc_target,
        information: TargetAbout,
    ) -> Target<'dev> {
        Target {
            handle,
            information,
            phantom: PhantomData,
        }
    }

    pub fn get_iso14443a_info(&mut self) -> Iso14443a {
        unsafe {
            Iso14443a {
                data: &mut *self.handle.nti.nai(),
            }
        }
    }

    pub fn get_iso14443b_info(&mut self) -> Iso14443b {
        unsafe {
            Iso14443b {
                data: &mut *self.handle.nti.nbi(),
            }
        }
    }

    pub fn get_iso14443bi_info(&mut self) -> Iso14443bi {
        unsafe {
            Iso14443bi {
                data: &mut *self.handle.nti.nii(),
            }
        }
    }

    pub fn get_iso14443b2sr_info(&mut self) -> Iso14443b2sr {
        unsafe {
            Iso14443b2sr {
                data: &mut *self.handle.nti.nsi(),
            }
        }
    }

    pub fn get_iso14443b2ct_info(&mut self) -> Iso14443b2ct {
        unsafe {
            Iso14443b2ct {
                data: &mut *self.handle.nti.nci(),
            }
        }
    }

    pub fn get_felicia_info(&mut self) -> Felica {
        unsafe {
            Felica {
                data: &mut *self.handle.nti.nfi(),
            }
        }
    }

    pub fn get_jewel_info(&mut self) -> Jewel {
        unsafe {
            Jewel {
                data: &mut *self.handle.nti.nji(),
            }
        }
    }

    pub fn get_barcode_info(&mut self) -> Barcode {
        unsafe {
            Barcode {
                data: &mut *self.handle.nti.nti(),
            }
        }
    }

    pub fn get_dep_info(&mut self) -> Dep {
        unsafe {
            Dep {
                data: &mut *self.handle.nti.ndi(),
            }
        }
    }

    /// Get the modulation settings used.
    pub fn target_modulation(&self) -> Modulation {
        let mod_type = ModulationType::from_raw(self.handle.nm.nmt);
        let baud_rate = BaudRate::from_raw(self.handle.nm.nbr);

        Modulation {
            nmt: mod_type,
            nbr: baud_rate,
        }
    }

    /// Get information about the `Target`.
    ///
    /// Upon success returns the target information.
    ///
    /// If the buffer returned internally is invalid, an Error::NfcNullPtr is returned.
    /// Any other errors are signified by a library error variant.
    pub fn str_target(&mut self, verbose: bool) -> Result<&CStr, Error> {
        unsafe {
            let target_info_len: i32 =
                ffi::str_nfc_target(&mut self.information.ptr, &self.handle, verbose) + 1;

            if self.information.ptr.is_null() {
                return Err(Error::NfcNullPtr);
            }

            if target_info_len < 0 {
                return Err(Error::from_raw(target_info_len));
            }

            self.information.len = target_info_len as usize;

            //let target_information_about: &[c_char] = std::slice::from_raw_parts(self.information.ptr, self.information.len);
            Ok(CStr::from_ptr(self.information.ptr))
        }
    }
}

#[doc = "Modulation type."]
#[doc = "Wraps `nfc_modulation`."]
pub struct Modulation {
    nmt: ModulationType,
    nbr: BaudRate,
}

impl Modulation {
    #[doc = " Create a new `Modulation` instance."]
    pub fn new(nmt: ModulationType, nbr: BaudRate) -> Modulation {
        Modulation { nmt, nbr }
    }

    #[doc = " Returns a human readable description of the inner `Baudrate`."]
    pub fn str_baud_rate(&self) -> &CStr {
        self.nbr.str_baud_rate()
    }

    #[doc = " Returns a human readable description of the inner `ModulationType`."]
    pub fn str_modulation_type(&self) -> &CStr {
        self.nmt.str_modulation_type()
    }
}

#[doc = "NFC ISO14443A tag (MIFARE) information"]
pub struct Iso14443a<'a> {
    pub data: &'a mut ffi::nfc_iso14443a_info,
}

impl<'a> Iso14443a<'a> {
    #[doc = " Return atqa"]
    pub fn atqa(&self) -> &[u8] {
        &self.data.abtAtqa
    }

    #[doc = " Return sak"]
    pub fn sak(&self) -> u8 {
        self.data.btSak
    }

    #[doc = " Return uid"]
    pub fn uid(&self) -> &[u8] {
        &self.data.abtUid[..self.data.szUidLen]
    }

    #[doc = " Return hex representation of the uid"]
    pub fn hex_uid(&self) -> String {
        hex::encode_upper(self.uid())
    }

    #[doc = " Return ats"]
    pub fn ats(&self) -> Option<&[u8]> {
        if self.data.szAtsLen != 0 {
            Some(&self.data.abtAts[..self.data.szAtsLen])
        } else {
            None
        }
    }

    #[doc = " Return hex representation of the ATS"]
    pub fn hex_ats(&self) -> Option<String> {
        match self.ats() {
            Some(ats) => Some(hex::encode_upper(ats)),
            None => None,
        }
    }
}
#[doc = " NFC ISO14443B tag information"]
pub struct Iso14443b<'a> {
    pub data: &'a mut ffi::nfc_iso14443b_info,
}

impl<'a> Iso14443b<'a> {
    #[doc = " Return PUPI contained in ATQB (Answer To reQuest of type B) (see ISO14443-3)"]
    pub fn pupi(&self) -> &[u8] {
        &self.data.abtPupi
    }

    #[doc = " Return Application Data contained in ATQB (see ISO14443-3)"]
    pub fn application_data(&self) -> &[u8] {
        &self.data.abtApplicationData
    }

    #[doc = " Return Protocol Info contained in ATQB (see ISO14443-3)"]
    pub fn protocol_info(&self) -> &[u8] {
        &self.data.abtProtocolInfo
    }

    #[doc = " Return CID (Card Identifier) attributted by PCD to the PICC"]
    pub fn card_identifier(&self) -> u8 {
        self.data.ui8CardIdentifier
    }
}

#[doc = " NFC ISO14443Bi tag information"]
pub struct Iso14443bi<'a> {
    pub data: &'a mut ffi::nfc_iso14443bi_info,
}

impl<'a> Iso14443bi<'a> {
    #[doc = " Return the 4 least significant bytes of tag serial number"]
    pub fn div(&self) -> &[u8] {
        &self.data.abtDIV
    }

    #[doc = " Return Software version & type of REPGEN"]
    pub fn software_version(&self) -> u8 {
        self.data.btVerLog
    }

    #[doc = " Return Config Byte, present if long REPGEN"]
    pub fn config_byte(&self) -> u8 {
        self.data.btConfig
    }

    #[doc = " Return ATR, if any"]
    pub fn atr(&self) -> Option<&[u8]> {
        if self.data.szAtrLen != 0 {
            Some(&self.data.abtAtr)
        } else {
            None
        }
    }

    #[doc = " Return hex representation of the ATR"]
    pub fn hex_atr(&self) -> Option<String> {
        match self.atr() {
            Some(atr) => Some(hex::encode_upper(atr)),
            None => None,
        }
    }
}

#[doc = " NFC ISO14443-2B ST SRx tag information"]
pub struct Iso14443b2sr<'a> {
    pub data: &'a mut ffi::nfc_iso14443b2sr_info,
}

impl<'a> Iso14443b2sr<'a> {
    #[doc = " Return uid"]
    pub fn uid(&self) -> &[u8] {
        &self.data.abtUID
    }

    #[doc = " Return hex representation of the uid"]
    pub fn hex_uid(&self) -> String {
        hex::encode_upper(self.uid())
    }
}

#[doc = " NFC ISO14443-2B ASK CTx tag information"]
pub struct Iso14443b2ct<'a> {
    pub data: &'a mut ffi::nfc_iso14443b2ct_info,
}

impl<'a> Iso14443b2ct<'a> {
    #[doc = " Return uid"]
    pub fn uid(&self) -> &[u8] {
        &self.data.abtUID
    }

    #[doc = " Return hex representation of the uid"]
    pub fn hex_uid(&self) -> String {
        hex::encode_upper(self.uid())
    }

    #[doc = " Return production code"]
    pub fn production_code(&self) -> u8 {
        self.data.btProdCode
    }

    #[doc = " Return fabrication code"]
    pub fn fabrication_code(&self) -> u8 {
        self.data.btFabCode
    }
}

#[doc = " NFC FeLiCa tag information"]
pub struct Felica<'a> {
    pub data: &'a mut ffi::nfc_felica_info,
}

impl<'a> Felica<'a> {
    #[doc = " Return Felicia length"]
    pub fn len(&self) -> usize {
        self.data.szLen
    }

    #[doc = " Return result code"]
    pub fn result_code(&self) -> u8 {
        self.data.btResCode
    }

    #[doc = " Return id"]
    pub fn id(&self) -> &[u8] {
        &self.data.abtId
    }

    #[doc = " Return hex representation of id"]
    pub fn hex_id(&self) -> String {
        hex::encode_upper(self.id())
    }

    #[doc = " Return system code"]
    pub fn system_code(&self) -> &[u8] {
        &self.data.abtSysCode
    }
}

#[doc = " NFC Jewel tag information"]
pub struct Jewel<'a> {
    pub data: &'a mut ffi::nfc_jewel_info,
}

impl<'a> Jewel<'a> {
    #[doc = " Return sensor result"]
    pub fn sensor_result(&self) -> &[u8] {
        &self.data.btSensRes
    }

    #[doc = " Return id"]
    pub fn id(&self) -> &[u8] {
        &self.data.btId
    }

    #[doc = " Return hex representation of id"]
    pub fn hex_id(&self) -> String {
        hex::encode_upper(self.id())
    }
}

#[doc = " Thinfilm NFC Barcode information"]
pub struct Barcode<'a> {
    pub data: &'a mut ffi::nfc_barcode_info,
}

impl<'a> Barcode<'a> {
    #[doc = " Return barcode data"]
    pub fn data(&self) -> &[u8] {
        &self.data.abtData[..self.data.szDataLen]
    }

    #[doc = " Return hex representation of barcode data"]
    pub fn hex_data(&self) -> String {
        hex::encode_upper(self.data())
    }
}

#[doc = " NFC target information in D.E.P. (Data Exchange Protocol) see ISO/IEC 18092 (NFCIP-1)"]
pub struct Dep<'a> {
    pub data: &'a mut ffi::nfc_dep_info,
}

impl<'a> Dep<'a> {
    #[doc = " Return NFCID3"]
    pub fn nfcid3(&self) -> &[u8] {
        &self.data.abtNFCID3
    }

    #[doc = " Return hex representation of NFCID3"]
    pub fn hex_nfcid3(&self) -> String {
        hex::encode_upper(self.nfcid3())
    }

    #[doc = " Return DID"]
    pub fn did(&self) -> u8 {
        self.data.btDID
    }

    #[doc = " Return supported send-bit rate"]
    pub fn supported_send_bit_rate(&self) -> u8 {
        self.data.btBS
    }

    #[doc = " Return supported recieve-bit rate"]
    pub fn supported_receive_bit_rate(&self) -> u8 {
        self.data.btBR
    }

    #[doc = " Return timeout value"]
    pub fn timeout_value(&self) -> u8 {
        self.data.btTO
    }

    #[doc = " Return PP parameters"]
    pub fn pp_parameters(&self) -> u8 {
        self.data.btPP
    }

    #[doc = " Return General Bytes"]
    pub fn general_bytes(&self) -> &[u8] {
        &self.data.abtGB[..self.data.szGB]
    }

    #[doc = " Return DEP Mode"]
    pub fn dep_mode(&self) -> DepMode {
        DepMode::from_raw(self.data.ndm)
    }
}
/// An exclusive transaction with a device.
///
/// A transaction ensures uninterrupted access to the device for its
/// duration. All other operations performed on the same underlying
/// device (even from other processes) will block until the transaction
/// is finished.
// By taking a mut reference to the device we statically enforce that:
// - There can only be one active transaction at a time.
// - All operations on the device must be performed through the transaction
//   for the duration of the transaction's lifetime.
pub struct Transaction<'tx> {
    device: &'tx mut Device,
}

impl<'tx> Deref for Transaction<'tx> {
    type Target = Device;

    fn deref(&self) -> &Device {
        self.device
    }
}

impl Drop for ContextInner {
    fn drop(&mut self) {
        //println!("Dropping ContextInner");
        unsafe { ffi::nfc_exit(self.handle) }
    }
}

impl Clone for Context {
    /// Clone for the 'Context'
    ///
    /// ## Implementation note
    ///
    /// This is implemented in the rust side with an `Arc::clone`.
    fn clone(&self) -> Self {
        Context {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        //println!("Dropping Device");
        unsafe {
            ffi::nfc_close(self.handle);
        }
    }
}

impl Drop for DeviceAbout {
    fn drop(&mut self) {
        //println!("Dropping DeviceInfo");
        unsafe {
            ffi::nfc_free(self.ptr as *mut c_void);
        }
    }
}

impl Drop for TargetAbout {
    fn drop(&mut self) {
        //println!("Dropping TargetAbout");
        unsafe { ffi::nfc_free(self.ptr as *mut c_void) }
    }
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}
unsafe impl Send for Device {}
unsafe impl Sync for Device {}
unsafe impl<'a> Send for Target<'a> {}
unsafe impl<'a> Sync for Target<'a> {}
