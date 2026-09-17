// Copyright 2026 Metaversal Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! ABI constants, mirrored from `sdk/include/sneeze_abi.h` (the single source of
//! truth). Keep the two in lockstep: a `wMethod` number is permanent, monotonic,
//! and append-only.

pub const SNEEZE_ABI_VERSION                                : u32 = 1;

// wType - subsystem registry.
pub const kSNEEZE_ABI_TYPE_DATA                             : u16 =  1;
pub const kSNEEZE_ABI_TYPE_CONSOLE                          : u16 =  2;
pub const kSNEEZE_ABI_TYPE_STORAGE                          : u16 =  3;
pub const kSNEEZE_ABI_TYPE_NETWORK                          : u16 =  4;
pub const kSNEEZE_ABI_TYPE_VIEWPORT                         : u16 =  5;
pub const kSNEEZE_ABI_TYPE_SCENE                            : u16 =  6;
pub const kSNEEZE_ABI_TYPE_FABRIC                           : u16 =  7;
pub const kSNEEZE_ABI_TYPE_NODE                             : u16 =  8;
pub const kSNEEZE_ABI_TYPE_CHRONO                           : u16 =  9;
pub const kSNEEZE_ABI_TYPE_PERFORMANCE                      : u16 = 10;
pub const kSNEEZE_ABI_TYPE_TIMER                            : u16 = 11;
pub const kSNEEZE_ABI_TYPE_SERVICES                         : u16 = 12;

// CONSOLE methods.
pub const kSNEEZE_ABI_METHOD_CONSOLE_LOG                    : u16 = 1;
pub const kSNEEZE_ABI_METHOD_CONSOLE_DEBUG                  : u16 = 2;
pub const kSNEEZE_ABI_METHOD_CONSOLE_INFO                   : u16 = 3;
pub const kSNEEZE_ABI_METHOD_CONSOLE_WARN                   : u16 = 4;
pub const kSNEEZE_ABI_METHOD_CONSOLE_ERROR                  : u16 = 5;
pub const kSNEEZE_ABI_METHOD_CONSOLE_ASSERT                 : u16 = 6;
pub const kSNEEZE_ABI_METHOD_CONSOLE_GROUP                  : u16 = 7;
pub const kSNEEZE_ABI_METHOD_CONSOLE_GROUP_COLLAPSED        : u16 = 8;
pub const kSNEEZE_ABI_METHOD_CONSOLE_GROUP_END              : u16 = 9;
pub const kSNEEZE_ABI_METHOD_CONSOLE_COUNT                  : u16 = 10;
pub const kSNEEZE_ABI_METHOD_CONSOLE_COUNT_RESET            : u16 = 11;
pub const kSNEEZE_ABI_METHOD_CONSOLE_TIME                   : u16 = 12;
pub const kSNEEZE_ABI_METHOD_CONSOLE_TIME_END               : u16 = 13;
pub const kSNEEZE_ABI_METHOD_CONSOLE_TIME_LOG               : u16 = 14;

// STORAGE methods.
pub const kSNEEZE_ABI_METHOD_STORAGE_HAS                    : u16 = 1;
pub const kSNEEZE_ABI_METHOD_STORAGE_GET                    : u16 = 2;
pub const kSNEEZE_ABI_METHOD_STORAGE_SET                    : u16 = 3;
pub const kSNEEZE_ABI_METHOD_STORAGE_REMOVE                 : u16 = 4;

// NETWORK methods. Two contiguous blocks: REQUEST (an XHR-shaped HTTP request)
// holds 1-29, SOCKET (a browser-shaped WebSocket) holds 30-59. Within each block
// the numbers past the calls are host -> guest Notify events, not calls: REQUEST
// COMPLETED and PROGRESS, and SOCKET OPENED, RECEIVED, FAILED and CLOSED.
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_OPEN           : u16 =  1;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_HEADER_SET     : u16 =  2;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_TIMEOUT_SET    : u16 =  3;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_SEND           : u16 =  4;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_ABORT          : u16 =  5;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_CLOSE          : u16 =  6;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_STATE          : u16 =  7;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_STATUS         : u16 =  8;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_STATUS_TEXT    : u16 =  9;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_URL            : u16 = 10;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_HEADER_GET     : u16 = 11;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_HEADER_ALL     : u16 = 12;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_BODY           : u16 = 13;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_SIZE           : u16 = 14;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_CONTENT_TYPE   : u16 = 15;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_ERROR          : u16 = 16;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_IS_CACHED      : u16 = 17;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_COMPLETED      : u16 = 18;
pub const kSNEEZE_ABI_METHOD_NETWORK_REQUEST_PROGRESS       : u16 = 19;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_OPEN            : u16 = 30;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_SEND_TEXT       : u16 = 31;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_SEND_BINARY     : u16 = 32;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_CLOSE           : u16 = 33;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_STATE           : u16 = 34;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_BUFFERED        : u16 = 35;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_PROTOCOL        : u16 = 36;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_URL             : u16 = 37;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_RECV            : u16 = 38;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_ERROR           : u16 = 39;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_OPENED          : u16 = 40;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_RECEIVED        : u16 = 41;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_FAILED          : u16 = 42;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_CLOSED          : u16 = 43;
pub const kSNEEZE_ABI_METHOD_NETWORK_SOCKET_FREE            : u16 = 44;

// VIEWPORT methods (not implemented yet host-side).
pub const kSNEEZE_ABI_METHOD_VIEWPORT_POSITION_GET          : u16 = 1;
pub const kSNEEZE_ABI_METHOD_VIEWPORT_POSITION_SET          : u16 = 2;
pub const kSNEEZE_ABI_METHOD_VIEWPORT_ROTATION_GET          : u16 = 3;
pub const kSNEEZE_ABI_METHOD_VIEWPORT_ROTATION_SET          : u16 = 4;

// SCENE methods. The node-tree calls are DEPRECATED - use the FABRIC subsystem
// instead; they remain only so already-deployed modules keep working. The scene
// globals (ambient/directional/background) are parked until Stage 4 renumbers
// this enum from 1 after the deprecated methods are removed.
pub const kSNEEZE_ABI_METHOD_SCENE_NODE_ROOT                : u16 = 1;    // DEPRECATED - use kSNEEZE_ABI_METHOD_FABRIC_NODE_ROOT
pub const kSNEEZE_ABI_METHOD_SCENE_NODE_MAP_DATA            : u16 = 2;    // DEPRECATED - use kSNEEZE_ABI_METHOD_FABRIC_NODE_MAP_DATA
pub const kSNEEZE_ABI_METHOD_SCENE_NODE_OPEN                : u16 = 3;    // DEPRECATED - use kSNEEZE_ABI_METHOD_FABRIC_NODE_OPEN
pub const kSNEEZE_ABI_METHOD_SCENE_NODE_CLOSE               : u16 = 4;    // DEPRECATED - use kSNEEZE_ABI_METHOD_FABRIC_NODE_CLOSE

// FABRIC methods (type 7) - the node-tree API. Order is deliberate: the two
// map-service calls first (browser-assigned mapping), then the four guest-
// assigned node-tree calls.
pub const kSNEEZE_ABI_METHOD_FABRIC_NODE_MAP_SERVICE        : u16 = 1;
pub const kSNEEZE_ABI_METHOD_FABRIC_NODE_MAP_SERVICE_EX     : u16 = 2;
pub const kSNEEZE_ABI_METHOD_FABRIC_NODE_MAP_DATA           : u16 = 3;
pub const kSNEEZE_ABI_METHOD_FABRIC_NODE_ROOT               : u16 = 4;
pub const kSNEEZE_ABI_METHOD_FABRIC_NODE_OPEN               : u16 = 5;
pub const kSNEEZE_ABI_METHOD_FABRIC_NODE_CLOSE              : u16 = 6;

// NODE methods.
pub const kSNEEZE_ABI_METHOD_NODE_POSITION                  : u16 = 1;
pub const kSNEEZE_ABI_METHOD_NODE_ROTATION                  : u16 = 2;
pub const kSNEEZE_ABI_METHOD_NODE_SCALE                     : u16 = 3;
pub const kSNEEZE_ABI_METHOD_NODE_SCALE_AXES                : u16 = 4;
pub const kSNEEZE_ABI_METHOD_NODE_BOUND                     : u16 = 5;
pub const kSNEEZE_ABI_METHOD_NODE_NAME                      : u16 = 6;
pub const kSNEEZE_ABI_METHOD_NODE_RESOURCE                  : u16 = 7;
pub const kSNEEZE_ABI_METHOD_NODE_PANEL                     : u16 = 8;

// DATA methods (read-only; the immutable analog of STORAGE).
pub const kSNEEZE_ABI_METHOD_DATA_HAS                       : u16 = 1;
pub const kSNEEZE_ABI_METHOD_DATA_GET                       : u16 = 2;

// CHRONO methods (wall clock + host-owned civil logic; fills a MOMENT).
pub const kSNEEZE_ABI_METHOD_CHRONO_TIME                    : u16 = 1;
pub const kSNEEZE_ABI_METHOD_CHRONO_DATE                    : u16 = 2;
pub const kSNEEZE_ABI_METHOD_CHRONO_NOW                     : u16 = 3;
pub const kSNEEZE_ABI_METHOD_CHRONO_MOMENT                  : u16 = 4;
pub const kSNEEZE_ABI_METHOD_CHRONO_SET                     : u16 = 5;
pub const kSNEEZE_ABI_METHOD_CHRONO_PARSE                   : u16 = 6;
pub const kSNEEZE_ABI_METHOD_CHRONO_FORMAT                  : u16 = 7;

// PERFORMANCE methods (monotonic high-resolution clock).
pub const kSNEEZE_ABI_METHOD_PERFORMANCE_NOW                : u16 = 1;
pub const kSNEEZE_ABI_METHOD_PERFORMANCE_ORIGIN             : u16 = 2;

// TIMER methods. SET/CLEAR are guest -> host; FIRED is the host -> guest Notify.
pub const kSNEEZE_ABI_METHOD_TIMER_SET                      : u16 = 1;
pub const kSNEEZE_ABI_METHOD_TIMER_CLEAR                    : u16 = 2;
pub const kSNEEZE_ABI_METHOD_TIMER_FIRED                    : u16 = 3;

// SERVICES methods (read-only service reads, keyed by service name).
pub const kSNEEZE_ABI_METHOD_SERVICES_HAS                   : u16 = 1;
pub const kSNEEZE_ABI_METHOD_SERVICES_GET                   : u16 = 2;

// TRUST levels - the container's verification standing (Container.eTrust in the
// Open snapshot). Ordered least-to-most trusted.
pub const kSNEEZE_ABI_TRUST_NONE                            : i32 = 0;
pub const kSNEEZE_ABI_TRUST_UNTRUSTED                       : i32 = 1;
pub const kSNEEZE_ABI_TRUST_UNVERIFIED                      : i32 = 2;
pub const kSNEEZE_ABI_TRUST_EXPIRED                         : i32 = 3;
pub const kSNEEZE_ABI_TRUST_VERIFIED                        : i32 = 4;
pub const kSNEEZE_ABI_TRUST_ROOT                            : i32 = 5;

// SILO scope - selects one of a SILO's four storage units.
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eSNEEZE_ABI_SILO_SCOPE
{
   kSNEEZE_ABI_SILO_SCOPE_PERMANENT_ORG                           = 0,
   kSNEEZE_ABI_SILO_SCOPE_PERMANENT_CONTAINER                     = 1,
   kSNEEZE_ABI_SILO_SCOPE_TEMPORARY_ORG                           = 2,
   kSNEEZE_ABI_SILO_SCOPE_TEMPORARY_CONTAINER                     = 3,
}

// TIMER_SET unit discriminant. TICK = 1/64 s count; MS = milliseconds; HZ =
// frequency (period is 1/nValue seconds).
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eSNEEZE_ABI_TIMER_UNIT
{
   kSNEEZE_ABI_TIMER_UNIT_TICK                                    = 0,
   kSNEEZE_ABI_TIMER_UNIT_MS                                      = 1,
   kSNEEZE_ABI_TIMER_UNIT_HZ                                      = 2,
}

// REQUEST_OPEN's verb. GET is 0 because it is the default and the only verb the
// engine caches; every other verb bypasses the cache.
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eSNEEZE_ABI_REQUEST_VERB
{
   kSNEEZE_ABI_REQUEST_VERB_GET                                   = 0,
   kSNEEZE_ABI_REQUEST_VERB_POST                                  = 1,
   kSNEEZE_ABI_REQUEST_VERB_PUT                                   = 2,
   kSNEEZE_ABI_REQUEST_VERB_PATCH                                 = 3,
   kSNEEZE_ABI_REQUEST_VERB_DELETE                                = 4,
   kSNEEZE_ABI_REQUEST_VERB_HEAD                                  = 5,
}

// REQUEST_STATE, the analog of XHR's readyState. COMPLETE means the server
// answered - it says nothing about the HTTP status, which may well be a 404.
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eSNEEZE_ABI_REQUEST_STATE
{
   kSNEEZE_ABI_REQUEST_STATE_IDLE                                 = 0,
   kSNEEZE_ABI_REQUEST_STATE_SENDING                              = 1,
   kSNEEZE_ABI_REQUEST_STATE_COMPLETE                             = 2,
   kSNEEZE_ABI_REQUEST_STATE_FAILED                               = 3,
   kSNEEZE_ABI_REQUEST_STATE_ABORTED                              = 4,
}

impl eSNEEZE_ABI_REQUEST_STATE
{
   /// One of the two ABI enums read back off the wire rather than written to it,
   /// so it needs the reverse mapping. An unrecognized value reads as IDLE.
   pub fn From_Value (nValue: i64) -> Self
   {
      match nValue
      {
         1 => Self::kSNEEZE_ABI_REQUEST_STATE_SENDING,
         2 => Self::kSNEEZE_ABI_REQUEST_STATE_COMPLETE,
         3 => Self::kSNEEZE_ABI_REQUEST_STATE_FAILED,
         4 => Self::kSNEEZE_ABI_REQUEST_STATE_ABORTED,
         _ => Self::kSNEEZE_ABI_REQUEST_STATE_IDLE,
      }
   }
}

// SOCKET_STATE mirrors WebSocket.readyState exactly, values included.
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eSNEEZE_ABI_SOCKET_STATE
{
   kSNEEZE_ABI_SOCKET_STATE_CONNECTING                            = 0,
   kSNEEZE_ABI_SOCKET_STATE_OPEN                                  = 1,
   kSNEEZE_ABI_SOCKET_STATE_CLOSING                               = 2,
   kSNEEZE_ABI_SOCKET_STATE_CLOSED                                = 3,
}

impl eSNEEZE_ABI_SOCKET_STATE
{
   /// Read back off the wire, like REQUEST_STATE. An unrecognized value reads as
   /// CLOSED, the safe answer for a handle the host no longer knows.
   pub fn From_Value (nValue: i64) -> Self
   {
      match nValue
      {
         0 => Self::kSNEEZE_ABI_SOCKET_STATE_CONNECTING,
         1 => Self::kSNEEZE_ABI_SOCKET_STATE_OPEN,
         2 => Self::kSNEEZE_ABI_SOCKET_STATE_CLOSING,
         _ => Self::kSNEEZE_ABI_SOCKET_STATE_CLOSED,
      }
   }
}

// CHRONO zone selector: how SET interprets its civil input and which cached
// view FORMAT renders. (Getters read both views straight from the MOMENT.)
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eSNEEZE_ABI_CHRONO_ZONE
{
   kSNEEZE_ABI_CHRONO_ZONE_UTC                                    = 0,
   kSNEEZE_ABI_CHRONO_ZONE_LOCAL                                  = 1,
}

// MAP_OBJECT class ids.
pub const kSNEEZE_ABI_MAP_OBJECT_CLASS_ROOT                 : u16 = 70;
pub const kSNEEZE_ABI_MAP_OBJECT_CLASS_CELESTIAL            : u16 = 71;
pub const kSNEEZE_ABI_MAP_OBJECT_CLASS_TERRESTRIAL          : u16 = 72;
pub const kSNEEZE_ABI_MAP_OBJECT_CLASS_PHYSICAL             : u16 = 73;
pub const kSNEEZE_ABI_MAP_OBJECT_CLASS_PANEL                : u16 = 74;
pub const kSNEEZE_ABI_MAP_OBJECT_CLASS_LIGHT                : u16 = 75;

// OBJECTIX sentinels + composition (macros in the C header, hence SNEEZE_* not
// kSNEEZE_ABI_*).
pub const SNEEZE_OBJECTIX_ERROR                             : u64 = 0x0000_FFFF_FFFF_FFFE;
pub const SNEEZE_OBJECTIX_IDENTITY                          : u64 = 0x0000_FFFF_FFFF_FFFF;

pub const fn SNEEZE_OBJECTIX_COMPOSE (wClass: u16, twObjectIx: u64) -> u64 { ((wClass as u64) << 48)  |  (twObjectIx & 0x0000_FFFF_FFFF_FFFF) }
pub const fn SNEEZE_OBJECTIX_CLASS   (             qwComposed: u64) -> u16 {                             (qwComposed >> 48) as u16 }
pub const fn SNEEZE_OBJECTIX_INDEX   (             qwComposed: u64) -> u64 {                             (qwComposed & 0x0000_FFFF_FFFF_FFFF) }
