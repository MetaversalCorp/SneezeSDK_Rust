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

//! `MAP_SERVICE` - the 592-byte map-service connection wire struct and its
//! fluent builder. Fill one in guest memory (from the module's own knowledge,
//! or from `SERVICES::Get (name)` which it parses), then hand it to
//! `SCENE::Node_Map_Service`, which passes its (offset, length) to the host. Layout
//! mirrors `sdk/include/sneeze_abi.h` (SNEEZE_ABI_MAP_SERVICE) field for field.

#![allow(non_snake_case)]

#[repr(C, packed)]
pub struct MAP_SERVICE
{
   sNamespace:  [u8; 32],
   sService:    [u8; 32],
   sConnect:    [u8; 256],
   sRootUrl:    [u8; 256],
   bAuth:       u8,
   abReserved:  [u8; 5],
   wClass:      u16,
   twObjectIx:  u64,
}

const _: () = assert! (core::mem::size_of::<MAP_SERVICE> () == 592);

impl MAP_SERVICE
{
   pub const SIZE: usize = 592;

   /// A zeroed struct; fill it with the builder methods below.
   pub fn New () -> Self
   {
      unsafe { core::mem::zeroed () }
   }

   pub fn Namespace (&mut self, sNamespace: &str) -> &mut Self { Self::Field_Set (&mut self.sNamespace, sNamespace); self }
   pub fn Service   (&mut self, sService:   &str) -> &mut Self { Self::Field_Set (&mut self.sService,   sService);   self }
   pub fn Connect   (&mut self, sConnect:   &str) -> &mut Self { Self::Field_Set (&mut self.sConnect,   sConnect);   self }
   pub fn RootUrl   (&mut self, sRootUrl:   &str) -> &mut Self { Self::Field_Set (&mut self.sRootUrl,   sRootUrl);   self }

   pub fn Auth     (&mut self, bAuth: bool)     -> &mut Self { self.bAuth      = if bAuth { 1 } else { 0 }; self }
   pub fn Class    (&mut self, wClass: u16)     -> &mut Self { self.wClass     = wClass;                    self }
   pub fn ObjectIx (&mut self, twObjectIx: u64) -> &mut Self { self.twObjectIx = twObjectIx;                self }

   pub fn Pointer (&self) -> *const u8
   {
      self as *const MAP_SERVICE as *const u8
   }

   // Copy a &str into a fixed NUL-padded field, truncating and always leaving a
   // terminator (the last byte stays zero).
   fn Field_Set (aField: &mut [u8], sValue: &str)
   {
      for nByte in aField.iter_mut ()
      {
         *nByte = 0;
      }

      let aByte   = sValue.as_bytes ();
      let nLength = if aByte.len () < aField.len () - 1 { aByte.len () } else { aField.len () - 1 };

      aField[..nLength].copy_from_slice (&aByte[..nLength]);
   }
}
