// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod p2p {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Content {
  NONE = 0,
  proposer = 1,
  voter = 2,
  transaction = 3,

}

pub const ENUM_MIN_CONTENT: u8 = 0;
pub const ENUM_MAX_CONTENT: u8 = 3;

impl<'a> flatbuffers::Follow<'a> for Content {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for Content {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const Content;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const Content;
    unsafe { *p }
  }
}

impl flatbuffers::Push for Content {
    type Output = Content;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<Content>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
pub const ENUM_VALUES_CONTENT:[Content; 4] = [
  Content::NONE,
  Content::proposer,
  Content::voter,
  Content::transaction
];

#[allow(non_camel_case_types)]
pub const ENUM_NAMES_CONTENT:[&'static str; 4] = [
    "NONE",
    "proposer",
    "voter",
    "transaction"
];

pub fn enum_name_content(e: Content) -> &'static str {
  let index = e as u8;
  ENUM_NAMES_CONTENT[index as usize]
}

pub struct ContentUnionTableOffset {}
// struct SHA256, aligned to 8
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHA256 {
  first_: u64,
  second_: u64,
  third_: u64,
  fourth_: u64,
} // pub struct SHA256
impl flatbuffers::SafeSliceAccess for SHA256 {}
impl<'a> flatbuffers::Follow<'a> for SHA256 {
  type Inner = &'a SHA256;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a SHA256>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a SHA256 {
  type Inner = &'a SHA256;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<SHA256>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for SHA256 {
    type Output = SHA256;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const SHA256 as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b SHA256 {
    type Output = SHA256;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const SHA256 as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl SHA256 {
  pub fn new<'a>(_first: u64, _second: u64, _third: u64, _fourth: u64) -> Self {
    SHA256 {
      first_: _first.to_little_endian(),
      second_: _second.to_little_endian(),
      third_: _third.to_little_endian(),
      fourth_: _fourth.to_little_endian(),

    }
  }
  pub fn first<'a>(&'a self) -> u64 {
    self.first_.from_little_endian()
  }
  pub fn second<'a>(&'a self) -> u64 {
    self.second_.from_little_endian()
  }
  pub fn third<'a>(&'a self) -> u64 {
    self.third_.from_little_endian()
  }
  pub fn fourth<'a>(&'a self) -> u64 {
    self.fourth_.from_little_endian()
  }
}

// struct UInt128, aligned to 8
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UInt128 {
  high_: u64,
  low_: u64,
} // pub struct UInt128
impl flatbuffers::SafeSliceAccess for UInt128 {}
impl<'a> flatbuffers::Follow<'a> for UInt128 {
  type Inner = &'a UInt128;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a UInt128>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a UInt128 {
  type Inner = &'a UInt128;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<UInt128>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for UInt128 {
    type Output = UInt128;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const UInt128 as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b UInt128 {
    type Output = UInt128;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const UInt128 as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl UInt128 {
  pub fn new<'a>(_high: u64, _low: u64) -> Self {
    UInt128 {
      high_: _high.to_little_endian(),
      low_: _low.to_little_endian(),

    }
  }
  pub fn high<'a>(&'a self) -> u64 {
    self.high_.from_little_endian()
  }
  pub fn low<'a>(&'a self) -> u64 {
    self.low_.from_little_endian()
  }
}

// struct Coin, aligned to 8
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coin {
  transaction_: SHA256,
  id_: u8,
  padding0__: u8,  padding1__: u16,  padding2__: u32,
} // pub struct Coin
impl flatbuffers::SafeSliceAccess for Coin {}
impl<'a> flatbuffers::Follow<'a> for Coin {
  type Inner = &'a Coin;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Coin>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Coin {
  type Inner = &'a Coin;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Coin>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Coin {
    type Output = Coin;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Coin as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Coin {
    type Output = Coin;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Coin as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl Coin {
  pub fn new<'a>(_transaction: &'a SHA256, _id: u8) -> Self {
    Coin {
      transaction_: *_transaction,
      id_: _id.to_little_endian(),

      padding0__: 0,padding1__: 0,padding2__: 0,
    }
  }
  pub fn transaction<'a>(&'a self) -> &'a SHA256 {
    &self.transaction_
  }
  pub fn id<'a>(&'a self) -> u8 {
    self.id_.from_little_endian()
  }
}

// struct Output, aligned to 8
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Output {
  value_: u32,
  padding0__: u32,
  recipient_: SHA256,
} // pub struct Output
impl flatbuffers::SafeSliceAccess for Output {}
impl<'a> flatbuffers::Follow<'a> for Output {
  type Inner = &'a Output;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Output>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Output {
  type Inner = &'a Output;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Output>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Output {
    type Output = Output;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Output as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Output {
    type Output = Output;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Output as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl Output {
  pub fn new<'a>(_value: u32, _recipient: &'a SHA256) -> Self {
    Output {
      value_: _value.to_little_endian(),
      recipient_: *_recipient,

      padding0__: 0,
    }
  }
  pub fn value<'a>(&'a self) -> u32 {
    self.value_.from_little_endian()
  }
  pub fn recipient<'a>(&'a self) -> &'a SHA256 {
    &self.recipient_
  }
}

// struct Signature, aligned to 8
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Signature {
  first_: u64,
  second_: u64,
  third_: u64,
  fourth_: u64,
} // pub struct Signature
impl flatbuffers::SafeSliceAccess for Signature {}
impl<'a> flatbuffers::Follow<'a> for Signature {
  type Inner = &'a Signature;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Signature>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Signature {
  type Inner = &'a Signature;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Signature>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Signature {
    type Output = Signature;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Signature as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Signature {
    type Output = Signature;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Signature as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl Signature {
  pub fn new<'a>(_first: u64, _second: u64, _third: u64, _fourth: u64) -> Self {
    Signature {
      first_: _first.to_little_endian(),
      second_: _second.to_little_endian(),
      third_: _third.to_little_endian(),
      fourth_: _fourth.to_little_endian(),

    }
  }
  pub fn first<'a>(&'a self) -> u64 {
    self.first_.from_little_endian()
  }
  pub fn second<'a>(&'a self) -> u64 {
    self.second_.from_little_endian()
  }
  pub fn third<'a>(&'a self) -> u64 {
    self.third_.from_little_endian()
  }
  pub fn fourth<'a>(&'a self) -> u64 {
    self.fourth_.from_little_endian()
  }
}

// struct PublicKey, aligned to 8
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PublicKey {
  first_: u64,
  second_: u64,
  third_: u64,
  fourth_: u64,
} // pub struct PublicKey
impl flatbuffers::SafeSliceAccess for PublicKey {}
impl<'a> flatbuffers::Follow<'a> for PublicKey {
  type Inner = &'a PublicKey;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a PublicKey>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a PublicKey {
  type Inner = &'a PublicKey;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<PublicKey>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for PublicKey {
    type Output = PublicKey;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const PublicKey as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b PublicKey {
    type Output = PublicKey;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const PublicKey as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl PublicKey {
  pub fn new<'a>(_first: u64, _second: u64, _third: u64, _fourth: u64) -> Self {
    PublicKey {
      first_: _first.to_little_endian(),
      second_: _second.to_little_endian(),
      third_: _third.to_little_endian(),
      fourth_: _fourth.to_little_endian(),

    }
  }
  pub fn first<'a>(&'a self) -> u64 {
    self.first_.from_little_endian()
  }
  pub fn second<'a>(&'a self) -> u64 {
    self.second_.from_little_endian()
  }
  pub fn third<'a>(&'a self) -> u64 {
    self.third_.from_little_endian()
  }
  pub fn fourth<'a>(&'a self) -> u64 {
    self.fourth_.from_little_endian()
  }
}

// struct Authorization, aligned to 8
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Authorization {
  signature_: Signature,
  pubkey_: PublicKey,
} // pub struct Authorization
impl flatbuffers::SafeSliceAccess for Authorization {}
impl<'a> flatbuffers::Follow<'a> for Authorization {
  type Inner = &'a Authorization;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Authorization>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Authorization {
  type Inner = &'a Authorization;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Authorization>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Authorization {
    type Output = Authorization;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Authorization as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Authorization {
    type Output = Authorization;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Authorization as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl Authorization {
  pub fn new<'a>(_signature: &'a Signature, _pubkey: &'a PublicKey) -> Self {
    Authorization {
      signature_: *_signature,
      pubkey_: *_pubkey,

    }
  }
  pub fn signature<'a>(&'a self) -> &'a Signature {
    &self.signature_
  }
  pub fn pubkey<'a>(&'a self) -> &'a PublicKey {
    &self.pubkey_
  }
}

// struct Header, aligned to 8
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Header {
  parent_: SHA256,
  timestamp_: UInt128,
  nonce_: u32,
  padding0__: u32,
  content_merkle_root_: SHA256,
} // pub struct Header
impl flatbuffers::SafeSliceAccess for Header {}
impl<'a> flatbuffers::Follow<'a> for Header {
  type Inner = &'a Header;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Header>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Header {
  type Inner = &'a Header;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Header>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Header {
    type Output = Header;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const Header as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b Header {
    type Output = Header;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const Header as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}


impl Header {
  pub fn new<'a>(_parent: &'a SHA256, _timestamp: &'a UInt128, _nonce: u32, _content_merkle_root: &'a SHA256) -> Self {
    Header {
      parent_: *_parent,
      timestamp_: *_timestamp,
      nonce_: _nonce.to_little_endian(),
      content_merkle_root_: *_content_merkle_root,

      padding0__: 0,
    }
  }
  pub fn parent<'a>(&'a self) -> &'a SHA256 {
    &self.parent_
  }
  pub fn timestamp<'a>(&'a self) -> &'a UInt128 {
    &self.timestamp_
  }
  pub fn nonce<'a>(&'a self) -> u32 {
    self.nonce_.from_little_endian()
  }
  pub fn content_merkle_root<'a>(&'a self) -> &'a SHA256 {
    &self.content_merkle_root_
  }
}

pub enum TransactionOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Transaction<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Transaction<'a> {
    type Inner = Transaction<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Transaction<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Transaction {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TransactionArgs<'args>) -> flatbuffers::WIPOffset<Transaction<'bldr>> {
      let mut builder = TransactionBuilder::new(_fbb);
      if let Some(x) = args.authorizations { builder.add_authorizations(x); }
      if let Some(x) = args.outputs { builder.add_outputs(x); }
      if let Some(x) = args.inputs { builder.add_inputs(x); }
      builder.finish()
    }

    pub const VT_INPUTS: flatbuffers::VOffsetT = 4;
    pub const VT_OUTPUTS: flatbuffers::VOffsetT = 6;
    pub const VT_AUTHORIZATIONS: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn inputs(&self) -> Option<&'a [Coin]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<Coin>>>(Transaction::VT_INPUTS, None).map(|v| v.safe_slice() )
  }
  #[inline]
  pub fn outputs(&self) -> Option<&'a [Output]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<Output>>>(Transaction::VT_OUTPUTS, None).map(|v| v.safe_slice() )
  }
  #[inline]
  pub fn authorizations(&self) -> Option<&'a [Authorization]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<Authorization>>>(Transaction::VT_AUTHORIZATIONS, None).map(|v| v.safe_slice() )
  }
}

pub struct TransactionArgs<'a> {
    pub inputs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , Coin>>>,
    pub outputs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , Output>>>,
    pub authorizations: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , Authorization>>>,
}
impl<'a> Default for TransactionArgs<'a> {
    #[inline]
    fn default() -> Self {
        TransactionArgs {
            inputs: None,
            outputs: None,
            authorizations: None,
        }
    }
}
pub struct TransactionBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TransactionBuilder<'a, 'b> {
  #[inline]
  pub fn add_inputs(&mut self, inputs: flatbuffers::WIPOffset<flatbuffers::Vector<'b , Coin>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Transaction::VT_INPUTS, inputs);
  }
  #[inline]
  pub fn add_outputs(&mut self, outputs: flatbuffers::WIPOffset<flatbuffers::Vector<'b , Output>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Transaction::VT_OUTPUTS, outputs);
  }
  #[inline]
  pub fn add_authorizations(&mut self, authorizations: flatbuffers::WIPOffset<flatbuffers::Vector<'b , Authorization>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Transaction::VT_AUTHORIZATIONS, authorizations);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TransactionBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TransactionBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Transaction<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum ProposerContentOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct ProposerContent<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ProposerContent<'a> {
    type Inner = ProposerContent<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> ProposerContent<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        ProposerContent {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ProposerContentArgs<'args>) -> flatbuffers::WIPOffset<ProposerContent<'bldr>> {
      let mut builder = ProposerContentBuilder::new(_fbb);
      if let Some(x) = args.proposer_refs { builder.add_proposer_refs(x); }
      if let Some(x) = args.transaction_refs { builder.add_transaction_refs(x); }
      builder.finish()
    }

    pub const VT_TRANSACTION_REFS: flatbuffers::VOffsetT = 4;
    pub const VT_PROPOSER_REFS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn transaction_refs(&self) -> Option<&'a [SHA256]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<SHA256>>>(ProposerContent::VT_TRANSACTION_REFS, None).map(|v| v.safe_slice() )
  }
  #[inline]
  pub fn proposer_refs(&self) -> Option<&'a [SHA256]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<SHA256>>>(ProposerContent::VT_PROPOSER_REFS, None).map(|v| v.safe_slice() )
  }
}

pub struct ProposerContentArgs<'a> {
    pub transaction_refs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , SHA256>>>,
    pub proposer_refs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , SHA256>>>,
}
impl<'a> Default for ProposerContentArgs<'a> {
    #[inline]
    fn default() -> Self {
        ProposerContentArgs {
            transaction_refs: None,
            proposer_refs: None,
        }
    }
}
pub struct ProposerContentBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ProposerContentBuilder<'a, 'b> {
  #[inline]
  pub fn add_transaction_refs(&mut self, transaction_refs: flatbuffers::WIPOffset<flatbuffers::Vector<'b , SHA256>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ProposerContent::VT_TRANSACTION_REFS, transaction_refs);
  }
  #[inline]
  pub fn add_proposer_refs(&mut self, proposer_refs: flatbuffers::WIPOffset<flatbuffers::Vector<'b , SHA256>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ProposerContent::VT_PROPOSER_REFS, proposer_refs);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ProposerContentBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ProposerContentBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ProposerContent<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum VoterContentOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct VoterContent<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for VoterContent<'a> {
    type Inner = VoterContent<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> VoterContent<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        VoterContent {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args VoterContentArgs<'args>) -> flatbuffers::WIPOffset<VoterContent<'bldr>> {
      let mut builder = VoterContentBuilder::new(_fbb);
      if let Some(x) = args.proposer_votes { builder.add_proposer_votes(x); }
      if let Some(x) = args.voter_parent { builder.add_voter_parent(x); }
      builder.finish()
    }

    pub const VT_VOTER_PARENT: flatbuffers::VOffsetT = 4;
    pub const VT_PROPOSER_VOTES: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn voter_parent(&self) -> Option<&'a SHA256> {
    self._tab.get::<SHA256>(VoterContent::VT_VOTER_PARENT, None)
  }
  #[inline]
  pub fn proposer_votes(&self) -> Option<&'a [SHA256]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<SHA256>>>(VoterContent::VT_PROPOSER_VOTES, None).map(|v| v.safe_slice() )
  }
}

pub struct VoterContentArgs<'a> {
    pub voter_parent: Option<&'a  SHA256>,
    pub proposer_votes: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , SHA256>>>,
}
impl<'a> Default for VoterContentArgs<'a> {
    #[inline]
    fn default() -> Self {
        VoterContentArgs {
            voter_parent: None,
            proposer_votes: None,
        }
    }
}
pub struct VoterContentBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> VoterContentBuilder<'a, 'b> {
  #[inline]
  pub fn add_voter_parent(&mut self, voter_parent: &'b  SHA256) {
    self.fbb_.push_slot_always::<&SHA256>(VoterContent::VT_VOTER_PARENT, voter_parent);
  }
  #[inline]
  pub fn add_proposer_votes(&mut self, proposer_votes: flatbuffers::WIPOffset<flatbuffers::Vector<'b , SHA256>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(VoterContent::VT_PROPOSER_VOTES, proposer_votes);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> VoterContentBuilder<'a, 'b> {
    let start = _fbb.start_table();
    VoterContentBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<VoterContent<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum TransactionContentOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct TransactionContent<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TransactionContent<'a> {
    type Inner = TransactionContent<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> TransactionContent<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TransactionContent {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TransactionContentArgs<'args>) -> flatbuffers::WIPOffset<TransactionContent<'bldr>> {
      let mut builder = TransactionContentBuilder::new(_fbb);
      if let Some(x) = args.transactions { builder.add_transactions(x); }
      builder.finish()
    }

    pub const VT_TRANSACTIONS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn transactions(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Transaction<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Transaction<'a>>>>>(TransactionContent::VT_TRANSACTIONS, None)
  }
}

pub struct TransactionContentArgs<'a> {
    pub transactions: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Transaction<'a >>>>>,
}
impl<'a> Default for TransactionContentArgs<'a> {
    #[inline]
    fn default() -> Self {
        TransactionContentArgs {
            transactions: None,
        }
    }
}
pub struct TransactionContentBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TransactionContentBuilder<'a, 'b> {
  #[inline]
  pub fn add_transactions(&mut self, transactions: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Transaction<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(TransactionContent::VT_TRANSACTIONS, transactions);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TransactionContentBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TransactionContentBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TransactionContent<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum BlockOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Block<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Block<'a> {
    type Inner = Block<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Block<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Block {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args BlockArgs<'args>) -> flatbuffers::WIPOffset<Block<'bldr>> {
      let mut builder = BlockBuilder::new(_fbb);
      if let Some(x) = args.content { builder.add_content(x); }
      if let Some(x) = args.header { builder.add_header(x); }
      builder.add_content_type(args.content_type);
      builder.finish()
    }

    pub const VT_HEADER: flatbuffers::VOffsetT = 4;
    pub const VT_CONTENT_TYPE: flatbuffers::VOffsetT = 6;
    pub const VT_CONTENT: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn header(&self) -> Option<&'a Header> {
    self._tab.get::<Header>(Block::VT_HEADER, None)
  }
  #[inline]
  pub fn content_type(&self) -> Content {
    self._tab.get::<Content>(Block::VT_CONTENT_TYPE, Some(Content::NONE)).unwrap()
  }
  #[inline]
  pub fn content(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Block::VT_CONTENT, None)
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn content_as_proposer(&self) -> Option<ProposerContent<'a>> {
    if self.content_type() == Content::proposer {
      self.content().map(|u| ProposerContent::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn content_as_voter(&self) -> Option<VoterContent<'a>> {
    if self.content_type() == Content::voter {
      self.content().map(|u| VoterContent::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn content_as_transaction(&self) -> Option<TransactionContent<'a>> {
    if self.content_type() == Content::transaction {
      self.content().map(|u| TransactionContent::init_from_table(u))
    } else {
      None
    }
  }

}

pub struct BlockArgs<'a> {
    pub header: Option<&'a  Header>,
    pub content_type: Content,
    pub content: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for BlockArgs<'a> {
    #[inline]
    fn default() -> Self {
        BlockArgs {
            header: None,
            content_type: Content::NONE,
            content: None,
        }
    }
}
pub struct BlockBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> BlockBuilder<'a, 'b> {
  #[inline]
  pub fn add_header(&mut self, header: &'b  Header) {
    self.fbb_.push_slot_always::<&Header>(Block::VT_HEADER, header);
  }
  #[inline]
  pub fn add_content_type(&mut self, content_type: Content) {
    self.fbb_.push_slot::<Content>(Block::VT_CONTENT_TYPE, content_type, Content::NONE);
  }
  #[inline]
  pub fn add_content(&mut self, content: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Block::VT_CONTENT, content);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> BlockBuilder<'a, 'b> {
    let start = _fbb.start_table();
    BlockBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Block<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum MessageOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Message<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Message<'a> {
    type Inner = Message<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Message<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Message {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MessageArgs<'args>) -> flatbuffers::WIPOffset<Message<'bldr>> {
      let mut builder = MessageBuilder::new(_fbb);
      if let Some(x) = args.blocks { builder.add_blocks(x); }
      if let Some(x) = args.get_blocks { builder.add_get_blocks(x); }
      if let Some(x) = args.new_block_hashes { builder.add_new_block_hashes(x); }
      if let Some(x) = args.pong { builder.add_pong(x); }
      if let Some(x) = args.ping { builder.add_ping(x); }
      builder.finish()
    }

    pub const VT_PING: flatbuffers::VOffsetT = 4;
    pub const VT_PONG: flatbuffers::VOffsetT = 6;
    pub const VT_NEW_BLOCK_HASHES: flatbuffers::VOffsetT = 8;
    pub const VT_GET_BLOCKS: flatbuffers::VOffsetT = 10;
    pub const VT_BLOCKS: flatbuffers::VOffsetT = 12;

  #[inline]
  pub fn ping(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Message::VT_PING, None)
  }
  #[inline]
  pub fn pong(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Message::VT_PONG, None)
  }
  #[inline]
  pub fn new_block_hashes(&self) -> Option<&'a [SHA256]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<SHA256>>>(Message::VT_NEW_BLOCK_HASHES, None).map(|v| v.safe_slice() )
  }
  #[inline]
  pub fn get_blocks(&self) -> Option<&'a [SHA256]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<SHA256>>>(Message::VT_GET_BLOCKS, None).map(|v| v.safe_slice() )
  }
  #[inline]
  pub fn blocks(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Block<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Block<'a>>>>>(Message::VT_BLOCKS, None)
  }
}

pub struct MessageArgs<'a> {
    pub ping: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub pong: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub new_block_hashes: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , SHA256>>>,
    pub get_blocks: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , SHA256>>>,
    pub blocks: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Block<'a >>>>>,
}
impl<'a> Default for MessageArgs<'a> {
    #[inline]
    fn default() -> Self {
        MessageArgs {
            ping: None,
            pong: None,
            new_block_hashes: None,
            get_blocks: None,
            blocks: None,
        }
    }
}
pub struct MessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_ping(&mut self, ping: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_PING, ping);
  }
  #[inline]
  pub fn add_pong(&mut self, pong: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_PONG, pong);
  }
  #[inline]
  pub fn add_new_block_hashes(&mut self, new_block_hashes: flatbuffers::WIPOffset<flatbuffers::Vector<'b , SHA256>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_NEW_BLOCK_HASHES, new_block_hashes);
  }
  #[inline]
  pub fn add_get_blocks(&mut self, get_blocks: flatbuffers::WIPOffset<flatbuffers::Vector<'b , SHA256>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_GET_BLOCKS, get_blocks);
  }
  #[inline]
  pub fn add_blocks(&mut self, blocks: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Block<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_BLOCKS, blocks);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Message<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

}  // pub mod P2P

