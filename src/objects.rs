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

//! The typed API surface: HOST and its subsystem views (CONSOLE, STORAGE, SCENE,
//! FABRIC, ...) plus NODE. Each view is a thin, copyable handle that packs and
//! sends the matching packet. Everything hangs off a `HOST`.

use crate::abi::*;
use crate::ffi::PACKET;
use crate::mapobject::SNEEZE_ABI_MAPOBJECT;
use crate::mapservice::MAP_SERVICE;
use crate::moment::MOMENT;
use crate::snapshot::{LOCATION, RESOURCE, CONTAINER, SIGNATURE, AGENT, MODULE};
use crate::Snapshot;

// ---------------------------------------------------------------------------
// HOST - the root handle: the browser host as the guest module sees it. The SDK
// allocates one per fabric at Open and hands your INSTANCE a &HOST that stays
// valid until after Close. Every subsystem view is reached through it. Scene
// node construction lives on the FABRIC view (Fabric ()).
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct HOST
{
   m_twFabricIx: u64,
}

impl HOST
{
   pub fn New (twFabricIx: u64) -> Self { HOST { m_twFabricIx: twFabricIx } }

   pub fn Index (&self) -> u64 { self.m_twFabricIx }

   pub fn Console     (&self) -> CONSOLE     { CONSOLE     { m_twFabricIx: self.m_twFabricIx } }
   pub fn Storage     (&self) -> STORAGE     { STORAGE     { m_twFabricIx: self.m_twFabricIx } }
   pub fn Scene       (&self) -> SCENE       { SCENE       { m_twFabricIx: self.m_twFabricIx } }
   pub fn Fabric      (&self) -> FABRIC      { FABRIC      { m_twFabricIx: self.m_twFabricIx } }
   pub fn Data        (&self) -> DATA        { DATA        { m_twFabricIx: self.m_twFabricIx } }
   pub fn Services    (&self) -> SERVICES    { SERVICES    { m_twFabricIx: self.m_twFabricIx } }
   pub fn Chrono      (&self) -> CHRONO      { CHRONO      { m_twFabricIx: self.m_twFabricIx } }
   pub fn Performance (&self) -> PERFORMANCE { PERFORMANCE { m_twFabricIx: self.m_twFabricIx } }
   pub fn Timer       (&self) -> TIMER       { TIMER       { m_twFabricIx: self.m_twFabricIx } }
   pub fn Network     (&self) -> NETWORK     { NETWORK     { m_twFabricIx: self.m_twFabricIx } }

   // Typed read-only views over the private Open snapshot. LOCATION is built from
   // the resource reference; the rest borrow their section directly.
   pub fn Location  (&self) -> LOCATION           { LOCATION::New (Snapshot ().Resource.Reference ()) }
   pub fn Resource  (&self) -> &'static RESOURCE  { &Snapshot ().Resource }
   pub fn Signature (&self) -> &'static SIGNATURE { &Snapshot ().Signature }
   pub fn Agent     (&self) -> &'static AGENT     { &Snapshot ().Agent }
   pub fn Container (&self) -> &'static CONTAINER { &Snapshot ().Container }
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
// SERVICES - the fabric's declared services, read-only and on-demand, keyed by
// service name (the DATA model, one level by name rather than a dotted path). A
// service object may carry any fields the fabric author chose, so Get returns
// the named service's whole JSON object as text (None for an absent service)
// for the module to parse itself.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct SERVICES
{
   m_twFabricIx: u64,
}

impl SERVICES
{
   pub fn Has (&self, sName: &str) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_SERVICES, kSNEEZE_ABI_METHOD_SERVICES_HAS);

      pPacket.Write_Qword (self.m_twFabricIx);
      pPacket.Write_Text  (sName);

      pPacket.Send () != 0
   }

   /// Reads the named service's whole JSON object as text. Returns None for an
   /// absent service. Sizes the buffer in one probe and, if needed, one re-read.
   pub fn Get (&self, sName: &str) -> Option<String>
   {
      let mut sResult: Option<String> = None;
      let mut aByte = vec![0u8; 256];
      let nProbe = self.Get_Into (sName, &mut aByte);

      if nProbe > 0
      {
         let mut nSizeNeeded = nProbe as usize;
         let mut bValid      = true;

         if nSizeNeeded > aByte.len ()
         {
            aByte = vec![0u8; nSizeNeeded];
            let nAgain = self.Get_Into (sName, &mut aByte);

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

   fn Get_Into (&self, sName: &str, aByte: &mut [u8]) -> i64
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_SERVICES, kSNEEZE_ABI_METHOD_SERVICES_GET);

      pPacket.Write_Qword (self.m_twFabricIx);
      pPacket.Write_Text  (sName);
      pPacket.Write_Bytes (aByte.as_ptr (), aByte.len ());

      pPacket.Send ()
   }
}

// ---------------------------------------------------------------------------
// SCENE - scene-global state (type 6): ambient light, directional light, and
// background. Its methods (the Ambient/Directional/Background get/set pairs) are
// not implemented yet; this view exists so `HOST::Scene ()` is ready for them.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct SCENE
{
   #[allow(dead_code)]
   m_twFabricIx: u64,
}

impl SCENE
{
}

// ---------------------------------------------------------------------------
// FABRIC - the node-tree API on the fabric's container (type 7). The first two
// hand the fabric to a browser-assigned map service (after which the module no
// longer mutates its nodes directly); the rest are the guest-assigned node-tree
// calls. Reached through `HOST::Fabric ()`. Method order matches sneeze_abi.h.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct FABRIC
{
   m_twFabricIx: u64,
}

impl FABRIC
{
   /// Connects a map service from a caller-filled MAP_SERVICE (from the module's
   /// own knowledge, or from `HOST::Services ().Get (name)` which it parses).
   /// Returns true if the host accepted the connection request.
   pub fn Node_Map_Service (&self, pService: &MAP_SERVICE) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_FABRIC, kSNEEZE_ABI_METHOD_FABRIC_NODE_MAP_SERVICE);

      pPacket.Write_Qword (self.m_twFabricIx);
      pPacket.Write_Bytes (pService.Pointer (), MAP_SERVICE::SIZE);

      pPacket.Send () != 0
   }

   /// Connects a map service the host reads from the fabric's Services[name] and
   /// fills the struct itself. Returns true if the host accepted the request.
   pub fn Node_Map_Service_Ex (&self, sName: &str) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_FABRIC, kSNEEZE_ABI_METHOD_FABRIC_NODE_MAP_SERVICE_EX);

      pPacket.Write_Qword (self.m_twFabricIx);
      pPacket.Write_Text  (sName);

      pPacket.Send () != 0
   }

   /// Builds the fabric's node tree from the MSF "Data" block at `sPath` (an empty
   /// path is the "Data" object itself). Returns the created root node.
   pub fn Node_Map_Data (&self, sPath: &str) -> NODE
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_FABRIC, kSNEEZE_ABI_METHOD_FABRIC_NODE_MAP_DATA);

      pPacket.Write_Qword (self.m_twFabricIx);
      pPacket.Write_Text  (sPath);

      NODE { m_qwComposed: pPacket.Send () as u64 }
   }

   /// Creates the fabric's root node from a map object.
   pub fn Node_Root (&self, pObject: &SNEEZE_ABI_MAPOBJECT) -> NODE
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_FABRIC, kSNEEZE_ABI_METHOD_FABRIC_NODE_ROOT);

      pPacket.Write_Qword (self.m_twFabricIx);
      pPacket.Write_Bytes (pObject.Pointer (), SNEEZE_ABI_MAPOBJECT::SIZE);

      NODE { m_qwComposed: pPacket.Send () as u64 }
   }

   /// Creates a child node from a map object. The parent is taken from the map
   /// object's own parent index (set via `SNEEZE_ABI_MAPOBJECT::Parent`), so any
   /// parent index may be named directly - no parent `NODE` handle is required.
   pub fn Node_Open (&self, pObject: &SNEEZE_ABI_MAPOBJECT) -> NODE
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_FABRIC, kSNEEZE_ABI_METHOD_FABRIC_NODE_OPEN);

      pPacket.Write_Bytes (pObject.Pointer (), SNEEZE_ABI_MAPOBJECT::SIZE);

      NODE { m_qwComposed: pPacket.Send () as u64 }
   }

   /// Removes and deletes a node.
   pub fn Node_Close (&self, pNode: NODE) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_FABRIC, kSNEEZE_ABI_METHOD_FABRIC_NODE_CLOSE);

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

   pub fn Rotation (&self, dX: f64, dY: f64, dZ: f64, dW: f64)
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NODE, kSNEEZE_ABI_METHOD_NODE_ROTATION);

      pPacket.Write_Qword  (self.m_qwComposed);
      pPacket.Write_Double (dX);
      pPacket.Write_Double (dY);
      pPacket.Write_Double (dZ);
      pPacket.Write_Double (dW);

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

// ---------------------------------------------------------------------------
// NETWORK - the fabric's window onto the network: HTTP through REQUEST, and
// WebSockets through SOCKET. Reached through `HOST::Network`.
//
// A request URL is resolved against the fabric's own URL, exactly like a node's
// resource or a module reference, so "api/state" means the fabric's folder and
// "/api/state" means the host root.
//
// A socket URL is not resolved: it must be an absolute ws:// or wss:// URL, the
// same rule the browser's WebSocket constructor applies.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct NETWORK
{
   pub (crate) m_twFabricIx: u64,
}

impl NETWORK
{
   /// Opens one HTTP exchange. The returned REQUEST is invalid (`IsValid` false)
   /// if the host refused, which happens when the URL is empty or the fabric is
   /// no longer live.
   pub fn Request_Open (&self, eVerb: eSNEEZE_ABI_REQUEST_VERB, sUrl: &str) -> REQUEST
   {
      self.Request_Open_Ex (eVerb, sUrl, "")
   }

   /// As `Request_Open`, but pins the response to a subresource-integrity hash
   /// ("sha256-<hex>"); the request fails if the bytes do not match.
   pub fn Request_Open_Ex (&self, eVerb: eSNEEZE_ABI_REQUEST_VERB, sUrl: &str, sIntegrity: &str) -> REQUEST
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NETWORK, kSNEEZE_ABI_METHOD_NETWORK_REQUEST_OPEN);

      pPacket.Write_Qword  (self.m_twFabricIx);
      pPacket.Write_Number (eVerb as i32);
      pPacket.Write_Text   (sUrl);
      pPacket.Write_Text   (sIntegrity);

      REQUEST { m_twRequestIx: pPacket.Send () as u64 }
   }

   /// Opens one WebSocket connection. The returned SOCKET is invalid (`IsValid`
   /// false) if the host refused, which happens when the URL is not an absolute
   /// ws:// or wss:// URL or the fabric is no longer live.
   pub fn Socket_Open (&self, sUrl: &str) -> SOCKET
   {
      self.Socket_Open_Ex (sUrl, "")
   }

   /// As `Socket_Open`, but offers subprotocols, comma-separated in order of
   /// preference.
   pub fn Socket_Open_Ex (&self, sUrl: &str, sProtocol: &str) -> SOCKET
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NETWORK, kSNEEZE_ABI_METHOD_NETWORK_SOCKET_OPEN);

      pPacket.Write_Qword (self.m_twFabricIx);
      pPacket.Write_Text  (sUrl);
      pPacket.Write_Text  (sProtocol);

      SOCKET { m_twSocketIx: pPacket.Send () as u64 }
   }
}

// ---------------------------------------------------------------------------
// REQUEST - one HTTP exchange, shaped like the browser's XMLHttpRequest. Open one
// from `HOST::Network`, configure it, send it, and read the answer when
// `INSTANCE::Request` fires. A REQUEST is a thin copyable handle, so passing it
// around costs nothing and every copy names the same exchange.
//
// Lifetime is the guest's: `Close` is the mirror of `Request_Open` and must be
// called, or the host keeps the response buffered for the fabric's life. A GET is
// cached like any other engine fetch; every other verb bypasses the cache.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct REQUEST
{
   m_twRequestIx: u64,
}

impl REQUEST
{
   pub fn New (twRequestIx: u64) -> Self { REQUEST { m_twRequestIx: twRequestIx } }

   pub fn Index (&self) -> u64 { self.m_twRequestIx }

   /// True unless the opening call failed.
   pub fn IsValid (&self) -> bool { self.m_twRequestIx != 0 }

   /// Sets one request header. Before the send; ignored afterwards.
   pub fn Header_Set (&self, sName: &str, sValue: &str) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NETWORK, kSNEEZE_ABI_METHOD_NETWORK_REQUEST_HEADER_SET);

      pPacket.Write_Qword (self.m_twRequestIx);
      pPacket.Write_Text  (sName);
      pPacket.Write_Text  (sValue);

      pPacket.Send () != 0
   }

   /// Caps this exchange at nMilli milliseconds. Before the send.
   pub fn Timeout_Set (&self, nMilli: i32) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NETWORK, kSNEEZE_ABI_METHOD_NETWORK_REQUEST_TIMEOUT_SET);

      pPacket.Write_Qword  (self.m_twRequestIx);
      pPacket.Write_Number (nMilli);

      pPacket.Send () != 0
   }

   /// Issues the request with no body - once per REQUEST.
   pub fn Send (&self) -> bool
   {
      self.Issue (core::ptr::null (), 0)
   }

   /// Issues the request with a UTF-8 text body. Meaningless on a GET or HEAD.
   pub fn Send_Text (&self, sText: &str) -> bool
   {
      self.Issue (sText.as_ptr (), sText.len ())
   }

   /// Issues the request with a binary body. Meaningless on a GET or HEAD.
   pub fn Send_Bytes (&self, aByte: &[u8]) -> bool
   {
      self.Issue (aByte.as_ptr (), aByte.len ())
   }

   /// Stops delivery to this module. The handle stays alive to be read and closed.
   pub fn Abort (&self) -> bool
   {
      self.Scalar (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_ABORT) != 0
   }

   /// Releases the handle and discards the response. The mirror of Request_Open.
   pub fn Close (&self) -> bool
   {
      self.Scalar (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_CLOSE) != 0
   }

   /// Where the exchange stands, the analog of XHR's readyState.
   pub fn State (&self) -> eSNEEZE_ABI_REQUEST_STATE
   {
      eSNEEZE_ABI_REQUEST_STATE::From_Value (self.Scalar (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_STATE))
   }

   /// The HTTP status, or 0 if the server never answered.
   pub fn Status (&self) -> i32 { self.Scalar (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_STATUS) as i32 }

   /// The response byte count.
   pub fn Size (&self) -> i64 { self.Scalar (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_SIZE) }

   /// True if the engine answered this from its cache instead of the network.
   pub fn IsCached (&self) -> bool { self.Scalar (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_IS_CACHED) != 0 }

   /// The reason phrase matching `Status`.
   pub fn Status_Text (&self) -> String { self.Value_Get (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_STATUS_TEXT, None) }

   /// The final URL, after any redirects.
   pub fn Url (&self) -> String { self.Value_Get (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_URL, None) }

   /// Every response header, as CRLF-separated "Name: value" lines.
   pub fn Header_All (&self) -> String { self.Value_Get (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_HEADER_ALL, None) }

   /// The response's content type.
   pub fn Content_Type (&self) -> String { self.Value_Get (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_CONTENT_TYPE, None) }

   /// The transport error, empty on success. Not an HTTP status - a 404 is a
   /// successful exchange with no error text.
   pub fn Error (&self) -> String { self.Value_Get (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_ERROR, None) }

   /// One response header by name, empty if absent.
   pub fn Header (&self, sName: &str) -> String
   {
      self.Value_Get (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_HEADER_GET, Some (sName))
   }

   /// The response body as text (XHR responseText). Invalid UTF-8 yields empty.
   pub fn Text (&self) -> String
   {
      String::from_utf8 (self.Body ()).unwrap_or_default ()
   }

   /// The response body. Bodies are the one read that can be megabytes, so this
   /// asks for the size first (a zero-length buffer is a size query the host
   /// answers without writing) and then reads it exactly, rather than probing into
   /// a small buffer the way the header-sized strings above do.
   pub fn Body (&self) -> Vec<u8>
   {
      let mut aResult: Vec<u8> = Vec::new ();
      let     nNeeded          = self.Value_Into (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_BODY, None, core::ptr::null (), 0);

      if nNeeded > 0
      {
         let mut aByte = vec![0u8; nNeeded as usize];
         let     nRead = self.Value_Into (kSNEEZE_ABI_METHOD_NETWORK_REQUEST_BODY, None, aByte.as_ptr (), aByte.len ());

         // The body cannot grow between the two calls (the host snapshots it on
         // completion), so a short read means the handle went away underneath us.
         if nRead > 0
         {
            if (nRead as usize) < aByte.len ()
            {
               aByte.truncate (nRead as usize);
            }

            aResult = aByte;
         }
      }

      aResult
   }

   // The shape shared by every handle-only scalar read (state/status/size/cached).
   fn Scalar (&self, wMethod: u16) -> i64
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NETWORK, wMethod);

      pPacket.Write_Qword (self.m_twRequestIx);

      pPacket.Send ()
   }

   // The send, with an optional body the three public forms supply differently.
   fn Issue (&self, pByte: *const u8, nLength: usize) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NETWORK, kSNEEZE_ABI_METHOD_NETWORK_REQUEST_SEND);

      pPacket.Write_Qword (self.m_twRequestIx);
      pPacket.Write_Bytes (pByte, nLength);

      pPacket.Send () != 0
   }

   // One out-buffer read. sName carries the header name for HEADER_GET and is None
   // for every other read, which takes no argument beyond the handle.
   fn Value_Into (&self, wMethod: u16, sName: Option<&str>, pByte: *const u8, nLength: usize) -> i64
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NETWORK, wMethod);

      pPacket.Write_Qword (self.m_twRequestIx);

      if let Some (sText) = sName
      {
         pPacket.Write_Text (sText);
      }

      pPacket.Write_Bytes (pByte, nLength);

      pPacket.Send ()
   }

   // The STORAGE::Get dance: one probe into a small buffer, then one exact re-read
   // if the value did not fit. Every string read this way is header-sized, so the
   // probe almost always wins.
   fn Value_Get (&self, wMethod: u16, sName: Option<&str>) -> String
   {
      let mut sResult = String::new ();
      let mut aByte   = vec![0u8; 256];
      let     nProbe  = self.Value_Into (wMethod, sName, aByte.as_ptr (), aByte.len ());

      if nProbe > 0
      {
         let mut nSizeNeeded = nProbe as usize;
         let mut bValid      = true;

         if nSizeNeeded > aByte.len ()
         {
            aByte = vec![0u8; nSizeNeeded];

            let nAgain = self.Value_Into (wMethod, sName, aByte.as_ptr (), aByte.len ());

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
            sResult = String::from_utf8 (aByte).unwrap_or_default ();
         }
      }

      sResult
   }
}

// ---------------------------------------------------------------------------
// SOCKET - one WebSocket connection, shaped like the browser's. Open one from
// `HOST::Network`, then let the four INSTANCE hooks drive it: `Socket_Opened`,
// `Socket_Received`, `Socket_Failed`, `Socket_Closed`. Like REQUEST it is a thin
// copyable handle, so every copy names the same connection.
//
// Two calls end a socket, and they are not the same one. `Close` runs the closing
// handshake and leaves the handle readable, so a guest can still ask what the
// close code was. `Free` is the mirror of `Socket_Open` and must be called, or
// the host holds the connection for the fabric's life.
//
// Every `Socket_Received` must be followed by a `Recv`: a message stays queued
// until it is taken, and a guest that stops taking them eventually overflows the
// queue, after which messages are dropped and `Error` says so.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct SOCKET
{
   m_twSocketIx: u64,
}

impl SOCKET
{
   pub fn New (twSocketIx: u64) -> Self { SOCKET { m_twSocketIx: twSocketIx } }

   pub fn Index (&self) -> u64 { self.m_twSocketIx }

   /// True unless the opening call failed.
   pub fn IsValid (&self) -> bool { self.m_twSocketIx != 0 }

   /// Sends a UTF-8 text frame. Refused unless the socket is OPEN.
   pub fn Send_Text (&self, sText: &str) -> bool
   {
      self.Issue (kSNEEZE_ABI_METHOD_NETWORK_SOCKET_SEND_TEXT, sText.as_ptr (), sText.len ())
   }

   /// Sends a binary frame. Refused unless the socket is OPEN.
   pub fn Send_Bytes (&self, aByte: &[u8]) -> bool
   {
      self.Issue (kSNEEZE_ABI_METHOD_NETWORK_SOCKET_SEND_BINARY, aByte.as_ptr (), aByte.len ())
   }

   /// Runs the closing handshake with a normal-closure code and no reason.
   pub fn Close (&self) -> bool
   {
      self.Close_Ex (1000, "")
   }

   /// Runs the closing handshake with a specific code and reason (the protocol
   /// caps the reason at 123 bytes).
   pub fn Close_Ex (&self, wCode: i32, sReason: &str) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NETWORK, kSNEEZE_ABI_METHOD_NETWORK_SOCKET_CLOSE);

      pPacket.Write_Qword  (self.m_twSocketIx);
      pPacket.Write_Number (wCode);
      pPacket.Write_Text   (sReason);

      pPacket.Send () != 0
   }

   /// Releases the handle. The mirror of `Socket_Open`.
   pub fn Free (&self) -> bool
   {
      self.Scalar (kSNEEZE_ABI_METHOD_NETWORK_SOCKET_FREE) != 0
   }

   /// Where the connection stands, mirroring `WebSocket.readyState`.
   pub fn State (&self) -> eSNEEZE_ABI_SOCKET_STATE
   {
      eSNEEZE_ABI_SOCKET_STATE::From_Value (self.Scalar (kSNEEZE_ABI_METHOD_NETWORK_SOCKET_STATE))
   }

   /// Bytes handed to the socket but not yet on the wire (bufferedAmount).
   pub fn Buffered (&self) -> i64 { self.Scalar (kSNEEZE_ABI_METHOD_NETWORK_SOCKET_BUFFERED) }

   /// The socket's URL, known from the moment it opens.
   pub fn Url (&self) -> String { self.Value_Get (kSNEEZE_ABI_METHOD_NETWORK_SOCKET_URL) }

   /// The negotiated subprotocol - empty until the socket opens, and empty after
   /// if none was agreed.
   pub fn Protocol (&self) -> String { self.Value_Get (kSNEEZE_ABI_METHOD_NETWORK_SOCKET_PROTOCOL) }

   /// The last error, empty unless something failed.
   pub fn Error (&self) -> String { self.Value_Get (kSNEEZE_ABI_METHOD_NETWORK_SOCKET_ERROR) }

   /// Takes the head of the receive queue as text. Invalid UTF-8 yields empty.
   pub fn Recv_Text (&self) -> String
   {
      String::from_utf8 (self.Recv ()).unwrap_or_default ()
   }

   /// Takes the head of the receive queue, empty when nothing is waiting. A
   /// message is only taken once it has somewhere to fit, so the size query first
   /// is not merely an optimization - it is what keeps a message that would not
   /// fit from being consumed and lost.
   pub fn Recv (&self) -> Vec<u8>
   {
      let mut aResult: Vec<u8> = Vec::new ();
      let     nNeeded          = self.Value_Into (kSNEEZE_ABI_METHOD_NETWORK_SOCKET_RECV, core::ptr::null (), 0);

      if nNeeded > 0
      {
         let mut aByte = vec![0u8; nNeeded as usize];
         let     nRead = self.Value_Into (kSNEEZE_ABI_METHOD_NETWORK_SOCKET_RECV, aByte.as_ptr (), aByte.len ());

         if nRead > 0
         {
            if (nRead as usize) < aByte.len ()
            {
               aByte.truncate (nRead as usize);
            }

            aResult = aByte;
         }
      }

      aResult
   }

   // The three shapes every socket method past the open takes, mirroring
   // REQUEST's: a handle-only scalar, a frame send, and an out-buffer read.
   fn Scalar (&self, wMethod: u16) -> i64
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NETWORK, wMethod);

      pPacket.Write_Qword (self.m_twSocketIx);

      pPacket.Send ()
   }

   fn Issue (&self, wMethod: u16, pByte: *const u8, nLength: usize) -> bool
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NETWORK, wMethod);

      pPacket.Write_Qword (self.m_twSocketIx);
      pPacket.Write_Bytes (pByte, nLength);

      pPacket.Send () != 0
   }

   fn Value_Into (&self, wMethod: u16, pByte: *const u8, nLength: usize) -> i64
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_NETWORK, wMethod);

      pPacket.Write_Qword (self.m_twSocketIx);
      pPacket.Write_Bytes (pByte, nLength);

      pPacket.Send ()
   }

   // The probe-then-reread string read, as REQUEST does it.
   fn Value_Get (&self, wMethod: u16) -> String
   {
      let mut sResult = String::new ();
      let mut aByte   = vec![0u8; 256];
      let     nProbe  = self.Value_Into (wMethod, aByte.as_ptr (), aByte.len ());

      if nProbe > 0
      {
         let mut nSizeNeeded = nProbe as usize;
         let mut bValid      = true;

         if nSizeNeeded > aByte.len ()
         {
            aByte = vec![0u8; nSizeNeeded];

            let nAgain = self.Value_Into (wMethod, aByte.as_ptr (), aByte.len ());

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
            sResult = String::from_utf8 (aByte).unwrap_or_default ();
         }
      }

      sResult
   }
}
