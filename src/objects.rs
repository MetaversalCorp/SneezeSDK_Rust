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

//! The typed API surface: FABRIC and its subsystems (CONSOLE, STORAGE, SCENE)
//! plus NODE. Each object is a thin, copyable handle that packs and sends the
//! matching packet. Everything hangs off a `FABRIC`.

use crate::abi::*;
use crate::ffi::PACKET;
use crate::mapobject::SNEEZE_ABI_MAPOBJECT;
use crate::moment::MOMENT;
use crate::snapshot::{LOCATION, RESOURCE, CONTAINER, SIGNATURE, AGENT, SERVICE, MODULE};
use crate::Snapshot;

// ---------------------------------------------------------------------------
// FABRIC - the root handle. All subsystems are reached through it.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct FABRIC
{
   m_twFabricIx: u64,
}

impl FABRIC
{
   pub fn New (twFabricIx: u64) -> Self { FABRIC { m_twFabricIx: twFabricIx } }

   pub fn Index (&self) -> u64 { self.m_twFabricIx }

   pub fn Console     (&self) -> CONSOLE     { CONSOLE     { m_twFabricIx: self.m_twFabricIx } }
   pub fn Storage     (&self) -> STORAGE     { STORAGE     { m_twFabricIx: self.m_twFabricIx } }
   pub fn Scene       (&self) -> SCENE       { SCENE       { m_twFabricIx: self.m_twFabricIx } }
   pub fn Data        (&self) -> DATA        { DATA        { m_twFabricIx: self.m_twFabricIx } }
   pub fn Chrono      (&self) -> CHRONO      { CHRONO      { m_twFabricIx: self.m_twFabricIx } }
   pub fn Performance (&self) -> PERFORMANCE { PERFORMANCE { m_twFabricIx: self.m_twFabricIx } }
   pub fn Timer       (&self) -> TIMER       { TIMER       { m_twFabricIx: self.m_twFabricIx } }

   // Typed read-only views over the private Open snapshot. LOCATION is built from
   // the resource reference; the rest borrow their section directly.
   pub fn Location  (&self) -> LOCATION           { LOCATION::New (Snapshot ().Resource.Reference ()) }
   pub fn Resource  (&self) -> &'static RESOURCE  { &Snapshot ().Resource }
   pub fn Signature (&self) -> &'static SIGNATURE { &Snapshot ().Signature }
   pub fn Agent     (&self) -> &'static AGENT     { &Snapshot ().Agent }
   pub fn Container (&self) -> &'static CONTAINER { &Snapshot ().Container }
   pub fn Services  (&self) -> &'static [SERVICE] { &Snapshot ().Services }
   pub fn Modules   (&self) -> &'static [MODULE]  { &Snapshot ().Modules }
}

// ---------------------------------------------------------------------------
// CONSOLE - developer console, forwarded to the container's stream.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct CONSOLE
{
   m_twFabricIx: u64,
}

impl CONSOLE
{
   fn Message (&self, wMethod: u16, sText: &str)
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_CONSOLE, wMethod);

      pPacket.Write_Qword (self.m_twFabricIx);
      pPacket.Write_Text (sText);

      pPacket.Send ();
   }

   pub fn Log             (&self, sText: &str) { self.Message (kSNEEZE_ABI_METHOD_CONSOLE_LOG,             sText); }
   pub fn Debug           (&self, sText: &str) { self.Message (kSNEEZE_ABI_METHOD_CONSOLE_DEBUG,           sText); }
   pub fn Info            (&self, sText: &str) { self.Message (kSNEEZE_ABI_METHOD_CONSOLE_INFO,            sText); }
   pub fn Warn            (&self, sText: &str) { self.Message (kSNEEZE_ABI_METHOD_CONSOLE_WARN,            sText); }
   pub fn Error           (&self, sText: &str) { self.Message (kSNEEZE_ABI_METHOD_CONSOLE_ERROR,           sText); }
   pub fn Group           (&self, sText: &str) { self.Message (kSNEEZE_ABI_METHOD_CONSOLE_GROUP,           sText); }
   pub fn Group_Collapsed (&self, sText: &str) { self.Message (kSNEEZE_ABI_METHOD_CONSOLE_GROUP_COLLAPSED, sText); }
   pub fn Count           (&self, sText: &str) { self.Message (kSNEEZE_ABI_METHOD_CONSOLE_COUNT,           sText); }
   pub fn Count_Reset     (&self, sText: &str) { self.Message (kSNEEZE_ABI_METHOD_CONSOLE_COUNT_RESET,     sText); }
   pub fn Time            (&self, sText: &str) { self.Message (kSNEEZE_ABI_METHOD_CONSOLE_TIME,            sText); }
   pub fn Time_End        (&self, sText: &str) { self.Message (kSNEEZE_ABI_METHOD_CONSOLE_TIME_END,        sText); }
   pub fn Time_Log        (&self, sText: &str) { self.Message (kSNEEZE_ABI_METHOD_CONSOLE_TIME_LOG,        sText); }

   pub fn Assert (&self, bCondition: bool, sText: &str)
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_CONSOLE, kSNEEZE_ABI_METHOD_CONSOLE_ASSERT);

      pPacket.Write_Qword  (self.m_twFabricIx);
      pPacket.Write_Number (if bCondition { 1 } else { 0 });
      pPacket.Write_Text   (sText);

      pPacket.Send ();
   }

   pub fn Group_End (&self)
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_CONSOLE, kSNEEZE_ABI_METHOD_CONSOLE_GROUP_END);

      pPacket.Write_Qword (self.m_twFabricIx);

      pPacket.Send ();
   }
}

// ---------------------------------------------------------------------------
// STORAGE - persistent JSON document store. Values are JSON text in both
// directions. An empty path ("") addresses the scope's whole root document.
//
// TODO (typed access, deferred): STORAGE is read/write, so when the SDK-owned
// typed accessor lands (see the DATA note below), it needs BOTH directions here
// - a typed Get (like DATA's) AND a typed Set that serializes a guest struct
// back to the wire. Blocked on the same coupling problem (no forced guest JSON
// dependency); do it with the sneeze-owned derive, not T: DeJson / T: SerJson
// bounds.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct STORAGE
{
   m_twFabricIx: u64,
}

impl STORAGE
{
   pub fn Has (&self, eScope: eSNEEZE_ABI_SILO_SCOPE, sPath: &str) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_STORAGE, kSNEEZE_ABI_METHOD_STORAGE_HAS);

      pPacket.Write_Qword  (self.m_twFabricIx);
      pPacket.Write_Number (eScope as i32);
      pPacket.Write_Text   (sPath);

      pPacket.Send () != 0
   }

   pub fn Set (&self, eScope: eSNEEZE_ABI_SILO_SCOPE, sPath: &str, sJson: &str) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_STORAGE, kSNEEZE_ABI_METHOD_STORAGE_SET);

      pPacket.Write_Qword  (self.m_twFabricIx);
      pPacket.Write_Number (eScope as i32);
      pPacket.Write_Text   (sPath);
      pPacket.Write_Text   (sJson);

      pPacket.Send () != 0
   }

   pub fn Remove (&self, eScope: eSNEEZE_ABI_SILO_SCOPE, sPath: &str) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_STORAGE, kSNEEZE_ABI_METHOD_STORAGE_REMOVE);

      pPacket.Write_Qword  (self.m_twFabricIx);
      pPacket.Write_Number (eScope as i32);
      pPacket.Write_Text   (sPath);

      pPacket.Send () != 0
   }

   /// Reads the JSON value at `sPath` as text. Returns None for a missing or null
   /// value. Sizes the buffer in one probe and, if needed, one exact re-read.
   pub fn Get (&self, eScope: eSNEEZE_ABI_SILO_SCOPE, sPath: &str) -> Option<String>
   {
      let mut sResult: Option<String> = None;
      let mut aByte = vec![0u8; 256];
      let nProbe = self.Get_Into (eScope, sPath, &mut aByte);

      if nProbe > 0
      {
         let mut nSizeNeeded = nProbe as usize;
         let mut bValid      = true;

         if nSizeNeeded > aByte.len ()
         {
            aByte = vec![0u8; nSizeNeeded];
            let nAgain = self.Get_Into (eScope, sPath, &mut aByte);

            if nAgain > 0
            {
               nSizeNeeded = nAgain as usize;
            }
            else
            {
               bValid = false;
            }
         }

         if bValid
         {
            let nCount = if nSizeNeeded < aByte.len () { nSizeNeeded } else { aByte.len () };

            aByte.truncate (nCount);
            sResult = String::from_utf8 (aByte).ok ();
         }
      }

      sResult
   }

   fn Get_Into (&self, eScope: eSNEEZE_ABI_SILO_SCOPE, sPath: &str, aByte: &mut [u8]) -> i64
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_STORAGE, kSNEEZE_ABI_METHOD_STORAGE_GET);

      pPacket.Write_Qword  (self.m_twFabricIx);
      pPacket.Write_Number (eScope as i32);
      pPacket.Write_Text   (sPath);
      pPacket.Write_Bytes  (aByte.as_ptr (), aByte.len ());

      pPacket.Send ()
   }
}

// ---------------------------------------------------------------------------
// DATA - the fabric's read-only config "Data" tree. Values are JSON text. An
// empty path ("") addresses the whole document. The immutable analog of STORAGE:
// Has and Get only, no scope (the data belongs to the one fabric).
//
// TODO (typed access, deferred): we want a transparent typed read here (and a
// typed write on the read/write side - STORAGE; see also the STORAGE note above),
// e.g. Get_As::<T> () populating a guest struct with no visible JSON step. The
// blocker is coupling: a T: DeJson bound (nanoserde) forces every guest module
// to depend on nanoserde directly - its derive emits nanoserde:: paths that must
// resolve in the guest crate - which also welds the SDK to nanoserde and blocks
// swapping it. Interim Get_As was removed for exactly this reason. The right fix
// is an SDK-owned derive (a companion sneeze-derive crate) so the guest derives
// sneeze::Data, depends only on sneeze, and the JSON backend stays a private,
// swappable SDK detail. Until then the SDK exposes only raw JSON text (Get) and
// the module parses it with a JSON crate of its own choosing.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct DATA
{
   m_twFabricIx: u64,
}

impl DATA
{
   pub fn Has (&self, sPath: &str) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_DATA, kSNEEZE_ABI_METHOD_DATA_HAS);

      pPacket.Write_Qword (self.m_twFabricIx);
      pPacket.Write_Text  (sPath);

      pPacket.Send () != 0
   }

   /// Reads the JSON value at `sPath` as text. Returns None for a missing or null
   /// value. Sizes the buffer in one probe and, if needed, one exact re-read.
   pub fn Get (&self, sPath: &str) -> Option<String>
   {
      let mut sResult: Option<String> = None;
      let mut aByte = vec![0u8; 256];
      let nProbe = self.Get_Into (sPath, &mut aByte);

      if nProbe > 0
      {
         let mut nSizeNeeded = nProbe as usize;
         let mut bValid      = true;

         if nSizeNeeded > aByte.len ()
         {
            aByte = vec![0u8; nSizeNeeded];
            let nAgain = self.Get_Into (sPath, &mut aByte);

            if nAgain > 0
            {
               nSizeNeeded = nAgain as usize;
            }
            else
            {
               bValid = false;
            }
         }

         if bValid
         {
            let nCount = if nSizeNeeded < aByte.len () { nSizeNeeded } else { aByte.len () };

            aByte.truncate (nCount);
            sResult = String::from_utf8 (aByte).ok ();
         }
      }

      sResult
   }

   fn Get_Into (&self, sPath: &str, aByte: &mut [u8]) -> i64
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_DATA, kSNEEZE_ABI_METHOD_DATA_GET);

      pPacket.Write_Qword (self.m_twFabricIx);
      pPacket.Write_Text  (sPath);
      pPacket.Write_Bytes (aByte.as_ptr (), aByte.len ());

      pPacket.Send ()
   }
}

// ---------------------------------------------------------------------------
// SCENE - node-tree construction on the fabric's container.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct SCENE
{
   m_twFabricIx: u64,
}

impl SCENE
{
   /// Creates the fabric's root node from a map object.
   pub fn Node_Root (&self, pObject: &SNEEZE_ABI_MAPOBJECT) -> NODE
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_SCENE, kSNEEZE_ABI_METHOD_SCENE_NODE_ROOT);

      pPacket.Write_Qword (self.m_twFabricIx);
      pPacket.Write_Bytes (pObject.Pointer (), SNEEZE_ABI_MAPOBJECT::SIZE);

      NODE { m_qwComposed: pPacket.Send () as u64 }
   }

   /// Builds the fabric's node tree from the MSF "Data" block at `sPath` (an empty
   /// path is the "Data" object itself). Returns the created root node.
   pub fn Node_Map (&self, sPath: &str) -> NODE
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_SCENE, kSNEEZE_ABI_METHOD_SCENE_NODE_MAP);

      pPacket.Write_Qword (self.m_twFabricIx);
      pPacket.Write_Text  (sPath);

      NODE { m_qwComposed: pPacket.Send () as u64 }
   }

   /// Creates a child node from a map object. The parent is taken from the map
   /// object's own parent index (set via `SNEEZE_ABI_MAPOBJECT::Parent`), so any
   /// parent index may be named directly - no parent `NODE` handle is required.
   pub fn Node_Open (&self, pObject: &SNEEZE_ABI_MAPOBJECT) -> NODE
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_SCENE, kSNEEZE_ABI_METHOD_SCENE_NODE_OPEN);

      pPacket.Write_Bytes (pObject.Pointer (), SNEEZE_ABI_MAPOBJECT::SIZE);

      NODE { m_qwComposed: pPacket.Send () as u64 }
   }

   /// Removes and deletes a node.
   pub fn Node_Close (&self, pNode: NODE) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_SCENE, kSNEEZE_ABI_METHOD_SCENE_NODE_CLOSE);

      pPacket.Write_Qword (pNode.m_qwComposed);

      pPacket.Send () != 0
   }
}

// ---------------------------------------------------------------------------
// NODE - a live scene object, mutated by its object index.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct NODE
{
   m_qwComposed: u64,
}

impl NODE
{
   pub fn Composed (&self) -> u64 { self.m_qwComposed }
   pub fn Class    (&self) -> u16 { SNEEZE_OBJECTIX_CLASS (self.m_qwComposed) }
   pub fn ObjectIx (&self) -> u64 { SNEEZE_OBJECTIX_INDEX (self.m_qwComposed) }

   /// True unless the creating call failed (SNEEZE_OBJECTIX_ERROR / zero).
   pub fn IsValid (&self) -> bool
   {
      self.m_qwComposed != SNEEZE_OBJECTIX_ERROR  &&  self.m_qwComposed != 0
   }

   pub fn Position (&self, dX: f64, dY: f64, dZ: f64)
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NODE, kSNEEZE_ABI_METHOD_NODE_POSITION);

      pPacket.Write_Qword  (self.m_qwComposed);
      pPacket.Write_Double (dX);
      pPacket.Write_Double (dY);
      pPacket.Write_Double (dZ);

      pPacket.Send ();
   }

   pub fn Scale (&self, dScale: f64)
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NODE, kSNEEZE_ABI_METHOD_NODE_SCALE);

      pPacket.Write_Qword  (self.m_qwComposed);
      pPacket.Write_Double (dScale);

      pPacket.Send ();
   }

   pub fn Scale_Axes (&self, dX: f64, dY: f64, dZ: f64)
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NODE, kSNEEZE_ABI_METHOD_NODE_SCALE_AXES);

      pPacket.Write_Qword  (self.m_qwComposed);
      pPacket.Write_Double (dX);
      pPacket.Write_Double (dY);
      pPacket.Write_Double (dZ);

      pPacket.Send ();
   }

   pub fn Bound (&self, dX: f64, dY: f64, dZ: f64)
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NODE, kSNEEZE_ABI_METHOD_NODE_BOUND);
      
      pPacket.Write_Qword  (self.m_qwComposed);
      pPacket.Write_Double (dX);
      pPacket.Write_Double (dY);
      pPacket.Write_Double (dZ);

      pPacket.Send ();
   }

   pub fn Name (&self, sName: &str)
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NODE, kSNEEZE_ABI_METHOD_NODE_NAME);

      pPacket.Write_Qword (self.m_qwComposed);
      pPacket.Write_Text  (sName);

      pPacket.Send ();
   }

   pub fn Resource (&self, sUrl: &str)
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NODE, kSNEEZE_ABI_METHOD_NODE_RESOURCE);

      pPacket.Write_Qword (self.m_qwComposed);
      pPacket.Write_Text  (sUrl);

      pPacket.Send ();
   }

   /// Sets a PANEL node's RML+CSS source (no effect on non-panel nodes).
   pub fn Panel (&self, sRml: &str)
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NODE, kSNEEZE_ABI_METHOD_NODE_PANEL);

      pPacket.Write_Qword (self.m_qwComposed);
      pPacket.Write_Text  (sRml);
      
      pPacket.Send ();
   }
}

// ---------------------------------------------------------------------------
// CHRONO - the wall clock and the calendar (civil) logic behind a MOMENT. Bare
// scalars (Time/Date) skip the struct; Now and the MOMENT constructors fill one.
// All breakdown / formatting / parsing lives host-side; a MOMENT caches the
// result and is read locally.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct CHRONO
{
   m_twFabricIx: u64,
}

impl CHRONO
{
   /// Current wall time as a `tm` scalar (1/64 s since 1601-01-01, UTC).
   pub fn Time (&self) -> i64
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_CHRONO, kSNEEZE_ABI_METHOD_CHRONO_TIME);

      pPacket.Write_Qword (self.m_twFabricIx);

      pPacket.Send ()
   }

   /// Current wall time as a `dt` scalar (Unix ms since 1970-01-01, UTC).
   pub fn Date (&self) -> i64
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_CHRONO, kSNEEZE_ABI_METHOD_CHRONO_DATE);

      pPacket.Write_Qword (self.m_twFabricIx);

      pPacket.Send ()
   }

   /// Current wall time as a fully-populated MOMENT.
   pub fn Now (&self) -> MOMENT { MOMENT::From_Now (self.m_twFabricIx) }
}

// ---------------------------------------------------------------------------
// PERFORMANCE - the monotonic high-resolution clock (JS performance). Now is a
// 100 ns count since a fixed origin; Origin is the wall MOMENT at that t0.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct PERFORMANCE
{
   m_twFabricIx: u64,
}

impl PERFORMANCE
{
   /// Monotonic elapsed time since the origin, in 100 ns units.
   pub fn Now (&self) -> i64
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_PERFORMANCE, kSNEEZE_ABI_METHOD_PERFORMANCE_NOW);

      pPacket.Write_Qword (self.m_twFabricIx);

      pPacket.Send ()
   }

   /// The wall-clock MOMENT the monotonic origin was anchored to (JS timeOrigin).
   pub fn Origin (&self) -> MOMENT { MOMENT::From_Origin (self.m_twFabricIx) }
}

// ---------------------------------------------------------------------------
// TIMER - one-shot (Set) and repeating (Interval) callbacks. Both return a
// twTimerIx (0 = failure) and echo qwParam on the Notify. The unit is ticks
// (1/64 s), milliseconds, or Hertz. The fire is delivered to INSTANCE::Timer.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct TIMER
{
   m_twFabricIx: u64,
}

impl TIMER
{
   /// Arms a one-shot timer. `nValue` is interpreted per `eUnit`.
   pub fn Set (&self, nValue: i32, eUnit: eSNEEZE_ABI_TIMER_UNIT, qwParam: u64) -> u64
   {
      self.Arm (nValue, eUnit, qwParam, false)
   }

   /// Arms a repeating timer that re-fires every period until cleared.
   pub fn Interval (&self, nValue: i32, eUnit: eSNEEZE_ABI_TIMER_UNIT, qwParam: u64) -> u64
   {
      self.Arm (nValue, eUnit, qwParam, true)
   }

   /// Disarms a timer by its id. Returns true if it was found.
   pub fn Clear (&self, twTimerIx: u64) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_TIMER, kSNEEZE_ABI_METHOD_TIMER_CLEAR);

      pPacket.Write_Qword (self.m_twFabricIx);
      pPacket.Write_Qword (twTimerIx);

      pPacket.Send () != 0
   }

   fn Arm (&self, nValue: i32, eUnit: eSNEEZE_ABI_TIMER_UNIT, qwParam: u64, bRepeat: bool) -> u64
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_TIMER, kSNEEZE_ABI_METHOD_TIMER_SET);

      pPacket.Write_Qword  (self.m_twFabricIx);
      pPacket.Write_Number (eUnit as i32);
      pPacket.Write_Number (nValue);
      pPacket.Write_Qword  (qwParam);
      pPacket.Write_Number (if bRepeat { 1 } else { 0 });

      pPacket.Send () as u64
   }
}
