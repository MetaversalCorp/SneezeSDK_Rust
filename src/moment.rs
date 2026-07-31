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

//! `MOMENT` - the 44-byte guest-resident wall-clock value produced by `CHRONO`.
//!
//! Like `SNEEZE_ABI_MAPOBJECT` this is a raw `#[repr(C, packed)]` wire struct,
//! but it flows host -> guest: the guest hands the host a zeroed `MOMENT` by
//! (offset, length) and the host fills it in one call - both scalar forms
//! (`tm`, `dt`) plus the full UTC and local calendar breakdowns - so every
//! getter reads a cached field with no host crossing. Setters re-cross (the
//! host owns all civil normalization) and rewrite the struct in place, matching
//! JavaScript `Date`'s mutable semantics.
//!
//! Sub-second is stored once, canonically, as `dwFraction` in 100 ns units;
//! `Tick` (1/64 s) and `Milli` (ms) are derived views. A zeroed `MOMENT`
//! (`bMonth == 0`) is the invalid sentinel.

use crate::abi::*;
use crate::abi::eSNEEZE_ABI_CHRONO_ZONE::{kSNEEZE_ABI_CHRONO_ZONE_UTC, kSNEEZE_ABI_CHRONO_ZONE_LOCAL};
use crate::ffi::PACKET;

// 100 ns units per derived sub-second grain. 1/64 s and 1 ms both divide
// 100 ns evenly (but not each other), so dwFraction round-trips both.
const HNS_PER_TICK: u32 = 156250;   // 1/64 s
const HNS_PER_MS:   u32 = 10000;    // 1 ms

// CHRONO_MOMENT eSource discriminant.
const SOURCE_TIME: i32 = 0;         // tm (1/64 s since 1601)
const SOURCE_DATE: i32 = 1;         // dt (Unix ms)

// Natural C layout (align 4, size 12): identical field offsets to the wire
// struct, but - unlike the packed MOMENT that embeds it - a copied-out CIVIL
// has aligned fields, so getters and format! can read them directly.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SNEEZE_ABI_CIVIL
{
   wYear:      i16,   // full year (2026)
   bMonth:     u8,    // 1-12 (7 = July); 0 = invalid
   bDay:       u8,    // 1-31
   bWeekday:   u8,    // 0-6 (0 = Sunday)
   bHour:      u8,    // 0-23
   bMinute:    u8,    // 0-59
   bSecond:    u8,    // 0-59
   dwFraction: u32,   // sub-second, 100 ns units (0..9,999,999)
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MOMENT
{
   tm:      i64,               // 1/64 s since 1601-01-01, UTC
   dt:      i64,               // Unix ms since 1970-01-01, UTC
   Utc:     SNEEZE_ABI_CIVIL,  // UTC calendar breakdown
   Local:   SNEEZE_ABI_CIVIL,  // local calendar breakdown
   nOffset: i32,               // local offset from UTC, minutes
}

const _: () = assert! (core::mem::size_of::<SNEEZE_ABI_CIVIL> () == 12);
const _: () = assert! (core::mem::size_of::<MOMENT> () == 44);

impl MOMENT
{
   pub const SIZE: usize = 44;

   // --- Construction. A fresh MOMENT is the zeroed invalid sentinel until a
   //     host fill stamps it. The reserved twFabricIx in every CHRONO call is 0:
   //     clocks and civil logic are global, so a MOMENT needs no fabric. ---

   pub fn Null () -> Self { unsafe { core::mem::zeroed () } }

   /// Fills from a `tm` scalar (1/64 s since 1601, UTC).
   pub fn From_Time (tm: i64) -> Self
   {
      let mut m = Self::Null ();
      m.Scalar (SOURCE_TIME, tm);
      m
   }

   /// Fills from a `dt` scalar (Unix ms, UTC).
   pub fn From_Date (dt: i64) -> Self
   {
      let mut m = Self::Null ();
      m.Scalar (SOURCE_DATE, dt);
      m
   }

   /// Builds from civil components in the given zone (normalizes overflow, e.g.
   /// month 13 -> next January). Sub-second is zero.
   pub fn From_Parts (nYear: i32, nMonth: i32, nDay: i32, nHour: i32, nMinute: i32, nSecond: i32, eZone: eSNEEZE_ABI_CHRONO_ZONE) -> Self
   {
      let mut m = Self::Null ();
      m.Set_Civil (eZone as i32, nYear, nMonth, nDay, nHour, nMinute, nSecond, 0);
      m
   }

   /// Parses an ISO-8601 string. A trailing `Z` (or offset) is honored; a
   /// naive string is read in `eZone`.
   pub fn Parse (sText: &str, eZone: eSNEEZE_ABI_CHRONO_ZONE) -> Self
   {
      let mut m = Self::Null ();

      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_CHRONO, kSNEEZE_ABI_METHOD_CHRONO_PARSE);

      pPacket.Write_Qword  (0);
      pPacket.Write_Number (eZone as i32);
      pPacket.Write_Text   (sText);
      pPacket.Write_Bytes  (m.Pointer_Mut (), MOMENT::SIZE);

      pPacket.Send ();

      m
   }

   // Crate-internal fills for the CHRONO / PERFORMANCE facades.

   pub (crate) fn From_Now (twFabricIx: u64) -> Self
   {
      let mut m = Self::Null ();

      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_CHRONO, kSNEEZE_ABI_METHOD_CHRONO_NOW);

      pPacket.Write_Qword (twFabricIx);
      pPacket.Write_Bytes (m.Pointer_Mut (), MOMENT::SIZE);

      pPacket.Send ();

      m
   }

   pub (crate) fn From_Origin (twFabricIx: u64) -> Self
   {
      let mut m = Self::Null ();

      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_PERFORMANCE, kSNEEZE_ABI_METHOD_PERFORMANCE_ORIGIN);

      pPacket.Write_Qword (twFabricIx);
      pPacket.Write_Bytes (m.Pointer_Mut (), MOMENT::SIZE);

      pPacket.Send ();

      m
   }

   // --- Validity + raw scalars (packed reads copy the field out by value) ---

   pub fn IsValid (&self) -> bool { self.Civil_Local ().bMonth != 0 }

   pub fn Time        (&self) -> i64 { self.tm }
   pub fn Date        (&self) -> i64 { self.dt }
   pub fn Zone_Offset (&self) -> i32 { self.nOffset }

   // --- Local calendar accessors (JS Date's getFullYear/getMonth/... family) ---

   pub fn Year    (&self) -> i32 {  self.Civil_Local ().wYear as i32 }
   pub fn Month   (&self) -> i32 {  self.Civil_Local ().bMonth as i32 }
   pub fn Day     (&self) -> i32 {  self.Civil_Local ().bDay as i32 }
   pub fn Weekday (&self) -> i32 {  self.Civil_Local ().bWeekday as i32 }
   pub fn Hour    (&self) -> i32 {  self.Civil_Local ().bHour as i32 }
   pub fn Minute  (&self) -> i32 {  self.Civil_Local ().bMinute as i32 }
   pub fn Second  (&self) -> i32 {  self.Civil_Local ().bSecond as i32 }
   pub fn Milli   (&self) -> i32 { (self.Civil_Local ().dwFraction / HNS_PER_MS)   as i32 }
   pub fn Tick    (&self) -> i32 { (self.Civil_Local ().dwFraction / HNS_PER_TICK) as i32 }

   // --- UTC calendar accessors (JS Date's getUTCFullYear/... family) ---

   pub fn Year_Utc    (&self) -> i32 {  self.Civil_Utc ().wYear as i32 }
   pub fn Month_Utc   (&self) -> i32 {  self.Civil_Utc ().bMonth as i32 }
   pub fn Day_Utc     (&self) -> i32 {  self.Civil_Utc ().bDay as i32 }
   pub fn Weekday_Utc (&self) -> i32 {  self.Civil_Utc ().bWeekday as i32 }
   pub fn Hour_Utc    (&self) -> i32 {  self.Civil_Utc ().bHour as i32 }
   pub fn Minute_Utc  (&self) -> i32 {  self.Civil_Utc ().bMinute as i32 }
   pub fn Second_Utc  (&self) -> i32 {  self.Civil_Utc ().bSecond as i32 }
   pub fn Milli_Utc   (&self) -> i32 { (self.Civil_Utc ().dwFraction / HNS_PER_MS)   as i32 }
   pub fn Tick_Utc    (&self) -> i32 { (self.Civil_Utc ().dwFraction / HNS_PER_TICK) as i32 }

   // --- Local mutators. Each substitutes one component into the cached local
   //     breakdown and re-sends the whole set; the host renormalizes and
   //     rewrites both breakdowns + scalars in place (JS Date set* semantics). ---

   pub fn Year_Set   (&mut self, nYear: i32)   { let c = self.Civil_Local (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_LOCAL as i32,   nYear,        c.bMonth as i32, c.bDay as i32, c.bHour as i32, c.bMinute as i32, c.bSecond as i32, c.dwFraction as i32); }
   pub fn Month_Set  (&mut self, nMonth: i32)  { let c = self.Civil_Local (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_LOCAL as i32, c.wYear as i32,   nMonth,        c.bDay as i32, c.bHour as i32, c.bMinute as i32, c.bSecond as i32, c.dwFraction as i32); }
   pub fn Day_Set    (&mut self, nDay: i32)    { let c = self.Civil_Local (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_LOCAL as i32, c.wYear as i32, c.bMonth as i32,   nDay,        c.bHour as i32, c.bMinute as i32, c.bSecond as i32, c.dwFraction as i32); }
   pub fn Hour_Set   (&mut self, nHour: i32)   { let c = self.Civil_Local (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_LOCAL as i32, c.wYear as i32, c.bMonth as i32, c.bDay as i32,   nHour,        c.bMinute as i32, c.bSecond as i32, c.dwFraction as i32); }
   pub fn Minute_Set (&mut self, nMinute: i32) { let c = self.Civil_Local (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_LOCAL as i32, c.wYear as i32, c.bMonth as i32, c.bDay as i32, c.bHour as i32,   nMinute,        c.bSecond as i32, c.dwFraction as i32); }
   pub fn Second_Set (&mut self, nSecond: i32) { let c = self.Civil_Local (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_LOCAL as i32, c.wYear as i32, c.bMonth as i32, c.bDay as i32, c.bHour as i32, c.bMinute as i32,   nSecond,        c.dwFraction as i32); }
   pub fn Milli_Set  (&mut self, nMilli: i32)  { let c = self.Civil_Local (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_LOCAL as i32, c.wYear as i32, c.bMonth as i32, c.bDay as i32, c.bHour as i32, c.bMinute as i32, c.bSecond as i32, nMilli * HNS_PER_MS as i32); }
   pub fn Tick_Set   (&mut self, nTick: i32)   { let c = self.Civil_Local (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_LOCAL as i32, c.wYear as i32, c.bMonth as i32, c.bDay as i32, c.bHour as i32, c.bMinute as i32, c.bSecond as i32, nTick  * HNS_PER_TICK as i32); }

   // --- UTC mutators (JS Date setUTC* family) ---

   pub fn Year_Utc_Set   (&mut self, nYear: i32)   { let c = self.Civil_Utc (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_UTC as i32,   nYear,        c.bMonth as i32, c.bDay as i32, c.bHour as i32, c.bMinute as i32, c.bSecond as i32, c.dwFraction as i32); }
   pub fn Month_Utc_Set  (&mut self, nMonth: i32)  { let c = self.Civil_Utc (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_UTC as i32, c.wYear as i32,   nMonth,        c.bDay as i32, c.bHour as i32, c.bMinute as i32, c.bSecond as i32, c.dwFraction as i32); }
   pub fn Day_Utc_Set    (&mut self, nDay: i32)    { let c = self.Civil_Utc (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_UTC as i32, c.wYear as i32, c.bMonth as i32,   nDay,        c.bHour as i32, c.bMinute as i32, c.bSecond as i32, c.dwFraction as i32); }
   pub fn Hour_Utc_Set   (&mut self, nHour: i32)   { let c = self.Civil_Utc (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_UTC as i32, c.wYear as i32, c.bMonth as i32, c.bDay as i32,   nHour,        c.bMinute as i32, c.bSecond as i32, c.dwFraction as i32); }
   pub fn Minute_Utc_Set (&mut self, nMinute: i32) { let c = self.Civil_Utc (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_UTC as i32, c.wYear as i32, c.bMonth as i32, c.bDay as i32, c.bHour as i32,   nMinute,        c.bSecond as i32, c.dwFraction as i32); }
   pub fn Second_Utc_Set (&mut self, nSecond: i32) { let c = self.Civil_Utc (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_UTC as i32, c.wYear as i32, c.bMonth as i32, c.bDay as i32, c.bHour as i32, c.bMinute as i32,   nSecond,        c.dwFraction as i32); }
   pub fn Milli_Utc_Set  (&mut self, nMilli: i32)  { let c = self.Civil_Utc (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_UTC as i32, c.wYear as i32, c.bMonth as i32, c.bDay as i32, c.bHour as i32, c.bMinute as i32, c.bSecond as i32, nMilli * HNS_PER_MS as i32); }
   pub fn Tick_Utc_Set   (&mut self, nTick: i32)   { let c = self.Civil_Utc (); self.Set_Civil (kSNEEZE_ABI_CHRONO_ZONE_UTC as i32, c.wYear as i32, c.bMonth as i32, c.bDay as i32, c.bHour as i32, c.bMinute as i32, c.bSecond as i32, nTick  * HNS_PER_TICK as i32); }

   /// Sets the whole instant from a `dt` scalar (Unix ms), JS Date's setTime.
   pub fn Date_Set (&mut self, dt: i64) { self.Scalar (SOURCE_DATE, dt); }

   // --- Formatting ---

   /// ISO-8601 UTC (`YYYY-MM-DDTHH:MM:SS.mmmZ`), built guest-side from the
   /// cached UTC breakdown - no host crossing (JS toISOString).
   pub fn String_Iso (&self) -> String
   {
      let c    = self.Civil_Utc ();
      let nMs  = c.dwFraction / HNS_PER_MS;

      format! ("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z", c.wYear, c.bMonth, c.bDay, c.bHour, c.bMinute, c.bSecond, nMs)
   }

   /// JSON serialization: the ISO-8601 UTC string (JS toJSON).
   pub fn Json (&self) -> String { self.String_Iso () }

   /// Renders via the host's formatter. An empty `sSpec` yields the default
   /// ISO rendering for the zone; a non-empty spec uses strftime-style fields.
   pub fn Format (&self, eZone: eSNEEZE_ABI_CHRONO_ZONE, sSpec: &str) -> String
   {
      let mut sResult = String::new ();
      let mut aByte   = vec![0u8; 64];
      let nProbe      = self.Format_Into (eZone, sSpec, &mut aByte);

      if nProbe > 0
      {
         let mut nNeeded = nProbe as usize;
         let mut bValid  = true;

         if nNeeded > aByte.len ()
         {
            aByte = vec![0u8; nNeeded];
            let nAgain = self.Format_Into (eZone, sSpec, &mut aByte);

            if nAgain > 0 { nNeeded = nAgain as usize; }
            else          { bValid  = false; }
         }

         if bValid
         {
            let nCount = if nNeeded < aByte.len () { nNeeded } else { aByte.len () };

            aByte.truncate (nCount);
            sResult = String::from_utf8 (aByte).unwrap_or_default ();
         }
      }

      sResult
   }

   /// Local default rendering (JS toString).
   pub fn String (&self) -> String
   {
      self.Format (eSNEEZE_ABI_CHRONO_ZONE::kSNEEZE_ABI_CHRONO_ZONE_LOCAL, "")
   }

   /// UTC default rendering (JS toUTCString).
   pub fn String_Utc (&self) -> String
   {
      self.Format (eSNEEZE_ABI_CHRONO_ZONE::kSNEEZE_ABI_CHRONO_ZONE_UTC, "")
   }

   // --- Wire access ---

   pub (crate) fn Pointer (&self) -> *const u8 { self as *const MOMENT as *const u8 }

   // -----------------------------------------------------------------------
   // Internals.
   // -----------------------------------------------------------------------

   // A packed struct forbids references to its fields; copying a Copy field
   // out by value is the sanctioned read, so both civil breakdowns are copied
   // whole and then read through the aligned local.
   fn Civil_Local (&self) -> SNEEZE_ABI_CIVIL { self.Local }
   fn Civil_Utc   (&self) -> SNEEZE_ABI_CIVIL { self.Utc }

   // Provenance for the host's write must come from &mut, not &self.
   fn Pointer_Mut (&mut self) -> *const u8 { self as *mut MOMENT as *const u8 }

   fn Scalar (&mut self, eSource: i32, qwValue: i64)
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_CHRONO, kSNEEZE_ABI_METHOD_CHRONO_MOMENT);

      pPacket.Write_Qword  (0);
      pPacket.Write_Number (eSource);
      pPacket.Write_Qword  (qwValue as u64);
      pPacket.Write_Bytes  (self.Pointer_Mut (), MOMENT::SIZE);

      pPacket.Send ();
   }

   fn Set_Civil (&mut self, eZone: i32, nYear: i32, nMonth: i32, nDay: i32, nHour: i32, nMinute: i32, nSecond: i32, nFraction: i32)
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_CHRONO, kSNEEZE_ABI_METHOD_CHRONO_SET);

      pPacket.Write_Qword  (0);
      pPacket.Write_Number (eZone);
      pPacket.Write_Number (nYear);
      pPacket.Write_Number (nMonth);
      pPacket.Write_Number (nDay);
      pPacket.Write_Number (nHour);
      pPacket.Write_Number (nMinute);
      pPacket.Write_Number (nSecond);
      pPacket.Write_Number (nFraction);
      pPacket.Write_Bytes  (self.Pointer_Mut (), MOMENT::SIZE);

      pPacket.Send ();
   }

   fn Format_Into (&self, eZone: eSNEEZE_ABI_CHRONO_ZONE, sSpec: &str, aByte: &mut [u8]) -> i64
   {
      let mut pPacket = PACKET::New (kSNEEZE_ABI_TYPE_CHRONO, kSNEEZE_ABI_METHOD_CHRONO_FORMAT);

      pPacket.Write_Qword  (0);
      pPacket.Write_Number (eZone as i32);
      pPacket.Write_Text   (sSpec);
      pPacket.Write_Bytes  (self.Pointer (), MOMENT::SIZE);
      pPacket.Write_Bytes  (aByte.as_ptr (), aByte.len ());

      pPacket.Send ()
   }
}
