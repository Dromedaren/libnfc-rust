#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
use libc;

pub const NFC_BUFSIZE_CONNSTRING: u32 = 1024;

pub const NFC_SUCCESS: i32 = 0;
pub const NFC_EIO: i32 = -1;
pub const NFC_EINVARG: i32 = -2;
pub const NFC_EDEVNOTSUPP: i32 = -3;
pub const NFC_ENOTSUCHDEV: i32 = -4;
pub const NFC_EOVFLOW: i32 = -5;
pub const NFC_ETIMEOUT: i32 = -6;
pub const NFC_EOPABORTED: i32 = -7;
pub const NFC_ENOTIMPL: i32 = -8;
pub const NFC_ETGRELEASED: i32 = -10;
pub const NFC_ERFTRANS: i32 = -20;
pub const NFC_EMFCAUTHFAIL: i32 = -30;
pub const NFC_ESOFT: i32 = -80;
pub const NFC_ECHIP: i32 = -90;

pub type size_t = libc::size_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nfc_context {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nfc_device {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nfc_emulator {
    pub target: *mut nfc_target,
    pub state_machine: *mut nfc_emulation_state_machine,
    pub user_data: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nfc_emulation_state_machine {
    pub io: ::std::option::Option<
        unsafe extern "C" fn(
            emulator: *mut nfc_emulator,
            data_in: *const u8,
            data_in_len: usize,
            data_out: *mut u8,
            data_out_len: usize,
        ) -> ::std::os::raw::c_int,
    >,
    data: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nfc_driver {
    pub name: *const ::std::os::raw::c_char,
    pub scan_type: scan_type_enum,
    pub scan: ::std::option::Option<
        unsafe extern "C" fn(
            context: *const nfc_context,
            connstrings: *mut nfc_connstring,
            connstrings_len: usize,
        ) -> usize,
    >,
    pub open: ::std::option::Option<
        unsafe extern "C" fn(
            context: *const nfc_context,
            connstring: *mut ::std::os::raw::c_char,
        ) -> *mut nfc_device,
    >,
    pub close: ::std::option::Option<unsafe extern "C" fn(pnd: *mut nfc_device)>,
    pub strerror: ::std::option::Option<
        unsafe extern "C" fn(pnd: *const nfc_device) -> *const ::std::os::raw::c_char,
    >,
    pub initiator_init:
        ::std::option::Option<unsafe extern "C" fn(pnd: *mut nfc_device) -> ::std::os::raw::c_int>,
    pub initiator_init_secure_element:
        ::std::option::Option<unsafe extern "C" fn(pnd: *mut nfc_device) -> ::std::os::raw::c_int>,
    pub initiator_select_passive_target: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            nm: nfc_modulation,
            pbtInitData: *const u8,
            szInitData: usize,
            pnt: *mut nfc_target,
        ) -> ::std::os::raw::c_int,
    >,
    pub initiator_poll_target: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            pnmModulations: *const nfc_modulation,
            szModulations: usize,
            uiPollNr: u8,
            btPeriod: u8,
            pnt: *mut nfc_target,
        ) -> ::std::os::raw::c_int,
    >,
    pub initiator_select_dep_target: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            ndm: nfc_dep_mode,
            nbr: nfc_baud_rate,
            pndiInitiator: *const nfc_dep_info,
            pnt: *mut nfc_target,
            timeout: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub initiator_deselect_target:
        ::std::option::Option<unsafe extern "C" fn(pnd: *mut nfc_device) -> ::std::os::raw::c_int>,
    pub initiator_transceive_bytes: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            pbtTx: *const u8,
            szTx: usize,
            pbtRx: *mut u8,
            szRx: usize,
            timeout: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub initiator_transceive_bits: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            pbtTx: *const u8,
            szTxBits: usize,
            pbtTxPar: *const u8,
            pbtRx: *mut u8,
            pbtRxPar: *mut u8,
        ) -> ::std::os::raw::c_int,
    >,
    pub initiator_transceive_bytes_timed: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            pbtTx: *const u8,
            szTx: usize,
            pbtRx: *mut u8,
            szRx: usize,
            cycles: *mut u32,
        ) -> ::std::os::raw::c_int,
    >,
    pub initiator_transceive_bits_timed: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            pbtTx: *const u8,
            szTxBits: usize,
            pbtTxPar: *const u8,
            pbtRx: *mut u8,
            pbtRxPar: *mut u8,
            cycles: *mut u32,
        ) -> ::std::os::raw::c_int,
    >,
    pub initiator_target_is_present: ::std::option::Option<
        unsafe extern "C" fn(pnd: *mut nfc_device, pnt: *const nfc_target) -> ::std::os::raw::c_int,
    >,
    pub target_init: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            pnt: *mut nfc_target,
            pbtRx: *mut u8,
            szRx: usize,
            timeout: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub target_send_bytes: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            pbtTx: *const u8,
            szTx: usize,
            timeout: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub target_receive_bytes: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            pbtRx: *mut u8,
            szRxLen: usize,
            timeout: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub target_send_bits: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            pbtTx: *const u8,
            szTxBits: usize,
            pbtTxPar: *const u8,
        ) -> ::std::os::raw::c_int,
    >,
    pub target_receive_bits: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            pbtRx: *mut u8,
            szRxLen: usize,
            pbtRxPar: *mut u8,
        ) -> ::std::os::raw::c_int,
    >,
    pub device_set_property_bool: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            property: nfc_property,
            bEnable: bool,
        ) -> ::std::os::raw::c_int,
    >,
    pub device_set_property_int: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            property: nfc_property,
            value: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_supported_modulation: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            mode: nfc_mode,
            supported_mt: *mut *const nfc_modulation_type,
        ) -> ::std::os::raw::c_int,
    >,
    pub get_supported_baud_rate: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            mode: nfc_mode,
            nmt: nfc_modulation_type,
            supported_br: *mut *const nfc_baud_rate,
        ) -> ::std::os::raw::c_int,
    >,
    pub device_get_information_about: ::std::option::Option<
        unsafe extern "C" fn(
            pnd: *mut nfc_device,
            buf: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub abort_command:
        ::std::option::Option<unsafe extern "C" fn(pnd: *mut nfc_device) -> ::std::os::raw::c_int>,
    pub idle:
        ::std::option::Option<unsafe extern "C" fn(pnd: *mut nfc_device) -> ::std::os::raw::c_int>,
    pub powerdown:
        ::std::option::Option<unsafe extern "C" fn(pnd: *mut nfc_device) -> ::std::os::raw::c_int>,
}

#[test]
fn bindgen_test_layout_nfc_driver() {
    assert_eq!(
        ::std::mem::size_of::<nfc_driver>(),
        240usize,
        concat!("Size of: ", stringify!(nfc_driver))
    );
    assert_eq!(
        ::std::mem::align_of::<nfc_driver>(),
        8usize,
        concat!("Alignment of ", stringify!(nfc_driver))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).scan_type as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(scan_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).scan as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(scan)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).open as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(open)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).close as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(close)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).strerror as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(strerror)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).initiator_init as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(initiator_init)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).initiator_init_secure_element as *const _
                as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(initiator_init_secure_element)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).initiator_select_passive_target as *const _
                as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(initiator_select_passive_target)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).initiator_poll_target as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(initiator_poll_target)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).initiator_select_dep_target as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(initiator_select_dep_target)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).initiator_deselect_target as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(initiator_deselect_target)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).initiator_transceive_bytes as *const _ as usize
        },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(initiator_transceive_bytes)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).initiator_transceive_bits as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(initiator_transceive_bits)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).initiator_transceive_bytes_timed as *const _
                as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(initiator_transceive_bytes_timed)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).initiator_transceive_bits_timed as *const _
                as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(initiator_transceive_bits_timed)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).initiator_target_is_present as *const _ as usize
        },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(initiator_target_is_present)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).target_init as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(target_init)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).target_send_bytes as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(target_send_bytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).target_receive_bytes as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(target_receive_bytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).target_send_bits as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(target_send_bits)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).target_receive_bits as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(target_receive_bits)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).device_set_property_bool as *const _ as usize
        },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(device_set_property_bool)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).device_set_property_int as *const _ as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(device_set_property_int)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).get_supported_modulation as *const _ as usize
        },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(get_supported_modulation)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).get_supported_baud_rate as *const _ as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(get_supported_baud_rate)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_driver>())).device_get_information_about as *const _ as usize
        },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(device_get_information_about)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).abort_command as *const _ as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(abort_command)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).idle as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(idle)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_driver>())).powerdown as *const _ as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_driver),
            "::",
            stringify!(powerdown)
        )
    );
}

#[doc = " Connection string"]
pub type nfc_connstring = [::std::os::raw::c_char; 1024usize];
#[doc = " Default command processing timeout"]
#[doc = " Property value\'s (duration) unit is ms and 0 means no timeout (infinite)."]
#[doc = " Default value is set by driver layer"]
pub const NP_TIMEOUT_COMMAND: nfc_property = 0;
#[doc = " Timeout between ATR_REQ and ATR_RES"]
#[doc = " When the device is in initiator mode, a target is considered as mute if no"]
#[doc = " valid ATR_RES is received within this timeout value."]
#[doc = " Default value for this property is 103 ms on PN53x based devices."]
pub const NP_TIMEOUT_ATR: nfc_property = 1;
#[doc = " Timeout value to give up reception from the target in case of no answer."]
#[doc = " Default value for this property is 52 ms)."]
pub const NP_TIMEOUT_COM: nfc_property = 2;
#[doc = " Let the PN53X chip handle the CRC bytes. This means that the chip appends"]
#[doc = " the CRC bytes to the frames that are transmitted. It will parse the last"]
#[doc = " bytes from received frames as incoming CRC bytes. They will be verified"]
#[doc = " against the used modulation and protocol. If an frame is expected with"]
#[doc = " incorrect CRC bytes this option should be disabled. Example frames where"]
#[doc = " this is useful are the ATQA and UID+BCC that are transmitted without CRC"]
#[doc = " bytes during the anti-collision phase of the ISO14443-A protocol."]
pub const NP_HANDLE_CRC: nfc_property = 3;
#[doc = " Parity bits in the network layer of ISO14443-A are by default generated and"]
#[doc = " validated in the PN53X chip. This is a very convenient feature. On certain"]
#[doc = " times though it is useful to get full control of the transmitted data. The"]
#[doc = " proprietary MIFARE Classic protocol uses for example custom (encrypted)"]
#[doc = " parity bits. For interoperability it is required to be completely"]
#[doc = " compatible, including the arbitrary parity bits. When this option is"]
#[doc = " disabled, the functions to communicating bits should be used."]
pub const NP_HANDLE_PARITY: nfc_property = 4;
#[doc = " This option can be used to enable or disable the electronic field of the"]
#[doc = " NFC device."]
pub const NP_ACTIVATE_FIELD: nfc_property = 5;
#[doc = " The internal CRYPTO1 co-processor can be used to transmit messages"]
#[doc = " encrypted. This option is automatically activated after a successful MIFARE"]
#[doc = " Classic authentication."]
pub const NP_ACTIVATE_CRYPTO1: nfc_property = 6;
#[doc = " The default configuration defines that the PN53X chip will try indefinitely"]
#[doc = " to invite a tag in the field to respond. This could be desired when it is"]
#[doc = " certain a tag will enter the field. On the other hand, when this is"]
#[doc = " uncertain, it will block the application. This option could best be compared"]
#[doc = " to the (NON)BLOCKING option used by (socket)network programming."]
pub const NP_INFINITE_SELECT: nfc_property = 7;
#[doc = " If this option is enabled, frames that carry less than 4 bits are allowed."]
#[doc = " According to the standards these frames should normally be handles as"]
#[doc = " invalid frames."]
pub const NP_ACCEPT_INVALID_FRAMES: nfc_property = 8;
#[doc = " If the NFC device should only listen to frames, it could be useful to let"]
#[doc = " it gather multiple frames in a sequence. They will be stored in the internal"]
#[doc = " FIFO of the PN53X chip. This could be retrieved by using the receive data"]
#[doc = " functions. Note that if the chip runs out of bytes (FIFO = 64 bytes long),"]
#[doc = " it will overwrite the first received frames, so quick retrieving of the"]
#[doc = " received data is desirable."]
pub const NP_ACCEPT_MULTIPLE_FRAMES: nfc_property = 9;
#[doc = " This option can be used to enable or disable the auto-switching mode to"]
#[doc = " ISO14443-4 if device is compliant. In initiator mode, it means that "]
#[doc = " NFC chip will send RATS automatically when in select and it will automatically"]
#[doc = " poll for ISO14443-4 card when ISO14443A is requested."]
#[doc = " In target mode, with a NFC chip compliant (ie. PN532), the chip will emulate"]
#[doc = " a 14443-4 PICC using hardware capability."]
pub const NP_AUTO_ISO14443_4: nfc_property = 10;
#[doc = " Use automatic frames encapsulation and chaining."]
pub const NP_EASY_FRAMING: nfc_property = 11;
#[doc = " Force the chip to switch in ISO14443-A"]
pub const NP_FORCE_ISO14443_A: nfc_property = 12;
#[doc = " Force the chip to switch in ISO14443-B"]
pub const NP_FORCE_ISO14443_B: nfc_property = 13;
#[doc = " Force the chip to run at 106 kbps"]
pub const NP_FORCE_SPEED_106: nfc_property = 14;
#[doc = " Properties"]
pub type nfc_property = i32;

pub const NDM_UNDEFINED: nfc_dep_mode = 0;
pub const NDM_PASSIVE: nfc_dep_mode = 1;
pub const NDM_ACTIVE: nfc_dep_mode = 2;
#[doc = " @enum nfc_dep_mode"]
#[doc = " @brief NFC D.E.P. (Data Exchange Protocol) active/passive mode"]
pub type nfc_dep_mode = i32;

pub const ST_NOT_INTRUSIVE: scan_type_enum = 0;
pub const ST_INTRUSIVE: scan_type_enum = 1;
pub const ST_NOT_AVAILABLE: scan_type_enum = 2;
pub type scan_type_enum = i32;

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum pn532_sam_mode {
    PSM_NORMAL = 0x01,
    PSM_VIRTUAL_CARD = 0x02,
    PSM_WIRED_CARD = 0x03,
    PSM_DUAL_CARD = 0x04,
}

#[doc = " @struct nfc_dep_info"]
#[doc = " @brief NFC target information in D.E.P. (Data Exchange Protocol) see ISO/IEC 18092 (NFCIP-1)"]
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_dep_info {
    #[doc = " NFCID3"]
    pub abtNFCID3: [u8; 10usize],
    #[doc = " DID"]
    pub btDID: u8,
    #[doc = " Supported send-bit rate"]
    pub btBS: u8,
    #[doc = " Supported receive-bit rate"]
    pub btBR: u8,
    #[doc = " Timeout value"]
    pub btTO: u8,
    #[doc = " PP Parameters"]
    pub btPP: u8,
    #[doc = " General Bytes"]
    pub abtGB: [u8; 48usize],
    pub szGB: usize,
    #[doc = " DEP mode"]
    pub ndm: nfc_dep_mode,
}

impl ::std::default::Default for nfc_dep_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[test]
fn bindgen_test_layout_nfc_dep_info() {
    assert_eq!(
        ::std::mem::size_of::<nfc_dep_info>(),
        75usize,
        concat!("Size of: ", stringify!(nfc_dep_info))
    );
    assert_eq!(
        ::std::mem::align_of::<nfc_dep_info>(),
        1usize,
        concat!("Alignment of ", stringify!(nfc_dep_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_dep_info>())).abtNFCID3 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_dep_info),
            "::",
            stringify!(abtNFCID3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_dep_info>())).btDID as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_dep_info),
            "::",
            stringify!(btDID)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_dep_info>())).btBS as *const _ as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_dep_info),
            "::",
            stringify!(btBS)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_dep_info>())).btBR as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_dep_info),
            "::",
            stringify!(btBR)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_dep_info>())).btTO as *const _ as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_dep_info),
            "::",
            stringify!(btTO)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_dep_info>())).btPP as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_dep_info),
            "::",
            stringify!(btPP)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_dep_info>())).abtGB as *const _ as usize },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_dep_info),
            "::",
            stringify!(abtGB)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_dep_info>())).szGB as *const _ as usize },
        63usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_dep_info),
            "::",
            stringify!(szGB)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_dep_info>())).ndm as *const _ as usize },
        71usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_dep_info),
            "::",
            stringify!(ndm)
        )
    );
}

#[doc = " @struct nfc_iso14443a_info"]
#[doc = " @brief NFC ISO14443A tag (MIFARE) information"]
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_iso14443a_info {
    pub abtAtqa: [u8; 2usize],
    pub btSak: u8,
    pub szUidLen: usize,
    pub abtUid: [u8; 10usize],
    pub szAtsLen: usize,
    pub abtAts: [u8; 254usize],
}

impl ::std::default::Default for nfc_iso14443a_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[test]
fn bindgen_test_layout_nfc_iso14443a_info() {
    assert_eq!(
        ::std::mem::size_of::<nfc_iso14443a_info>(),
        283usize,
        concat!("Size of: ", stringify!(nfc_iso14443a_info))
    );
    assert_eq!(
        ::std::mem::align_of::<nfc_iso14443a_info>(),
        1usize,
        concat!("Alignment of ", stringify!(nfc_iso14443a_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443a_info>())).abtAtqa as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443a_info),
            "::",
            stringify!(abtAtqa)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443a_info>())).btSak as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443a_info),
            "::",
            stringify!(btSak)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443a_info>())).szUidLen as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443a_info),
            "::",
            stringify!(szUidLen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443a_info>())).abtUid as *const _ as usize },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443a_info),
            "::",
            stringify!(abtUid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443a_info>())).szAtsLen as *const _ as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443a_info),
            "::",
            stringify!(szAtsLen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443a_info>())).abtAts as *const _ as usize },
        29usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443a_info),
            "::",
            stringify!(abtAts)
        )
    );
}

#[doc = " @struct nfc_felica_info"]
#[doc = " @brief NFC FeLiCa tag information"]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct nfc_felica_info {
    pub szLen: usize,
    pub btResCode: u8,
    pub abtId: [u8; 8usize],
    pub abtPad: [u8; 8usize],
    pub abtSysCode: [u8; 2usize],
}
impl ::std::default::Default for nfc_felica_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[test]
fn bindgen_test_layout_nfc_felica_info() {
    assert_eq!(
        ::std::mem::size_of::<nfc_felica_info>(),
        27usize,
        concat!("Size of: ", stringify!(nfc_felica_info))
    );
    assert_eq!(
        ::std::mem::align_of::<nfc_felica_info>(),
        1usize,
        concat!("Alignment of ", stringify!(nfc_felica_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_felica_info>())).szLen as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_felica_info),
            "::",
            stringify!(szLen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_felica_info>())).btResCode as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_felica_info),
            "::",
            stringify!(btResCode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_felica_info>())).abtId as *const _ as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_felica_info),
            "::",
            stringify!(abtId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_felica_info>())).abtPad as *const _ as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_felica_info),
            "::",
            stringify!(abtPad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_felica_info>())).abtSysCode as *const _ as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_felica_info),
            "::",
            stringify!(abtSysCode)
        )
    );
}

#[doc = " @struct nfc_iso14443b_info"]
#[doc = " @brief NFC ISO14443B tag information"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nfc_iso14443b_info {
    #[doc = " abtPupi store PUPI contained in ATQB (Answer To reQuest of type B) (see ISO14443-3)"]
    pub abtPupi: [u8; 4usize],
    #[doc = " abtApplicationData store Application Data contained in ATQB (see ISO14443-3)"]
    pub abtApplicationData: [u8; 4usize],
    #[doc = " abtProtocolInfo store Protocol Info contained in ATQB (see ISO14443-3)"]
    pub abtProtocolInfo: [u8; 3usize],
    #[doc = " ui8CardIdentifier store CID (Card Identifier) attributted by PCD to the PICC"]
    pub ui8CardIdentifier: u8,
}

impl ::std::default::Default for nfc_iso14443b_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[test]
fn bindgen_test_layout_nfc_iso14443b_info() {
    assert_eq!(
        ::std::mem::size_of::<nfc_iso14443b_info>(),
        12usize,
        concat!("Size of: ", stringify!(nfc_iso14443b_info))
    );
    assert_eq!(
        ::std::mem::align_of::<nfc_iso14443b_info>(),
        1usize,
        concat!("Alignment of ", stringify!(nfc_iso14443b_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443b_info>())).abtPupi as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443b_info),
            "::",
            stringify!(abtPupi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_iso14443b_info>())).abtApplicationData as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443b_info),
            "::",
            stringify!(abtApplicationData)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_iso14443b_info>())).abtProtocolInfo as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443b_info),
            "::",
            stringify!(abtProtocolInfo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_iso14443b_info>())).ui8CardIdentifier as *const _ as usize
        },
        11usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443b_info),
            "::",
            stringify!(ui8CardIdentifier)
        )
    );
}

#[doc = " @struct nfc_iso14443bi_info"]
#[doc = " @brief NFC ISO14443B\' tag information"]
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct nfc_iso14443bi_info {
    #[doc = " DIV: 4 LSBytes of tag serial number"]
    pub abtDIV: [u8; 4usize],
    #[doc = " Software version & type of REPGEN"]
    pub btVerLog: u8,
    #[doc = " Config Byte, present if long REPGEN"]
    pub btConfig: u8,
    #[doc = " ATR, if any"]
    pub szAtrLen: usize,
    pub abtAtr: [u8; 33usize],
}

impl ::std::default::Default for nfc_iso14443bi_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[test]
fn bindgen_test_layout_nfc_iso14443bi_info() {
    assert_eq!(
        ::std::mem::size_of::<nfc_iso14443bi_info>(),
        47usize,
        concat!("Size of: ", stringify!(nfc_iso14443bi_info))
    );
    assert_eq!(
        ::std::mem::align_of::<nfc_iso14443bi_info>(),
        1usize,
        concat!("Alignment of ", stringify!(nfc_iso14443bi_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443bi_info>())).abtDIV as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443bi_info),
            "::",
            stringify!(abtDIV)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443bi_info>())).btVerLog as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443bi_info),
            "::",
            stringify!(btVerLog)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443bi_info>())).btConfig as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443bi_info),
            "::",
            stringify!(btConfig)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443bi_info>())).szAtrLen as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443bi_info),
            "::",
            stringify!(szAtrLen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443bi_info>())).abtAtr as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443bi_info),
            "::",
            stringify!(abtAtr)
        )
    );
}

#[doc = " @struct nfc_iso14443b2sr_info"]
#[doc = " @brief NFC ISO14443-2B ST SRx tag information"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nfc_iso14443b2sr_info {
    pub abtUID: [u8; 8usize],
}

impl ::std::default::Default for nfc_iso14443b2sr_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[test]
fn bindgen_test_layout_nfc_iso14443b2sr_info() {
    assert_eq!(
        ::std::mem::size_of::<nfc_iso14443b2sr_info>(),
        8usize,
        concat!("Size of: ", stringify!(nfc_iso14443b2sr_info))
    );
    assert_eq!(
        ::std::mem::align_of::<nfc_iso14443b2sr_info>(),
        1usize,
        concat!("Alignment of ", stringify!(nfc_iso14443b2sr_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443b2sr_info>())).abtUID as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443b2sr_info),
            "::",
            stringify!(abtUID)
        )
    );
}

#[doc = " @struct nfc_iso14443b2ct_info"]
#[doc = " @brief NFC ISO14443-2B ASK CTx tag information"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nfc_iso14443b2ct_info {
    pub abtUID: [u8; 4usize],
    pub btProdCode: u8,
    pub btFabCode: u8,
}

impl ::std::default::Default for nfc_iso14443b2ct_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[test]
fn bindgen_test_layout_nfc_iso14443b2ct_info() {
    assert_eq!(
        ::std::mem::size_of::<nfc_iso14443b2ct_info>(),
        6usize,
        concat!("Size of: ", stringify!(nfc_iso14443b2ct_info))
    );
    assert_eq!(
        ::std::mem::align_of::<nfc_iso14443b2ct_info>(),
        1usize,
        concat!("Alignment of ", stringify!(nfc_iso14443b2ct_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443b2ct_info>())).abtUID as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443b2ct_info),
            "::",
            stringify!(abtUID)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<nfc_iso14443b2ct_info>())).btProdCode as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443b2ct_info),
            "::",
            stringify!(btProdCode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_iso14443b2ct_info>())).btFabCode as *const _ as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_iso14443b2ct_info),
            "::",
            stringify!(btFabCode)
        )
    );
}

#[doc = " @struct nfc_jewel_info"]
#[doc = " @brief NFC Jewel tag information"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nfc_jewel_info {
    pub btSensRes: [u8; 2usize],
    pub btId: [u8; 4usize],
}

impl ::std::default::Default for nfc_jewel_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[test]
fn bindgen_test_layout_nfc_jewel_info() {
    assert_eq!(
        ::std::mem::size_of::<nfc_jewel_info>(),
        6usize,
        concat!("Size of: ", stringify!(nfc_jewel_info))
    );
    assert_eq!(
        ::std::mem::align_of::<nfc_jewel_info>(),
        1usize,
        concat!("Alignment of ", stringify!(nfc_jewel_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_jewel_info>())).btSensRes as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_jewel_info),
            "::",
            stringify!(btSensRes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_jewel_info>())).btId as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_jewel_info),
            "::",
            stringify!(btId)
        )
    );
}

#[doc = " @struct nfc_barcode_info"]
#[doc = " @brief Thinfilm NFC Barcode information"]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct nfc_barcode_info {
    pub szDataLen: usize,
    pub abtData: [u8; 32usize],
}

#[test]
fn bindgen_test_layout_nfc_barcode_info() {
    assert_eq!(
        ::std::mem::size_of::<nfc_barcode_info>(),
        40usize,
        concat!("Size of: ", stringify!(nfc_barcode_info))
    );
    assert_eq!(
        ::std::mem::align_of::<nfc_barcode_info>(),
        1usize,
        concat!("Alignment of ", stringify!(nfc_barcode_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_barcode_info>())).szDataLen as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_barcode_info),
            "::",
            stringify!(szDataLen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_barcode_info>())).abtData as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_barcode_info),
            "::",
            stringify!(abtData)
        )
    );
}

#[doc = " @union nfc_target_info"]
#[doc = " @brief Union between all kind of tags information structures."]
#[repr(C)]
#[derive(Copy, Clone)]
pub union nfc_target_info {
    /*pub nai: nfc_iso14443a_info,
    pub nfi: nfc_felica_info,
    pub nbi: nfc_iso14443b_info,
    pub nii: nfc_iso14443bi_info,
    pub nsi: nfc_iso14443b2sr_info,
    pub nci: nfc_iso14443b2ct_info,
    pub nji: nfc_jewel_info,
    pub nti: nfc_barcode_info,
    pub ndi: nfc_dep_info,*/
    _bindgen_data_: [u8; 283usize],
}

impl nfc_target_info {
    pub unsafe fn nai(&mut self) -> *mut nfc_iso14443a_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }

    pub unsafe fn nbi(&mut self) -> *mut nfc_iso14443b_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nii(&mut self) -> *mut nfc_iso14443bi_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nsi(&mut self) -> *mut nfc_iso14443b2sr_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nci(&mut self) -> *mut nfc_iso14443b2ct_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }

    pub unsafe fn nfi(&mut self) -> *mut nfc_felica_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }

    pub unsafe fn nji(&mut self) -> *mut nfc_jewel_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }

    pub unsafe fn nti(&mut self) -> *mut nfc_barcode_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }

    pub unsafe fn ndi(&mut self) -> *mut nfc_dep_info {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}

impl ::std::default::Default for nfc_target_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub const NBR_UNDEFINED: nfc_baud_rate = 0;
pub const NBR_106: nfc_baud_rate = 1;
pub const NBR_212: nfc_baud_rate = 2;
pub const NBR_424: nfc_baud_rate = 3;
pub const NBR_847: nfc_baud_rate = 4;
#[doc = " @enum nfc_baud_rate"]
#[doc = " @brief NFC baud rate enumeration"]
pub type nfc_baud_rate = i32;
pub const NMT_ISO14443A: nfc_modulation_type = 1;
pub const NMT_JEWEL: nfc_modulation_type = 2;
pub const NMT_BARCODE: nfc_modulation_type = 3;
pub const NMT_ISO14443B: nfc_modulation_type = 4;
pub const NMT_ISO14443BI: nfc_modulation_type = 5;
pub const NMT_ISO14443B2SR: nfc_modulation_type = 6;
pub const NMT_ISO14443B2CT: nfc_modulation_type = 7;
pub const NMT_FELICA: nfc_modulation_type = 8;
pub const NMT_DEP: nfc_modulation_type = 9;
#[doc = " @enum nfc_modulation_type"]
#[doc = " @brief NFC modulation type enumeration"]
pub type nfc_modulation_type = i32;
pub const N_TARGET: nfc_mode = 0;
pub const N_INITIATOR: nfc_mode = 1;
#[doc = " @enum nfc_mode"]
#[doc = " @brief NFC mode type enumeration"]
pub type nfc_mode = i32;

#[doc = " @struct nfc_modulation"]
#[doc = " @brief NFC modulation structure"]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct nfc_modulation {
    pub nmt: nfc_modulation_type,
    pub nbr: nfc_baud_rate,
}

impl ::std::default::Default for nfc_modulation {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[test]
fn bindgen_test_layout_nfc_modulation() {
    assert_eq!(
        ::std::mem::size_of::<nfc_modulation>(),
        8usize,
        concat!("Size of: ", stringify!(nfc_modulation))
    );
    assert_eq!(
        ::std::mem::align_of::<nfc_modulation>(),
        1usize,
        concat!("Alignment of ", stringify!(nfc_modulation))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_modulation>())).nmt as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_modulation),
            "::",
            stringify!(nmt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_modulation>())).nbr as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_modulation),
            "::",
            stringify!(nbr)
        )
    );
}
#[doc = " @struct nfc_target"]
#[doc = " @brief NFC target structure"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfc_target {
    pub nti: nfc_target_info,
    pub nm: nfc_modulation,
}

impl ::std::default::Default for nfc_target {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[test]
fn bindgen_test_layout_nfc_target() {
    assert_eq!(
        ::std::mem::size_of::<nfc_target>(),
        291usize,
        concat!("Size of: ", stringify!(nfc_target))
    );
    assert_eq!(
        ::std::mem::align_of::<nfc_target>(),
        1usize,
        concat!("Alignment of ", stringify!(nfc_target))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_target>())).nti as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_target),
            "::",
            stringify!(nti)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nfc_target>())).nm as *const _ as usize },
        283usize,
        concat!(
            "Offset of field: ",
            stringify!(nfc_target),
            "::",
            stringify!(nm)
        )
    );
}

#[cfg_attr(target_os = "windows", link(name = "libnfc"))]
extern "C" {
    /* Library initialization/deinitialization */

    pub fn nfc_init(context: *mut *mut nfc_context);

    pub fn nfc_exit(context: *mut nfc_context);

    pub fn nfc_register_driver(driver: *const nfc_driver) -> ::std::os::raw::c_int;

    /* NFC Device/Hardware manipulation */

    // wrapper need to cast return type to *mut nfc_device
    pub fn nfc_open(
        context: *mut nfc_context,
        connstring: *const ::std::os::raw::c_char,
    ) -> *mut nfc_device;

    // wrapper need to cast pnd to *mut nfc_device
    pub fn nfc_close(pnd: *mut nfc_device);

    pub fn nfc_abort_command(pnd: *mut nfc_device) -> ::std::os::raw::c_int;

    pub fn nfc_list_devices(
        context: *mut nfc_context,
        connstrings: *mut nfc_connstring,
        connstrings_len: usize,
    ) -> usize;

    // wrapper need to cast pnd to *mut nfc_device
    pub fn nfc_idle(pnd: *mut nfc_device) -> ::std::os::raw::c_int;

    pub fn nfc_initiator_init(pnd: *mut nfc_device) -> ::std::os::raw::c_int;

    pub fn nfc_initiator_init_secure_element(pnd: *mut nfc_device) -> ::std::os::raw::c_int;

    pub fn nfc_initiator_select_passive_target(
        pnd: *mut nfc_device,
        nm: nfc_modulation,
        pbtInitData: *const u8,
        szInitData: usize,
        pnt: *mut nfc_target,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_initiator_list_passive_targets(
        pnd: *mut nfc_device,
        nm: nfc_modulation,
        ant: *mut nfc_target,
        szTargets: usize,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_initiator_poll_target(
        pnd: *mut nfc_device,
        pnmTargetTypes: *const nfc_modulation,
        szTargetTypes: usize,
        uiPollNr: u8,
        uiPeriod: u8,
        pnt: *mut nfc_target,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_initiator_select_dep_target(
        pnd: *mut nfc_device,
        ndm: nfc_dep_mode,
        nbr: nfc_baud_rate,
        pndiInitiator: *const nfc_dep_info,
        pnt: *mut nfc_target,
        timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_initiator_poll_dep_target(
        pnd: *mut nfc_device,
        ndm: nfc_dep_mode,
        nbr: nfc_baud_rate,
        pndiInitiator: *const nfc_dep_info,
        pnt: *mut nfc_target,
        timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_initiator_deselect_target(pnd: *mut nfc_device) -> ::std::os::raw::c_int;

    pub fn nfc_initiator_transceive_bytes(
        pnd: *mut nfc_device,
        pbtTx: *const u8,
        szTx: usize,
        pbtRx: *mut u8,
        szRx: usize,
        timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_initiator_transceive_bits(
        pnd: *mut nfc_device,
        pbtTx: *const u8,
        szTxBits: usize,
        pbtTxPar: *const u8,
        pbtRx: *mut u8,
        szRx: usize,
        pbtRxPar: *mut u8,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_initiator_transceive_bytes_timed(
        pnd: *mut nfc_device,
        pbtTx: *const u8,
        szTx: usize,
        pbtRx: *mut u8,
        szRx: usize,
        cycles: *mut u32,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_initiator_transceive_bits_timed(
        pnd: *mut nfc_device,
        pbtTx: *const u8,
        szTxBits: usize,
        pbtTxPar: *const u8,
        pbtRx: *mut u8,
        szRx: usize,
        pbtRxPar: *mut u8,
        cycles: *mut u32,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_initiator_target_is_present(
        pnd: *mut nfc_device,
        pnt: *const nfc_target,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_target_init(
        pnd: *mut nfc_device,
        pnt: *mut nfc_target,
        pbtRx: *mut u8,
        szRx: usize,
        timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_emulate_target(
        pnd: *mut nfc_device,
        emulator: *mut nfc_emulator,
        timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_target_send_bytes(
        pnd: *mut nfc_device,
        pbtTx: *const u8,
        szTx: usize,
        timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_target_receive_bytes(
        pnd: *mut nfc_device,
        pbtRx: *mut u8,
        szRx: usize,
        timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_target_send_bits(
        pnd: *mut nfc_device,
        pbtTx: *const u8,
        szTxBits: usize,
        pbtTxPar: *const u8,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_target_receive_bits(
        pnd: *mut nfc_device,
        pbtRx: *mut u8,
        szRx: usize,
        pbtRxPar: *mut u8,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_strerror(pnd: *const nfc_device) -> *const ::std::os::raw::c_char;

    pub fn nfc_strerror_r(
        pnd: *const nfc_device,
        buf: *mut ::std::os::raw::c_char,
        buflen: usize,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_perror(pnd: *const nfc_device, s: *const ::std::os::raw::c_char);

    pub fn nfc_device_get_last_error(pnd: *const nfc_device) -> ::std::os::raw::c_int;

    pub fn nfc_device_get_name(pnd: *mut nfc_device) -> *const ::std::os::raw::c_char;

    pub fn nfc_device_get_connstring(pnd: *mut nfc_device) -> *const ::std::os::raw::c_char;

    pub fn nfc_device_get_supported_modulation(
        pnd: *mut nfc_device,
        mode: nfc_mode,
        supported_mt: *mut *const nfc_modulation_type,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_device_get_supported_baud_rate(
        pnd: *mut nfc_device,
        nmt: nfc_modulation_type,
        supported_br: *mut *const nfc_baud_rate,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_device_get_supported_baud_rate_target_mode(
        pnd: *mut nfc_device,
        nmt: nfc_modulation_type,
        supported_br: *mut *const nfc_baud_rate,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_device_set_property_int(
        pnd: *mut nfc_device,
        property: nfc_property,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn nfc_device_set_property_bool(
        pnd: *mut nfc_device,
        property: nfc_property,
        bEnable: bool,
    ) -> ::std::os::raw::c_int;

    pub fn iso14443a_crc(pbtData: *mut u8, szLen: usize, pbtCrc: *mut u8);

    pub fn iso14443a_crc_append(pbtData: *mut u8, szLen: usize);

    pub fn iso14443b_crc(pbtData: *mut u8, szLen: usize, pbtCrc: *mut u8);

    pub fn iso14443b_crc_append(pbtData: *mut u8, szLen: usize);

    pub fn iso14443a_locate_historical_bytes(
        pbtAts: *mut u8,
        szAts: usize,
        pszTk: *mut usize,
    ) -> *mut u8;

    pub fn nfc_free(p: *mut ::std::os::raw::c_void);

    pub fn nfc_version() -> *const ::std::os::raw::c_char;

    pub fn nfc_device_get_information_about(
        pnd: *mut nfc_device,
        buf: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn str_nfc_modulation_type(nmt: nfc_modulation_type) -> *const ::std::os::raw::c_char;

    pub fn str_nfc_baud_rate(nbr: nfc_baud_rate) -> *const ::std::os::raw::c_char;

    pub fn str_nfc_target(
        buf: *mut *mut ::std::os::raw::c_char,
        pnt: *const nfc_target,
        verbose: bool,
    ) -> ::std::os::raw::c_int;

    pub fn pn532_SAMConfiguration(
        pnd: *mut nfc_device,
        mode: pn532_sam_mode,
        timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn pn53x_read_register(
        pnd: *mut nfc_device,
        ui16Reg: u16,
        ui8Value: *mut u8,
    ) -> ::std::os::raw::c_int;

    pub fn pn53x_transceive(
        pnd: *mut nfc_device,
        pbtTx: *const u8,
        szTx: usize,
        pbtRx: *mut u8,
        szRxLen: usize,
        timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn pn53x_write_register(
        pnd: *mut nfc_device,
        ui16Reg: u16,
        ui8SymbolMask: u8,
        ui8Value: u8,
    ) -> ::std::os::raw::c_int;
}
