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

//! # sneeze - the guest-side SDK for Sneeze WASM modules
//!
//! A module talks to the engine through exactly one host import (`Sneeze.Call`)
//! and a handful of exports. This crate hides that ABI behind typed objects:
//! a [`HOST`] handed to your [`INSTANCE`] at `Open` (a `&HOST`, persistent for
//! that fabric's lifetime), from which you reach [`CONSOLE`], [`STORAGE`],
//! [`DATA`], [`SERVICES`], [`SCENE`], and [`FABRIC`] (the node-tree API); build
//! nodes with [`SNEEZE_ABI_MAPOBJECT`] and mutate them through [`NODE`].
//!
//! ```ignore
//! use sneeze::*;
//!
//! struct MY_MODULE;
//! impl INSTANCE for MY_MODULE
//! {
//!    fn Open (pHost: &HOST)
//!    {
//!       pHost.Console ().Log ("hello from wasm");
//!
//!       let mut root = SNEEZE_ABI_MAPOBJECT::Physical ();
//!       root.Name ("Stool").Reference ("assets/Stool.glb");
//!       pHost.Fabric ().Node_Root (&root);
//!    }
//! }
//!
//! sneeze::instance! (MY_MODULE);
//! ```
//!
//! The ABI contract is `sdk/include/sneeze_abi.h`; this crate mirrors it.

#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, unused_parens)]

pub mod abi;
mod ffi;
mod objects;
mod mapobject;
mod mapservice;
mod moment;
mod snapshot;

use nanoserde::DeJson;

pub use abi::{SNEEZE_OBJECTIX_CLASS, SNEEZE_OBJECTIX_COMPOSE, SNEEZE_OBJECTIX_INDEX, SNEEZE_OBJECTIX_ERROR, SNEEZE_OBJECTIX_IDENTITY};
pub use abi::{eSNEEZE_ABI_SILO_SCOPE, eSNEEZE_ABI_TIMER_UNIT, eSNEEZE_ABI_CHRONO_ZONE};
pub use objects::{CHRONO, CONSOLE, DATA, FABRIC, HOST, NODE, PERFORMANCE, SCENE, SERVICES, STORAGE, TIMER};
pub use mapobject::SNEEZE_ABI_MAPOBJECT;
pub use mapservice::MAP_SERVICE;
pub use moment::MOMENT;
pub use snapshot::{LOCATION, RESOURCE, CONTAINER, SIGNATURE, AGENT, MODULE};

use snapshot::SNAPSHOT_DATA;

// ---------------------------------------------------------------------------
// The parsed Open snapshot, held privately for the life of the module instance.
// A module never touches it directly; it reads the typed views off its HOST
// (Location/Resource/Signature/Agent/Container). Single-threaded wasm, written
// once by the generated Open before user code runs, read-only thereafter.
// ---------------------------------------------------------------------------

static mut SNAPSHOT_STORE: Option<SNAPSHOT_DATA> = None;

pub (crate) fn Snapshot () -> &'static SNAPSHOT_DATA
{
   unsafe
   {
      let pStore = &mut *core::ptr::addr_of_mut! (SNAPSHOT_STORE);

      if pStore.is_none ()
      {
         *pStore = Some (SNAPSHOT_DATA::default ());
      }

      pStore.as_ref ().unwrap ()
   }
}

#[doc(hidden)]
pub fn Snapshot_Load (pSnapshot: SNAPSHOT)
{
   unsafe
   {
      let pStore = &mut *core::ptr::addr_of_mut! (SNAPSHOT_STORE);

      *pStore = Some (pSnapshot.Parse ());
   }
}

// ---------------------------------------------------------------------------
// SNAPSHOT - the immutable blob the engine pushes at Open. Internal plumbing:
// the engine synthesizes a JSON document of fixed-shape sections, copies it into
// guest memory via the Alloc handshake, and hands the generated Open its
// (offset, size). Snapshot_Load parses it once into the private SNAPSHOT_STORE
// before user code runs. A module never sees this type; it reads the parsed
// data through the typed HOST views (Location/Resource/Signature/Agent/
// Container). The raw bytes are valid only for the duration of the generated
// Open - the engine frees the guest block as soon as Open returns.
// ---------------------------------------------------------------------------

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct SNAPSHOT
{
   m_nOffset: u32,
   m_nLength: u32,
}

impl SNAPSHOT
{
   pub fn From_Raw (nOffset: i32, nSize: i32) -> Self
   {
      SNAPSHOT { m_nOffset: nOffset as u32, m_nLength: nSize as u32 }
   }

   pub (crate) fn Bytes (&self) -> &[u8]
   {
      if self.m_nLength == 0
      {
         &[]
      }
      else
      {
         unsafe { core::slice::from_raw_parts (self.m_nOffset as *const u8, self.m_nLength as usize) }
      }
   }

   pub (crate) fn Text (&self) -> &str
   {
      core::str::from_utf8 (self.Bytes ()).unwrap_or ("")
   }

   pub (crate) fn Parse (&self) -> SNAPSHOT_DATA
   {
      SNAPSHOT_DATA::deserialize_json (self.Text ()).unwrap_or_default ()
   }
}

// ---------------------------------------------------------------------------
// INSTANCE - the lifecycle a guest wasm instance implements. Wire it up with
// instance!. (The engine calls the running module a WASM_INSTANCE; a declared
// module in the manifest is the separate MODULE record.) Open receives a &HOST
// (persistent for the fabric's lifetime); the Open snapshot is parsed privately
// and read through the host's typed views.
// ---------------------------------------------------------------------------

pub trait INSTANCE
{
   fn Init () {}
   fn Open (pHost: &HOST) { let _ = pHost; }
   fn Close (pHost: &HOST) { let _ = pHost; }
   fn Shutdown () {}

   /// A timer armed via `HOST::Timer` fired. `twTimerIx` is the id returned by
   /// Set/Interval; `qwParam` is the cookie passed when arming. Default: ignore.
   fn Timer (pHost: &HOST, twTimerIx: u64, qwParam: u64) { let _ = (pHost, twTimerIx, qwParam); }
}

// ---------------------------------------------------------------------------
// Memory + event exports the host calls. module! re-exports these under the
// ABI names. Alloc/Free let the host place bytes into guest memory (the Open
// snapshot, and later event packets); Notify is the host -> guest event entry
// point (inert until node events land).
// ---------------------------------------------------------------------------

pub fn Alloc (nSize: i32) -> i32
{
   let mut nOffset = 0;

   if nSize > 0
   {
      let mut aByte: Vec<u8> = Vec::with_capacity (nSize as usize);
      let pByte = aByte.as_mut_ptr ();

      core::mem::forget (aByte);

      nOffset = pByte as u32 as i32;
   }

   nOffset
}

pub fn Free (nOffset: i32, nSize: i32)
{
   if nOffset != 0  &&  nSize > 0
   {
      unsafe
      {
         let _ = Vec::from_raw_parts (nOffset as u32 as *mut u8, 0, nSize as usize);
      }
   }
}

// ---------------------------------------------------------------------------
// HOST registry - one persistent HOST per fabric, keyed by fabric index. The
// generated Open acquires it (creating on first use); callbacks find it; the
// generated Close releases it. Boxed so each HOST keeps a stable address as
// fabrics come and go. Single-threaded wasm, so the static mut is sound. One
// WASM instance can serve several fabrics, so several HOSTs may be live at once.
// ---------------------------------------------------------------------------

static mut HOST_STORE: Vec<Box<HOST>> = Vec::new ();

#[doc(hidden)]
pub fn Host_Acquire (twFabricIx: u64) -> &'static HOST
{
   unsafe
   {
      let pStore = &mut *core::ptr::addr_of_mut! (HOST_STORE);
      let mut pResult: *const HOST = core::ptr::null ();

      for pHost in pStore.iter ()
      {
         if pHost.Index () == twFabricIx
         {
            pResult = pHost.as_ref () as *const HOST;
            break;
         }
      }

      if pResult.is_null ()
      {
         pStore.push (Box::new (HOST::New (twFabricIx)));
         pResult = pStore.last ().unwrap ().as_ref () as *const HOST;
      }

      &*pResult
   }
}

#[doc(hidden)]
pub fn Host_Find (twFabricIx: u64) -> Option<&'static HOST>
{
   unsafe
   {
      let pStore = &mut *core::ptr::addr_of_mut! (HOST_STORE);
      let mut pResult: Option<&'static HOST> = None;

      for pHost in pStore.iter ()
      {
         if pHost.Index () == twFabricIx
         {
            pResult = Some (&*(pHost.as_ref () as *const HOST));
            break;
         }
      }

      pResult
   }
}

#[doc(hidden)]
pub fn Host_Release (twFabricIx: u64)
{
   unsafe
   {
      let pStore = &mut *core::ptr::addr_of_mut! (HOST_STORE);
      let mut nIndex = usize::MAX;

      for (nIz, pHost) in pStore.iter ().enumerate ()
      {
         if pHost.Index () == twFabricIx
         {
            nIndex = nIz;
            break;
         }
      }

      if nIndex != usize::MAX
      {
         pStore.remove (nIndex);
      }
   }
}

// ---------------------------------------------------------------------------
// EVENT - a decoded host -> guest Notify packet. The generated Notify export
// runs Event_Parse and dispatches to the matching INSTANCE hook, so a module
// only ever sees typed events (never the raw packet). Unknown events are inert
// (forward-compatible: a new host event an old module has no hook for is
// silently dropped).
// ---------------------------------------------------------------------------

#[doc(hidden)]
pub enum EVENT
{
   Timer { pHost: &'static HOST, twTimerIx: u64, qwParam: u64 },
   Unknown,
}

#[doc(hidden)]
pub fn Event_Parse (nOffset: i32, nSize: i32) -> EVENT
{
   let mut eEvent = EVENT::Unknown;

   // Header is 8 bytes (wType u16, wMethod u16, dwSize u32); payload follows.
   if nOffset != 0  &&  nSize >= 8
   {
      let aByte = unsafe { core::slice::from_raw_parts (nOffset as u32 as *const u8, nSize as usize) };

      let wType    = u16::from_le_bytes ([aByte[0], aByte[1]]);
      let wMethod  = u16::from_le_bytes ([aByte[2], aByte[3]]);
      let aPayload = &aByte[8..];

      if wType == abi::kSNEEZE_ABI_TYPE_TIMER  &&  wMethod == abi::kSNEEZE_ABI_METHOD_TIMER_FIRED  &&  aPayload.len () >= 24
      {
         let twFabricIx = u64::from_le_bytes (aPayload[ 0.. 8].try_into ().unwrap ());
         let twTimerIx  = u64::from_le_bytes (aPayload[ 8..16].try_into ().unwrap ());
         let qwParam    = u64::from_le_bytes (aPayload[16..24].try_into ().unwrap ());

         if let Some (pHost) = Host_Find (twFabricIx)
         {
            eEvent = EVENT::Timer { pHost, twTimerIx, qwParam };
         }
      }
   }

   eEvent
}

// ---------------------------------------------------------------------------
// instance! - generates the exports the engine looks up, delegating the
// lifecycle to a type that implements INSTANCE.
// ---------------------------------------------------------------------------

#[macro_export]
macro_rules! instance
{
   ($instance:ty) =>
   {
      #[no_mangle]
      pub extern "C" fn Init ()
      {
         <$instance as $crate::INSTANCE>::Init ();
      }

      #[no_mangle]
      pub extern "C" fn Open (twFabricIx: u64, nOffset: i32, nSize: i32)
      {
         $crate::Snapshot_Load ($crate::SNAPSHOT::From_Raw (nOffset, nSize));
         <$instance as $crate::INSTANCE>::Open ($crate::Host_Acquire (twFabricIx));
      }

      #[no_mangle]
      pub extern "C" fn Close (twFabricIx: u64)
      {
         if let Some (pHost) = $crate::Host_Find (twFabricIx)
         {
            <$instance as $crate::INSTANCE>::Close (pHost);
         }

         $crate::Host_Release (twFabricIx);
      }

      #[no_mangle]
      pub extern "C" fn Shutdown ()
      {
         <$instance as $crate::INSTANCE>::Shutdown ();
      }

      #[no_mangle]
      pub extern "C" fn Alloc (nSize: i32) -> i32
      {
         $crate::Alloc (nSize)
      }

      #[no_mangle]
      pub extern "C" fn Free (nOffset: i32, nSize: i32)
      {
         $crate::Free (nOffset, nSize);
      }

      #[no_mangle]
      pub extern "C" fn Notify (nOffset: i32, nSize: i32) -> i64
      {
         match $crate::Event_Parse (nOffset, nSize)
         {
            $crate::EVENT::Timer { pHost, twTimerIx, qwParam } =>
               <$instance as $crate::INSTANCE>::Timer (pHost, twTimerIx, qwParam),

            $crate::EVENT::Unknown => {},
         }

         0
      }
   };
}
