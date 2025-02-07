// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY:
//
//   ucd-generate dfa --name SENTENCE_BREAK_FWD --minimize --sparse --anchored --state-size 4 src/unicode/fsm/ [snip (arg too long)]
//
// ucd-generate 0.2.12 is available on crates.io.

#[cfg(target_endian = "big")]
lazy_static::lazy_static! {
  pub static ref SENTENCE_BREAK_FWD: ::regex_automata::SparseDFA<&'static [u8], u32> = {
    #[repr(C)]
    struct Aligned<B: ?Sized> {
        _align: [u8; 0],
        bytes: B,
    }

    static ALIGNED: &'static Aligned<[u8]> = &Aligned {
        _align: [],
        bytes: *include_bytes!("sentence_break_fwd.bigendian.dfa"),
    };

    unsafe {
      ::regex_automata::SparseDFA::from_bytes(&ALIGNED.bytes)
    }
  };
}

#[cfg(target_endian = "little")]
lazy_static::lazy_static! {
  pub static ref SENTENCE_BREAK_FWD: ::regex_automata::SparseDFA<&'static [u8], u32> = {
    #[repr(C)]
    struct Aligned<B: ?Sized> {
        _align: [u8; 0],
        bytes: B,
    }

    static ALIGNED: &'static Aligned<[u8]> = &Aligned {
        _align: [],
        bytes: *include_bytes!("sentence_break_fwd.littleendian.dfa"),
    };

    unsafe {
      ::regex_automata::SparseDFA::from_bytes(&ALIGNED.bytes)
    }
  };
}
