//#![no_std]
pub mod config;
pub mod measurement;

/// Messages Overview
/// Description         Default Length  Remark
///                     CAN-ID  DLC
/// IVT_Msg_Command     0x411   8       Function commands, SET and GET commandsA command-ID-byte is included for identificationi
/// IVT_Msg_Debug       0x510   8       Message only for internal use
/// IVT_Msg_Response    0x511   8       Response to SET and GET command messagesA response-ID-byte is included for identification
/// IVT_Msg_Result_I    0x521   6       Current
/// IVT_Msg_Result_U1   0x522   6   Voltage 1
/// IVT_Msg_Result_U2   0x523   6   Voltage 2
/// IVT_Msg_Result_U3   0x524   6   Voltage 3
/// IVT_Msg_Result_T    0x525   6   Temperature
/// IVT_Msg_Result_W    0x526   6   Power(referring to current and voltage U1)
/// IVT_Msg_Result_As   0x527   6   Current counter
/// IVT_Msg_Result_Wh   0x528   6   Energy counter(referring to current and voltage U1)
/// * Not used bytes in response messages are undefined and reported as 0x00.
/// * Not used / undefined bytes in command messages must be set to 0x00.
/// * Each defined command will report its response message even if there was no change done or is currently not allowed (e.g. set configuration during run mode). This is done to give acknowledge toi the sender.
/// * Consecutive commands must be sent not faster than 2ms, or you can wait until the related responseis sent.
/// * Response messages must be available on the bus (free bus) at least +500ms after the relatedcommand, if not otherwise specified.
/// * If not otherwise mentioned byte orders are Big Endian.i

/// Multiplexable Messages
/// All Messages sent by the IVT shall be unique identifiable by the first databyte sent as muxbyte.
/// DB0 (Muxbyte)   Remark
/// 0x0n            Results (measured or calculated)
/// 0x1n            Set CAN ID
/// 0x2n            Set config result
/// 0x3n            Set commands
/// 0x4n            Get error/log data
/// 0x5n            Get CAN ID
/// 0x6n            Get configresult
/// 0x7n            Get commands
/// 0x8n            Response on error/log data
/// 0x9n            Responses on CAN ID
/// 0xAn            Responses on Config Result
/// 0xBn            Responses on Set and Get Comma
/// 0xFF            Response on not allowed message

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
