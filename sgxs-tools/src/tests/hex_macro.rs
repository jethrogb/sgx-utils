/*
 * hex! macro
 *
 * Author: Jethro G. Beekman
 *
 * Public domain / CC0
 */

macro_rules! hex {
	(_00)=>(0x00); (_01)=>(0x01); (_02)=>(0x02); (_03)=>(0x03); (_04)=>(0x04);
	(_05)=>(0x05); (_06)=>(0x06); (_07)=>(0x07); (_08)=>(0x08); (_09)=>(0x09);
	(_0a)=>(0x0a); (_0b)=>(0x0b); (_0c)=>(0x0c); (_0d)=>(0x0d); (_0e)=>(0x0e);
	(_0f)=>(0x0f); (_10)=>(0x10); (_11)=>(0x11); (_12)=>(0x12); (_13)=>(0x13);
	(_14)=>(0x14); (_15)=>(0x15); (_16)=>(0x16); (_17)=>(0x17); (_18)=>(0x18);
	(_19)=>(0x19); (_1a)=>(0x1a); (_1b)=>(0x1b); (_1c)=>(0x1c); (_1d)=>(0x1d);
	(_1e)=>(0x1e); (_1f)=>(0x1f); (_20)=>(0x20); (_21)=>(0x21); (_22)=>(0x22);
	(_23)=>(0x23); (_24)=>(0x24); (_25)=>(0x25); (_26)=>(0x26); (_27)=>(0x27);
	(_28)=>(0x28); (_29)=>(0x29); (_2a)=>(0x2a); (_2b)=>(0x2b); (_2c)=>(0x2c);
	(_2d)=>(0x2d); (_2e)=>(0x2e); (_2f)=>(0x2f); (_30)=>(0x30); (_31)=>(0x31);
	(_32)=>(0x32); (_33)=>(0x33); (_34)=>(0x34); (_35)=>(0x35); (_36)=>(0x36);
	(_37)=>(0x37); (_38)=>(0x38); (_39)=>(0x39); (_3a)=>(0x3a); (_3b)=>(0x3b);
	(_3c)=>(0x3c); (_3d)=>(0x3d); (_3e)=>(0x3e); (_3f)=>(0x3f); (_40)=>(0x40);
	(_41)=>(0x41); (_42)=>(0x42); (_43)=>(0x43); (_44)=>(0x44); (_45)=>(0x45);
	(_46)=>(0x46); (_47)=>(0x47); (_48)=>(0x48); (_49)=>(0x49); (_4a)=>(0x4a);
	(_4b)=>(0x4b); (_4c)=>(0x4c); (_4d)=>(0x4d); (_4e)=>(0x4e); (_4f)=>(0x4f);
	(_50)=>(0x50); (_51)=>(0x51); (_52)=>(0x52); (_53)=>(0x53); (_54)=>(0x54);
	(_55)=>(0x55); (_56)=>(0x56); (_57)=>(0x57); (_58)=>(0x58); (_59)=>(0x59);
	(_5a)=>(0x5a); (_5b)=>(0x5b); (_5c)=>(0x5c); (_5d)=>(0x5d); (_5e)=>(0x5e);
	(_5f)=>(0x5f); (_60)=>(0x60); (_61)=>(0x61); (_62)=>(0x62); (_63)=>(0x63);
	(_64)=>(0x64); (_65)=>(0x65); (_66)=>(0x66); (_67)=>(0x67); (_68)=>(0x68);
	(_69)=>(0x69); (_6a)=>(0x6a); (_6b)=>(0x6b); (_6c)=>(0x6c); (_6d)=>(0x6d);
	(_6e)=>(0x6e); (_6f)=>(0x6f); (_70)=>(0x70); (_71)=>(0x71); (_72)=>(0x72);
	(_73)=>(0x73); (_74)=>(0x74); (_75)=>(0x75); (_76)=>(0x76); (_77)=>(0x77);
	(_78)=>(0x78); (_79)=>(0x79); (_7a)=>(0x7a); (_7b)=>(0x7b); (_7c)=>(0x7c);
	(_7d)=>(0x7d); (_7e)=>(0x7e); (_7f)=>(0x7f); (_80)=>(0x80); (_81)=>(0x81);
	(_82)=>(0x82); (_83)=>(0x83); (_84)=>(0x84); (_85)=>(0x85); (_86)=>(0x86);
	(_87)=>(0x87); (_88)=>(0x88); (_89)=>(0x89); (_8a)=>(0x8a); (_8b)=>(0x8b);
	(_8c)=>(0x8c); (_8d)=>(0x8d); (_8e)=>(0x8e); (_8f)=>(0x8f); (_90)=>(0x90);
	(_91)=>(0x91); (_92)=>(0x92); (_93)=>(0x93); (_94)=>(0x94); (_95)=>(0x95);
	(_96)=>(0x96); (_97)=>(0x97); (_98)=>(0x98); (_99)=>(0x99); (_9a)=>(0x9a);
	(_9b)=>(0x9b); (_9c)=>(0x9c); (_9d)=>(0x9d); (_9e)=>(0x9e); (_9f)=>(0x9f);
	(_a0)=>(0xa0); (_a1)=>(0xa1); (_a2)=>(0xa2); (_a3)=>(0xa3); (_a4)=>(0xa4);
	(_a5)=>(0xa5); (_a6)=>(0xa6); (_a7)=>(0xa7); (_a8)=>(0xa8); (_a9)=>(0xa9);
	(_aa)=>(0xaa); (_ab)=>(0xab); (_ac)=>(0xac); (_ad)=>(0xad); (_ae)=>(0xae);
	(_af)=>(0xaf); (_b0)=>(0xb0); (_b1)=>(0xb1); (_b2)=>(0xb2); (_b3)=>(0xb3);
	(_b4)=>(0xb4); (_b5)=>(0xb5); (_b6)=>(0xb6); (_b7)=>(0xb7); (_b8)=>(0xb8);
	(_b9)=>(0xb9); (_ba)=>(0xba); (_bb)=>(0xbb); (_bc)=>(0xbc); (_bd)=>(0xbd);
	(_be)=>(0xbe); (_bf)=>(0xbf); (_c0)=>(0xc0); (_c1)=>(0xc1); (_c2)=>(0xc2);
	(_c3)=>(0xc3); (_c4)=>(0xc4); (_c5)=>(0xc5); (_c6)=>(0xc6); (_c7)=>(0xc7);
	(_c8)=>(0xc8); (_c9)=>(0xc9); (_ca)=>(0xca); (_cb)=>(0xcb); (_cc)=>(0xcc);
	(_cd)=>(0xcd); (_ce)=>(0xce); (_cf)=>(0xcf); (_d0)=>(0xd0); (_d1)=>(0xd1);
	(_d2)=>(0xd2); (_d3)=>(0xd3); (_d4)=>(0xd4); (_d5)=>(0xd5); (_d6)=>(0xd6);
	(_d7)=>(0xd7); (_d8)=>(0xd8); (_d9)=>(0xd9); (_da)=>(0xda); (_db)=>(0xdb);
	(_dc)=>(0xdc); (_dd)=>(0xdd); (_de)=>(0xde); (_df)=>(0xdf); (_e0)=>(0xe0);
	(_e1)=>(0xe1); (_e2)=>(0xe2); (_e3)=>(0xe3); (_e4)=>(0xe4); (_e5)=>(0xe5);
	(_e6)=>(0xe6); (_e7)=>(0xe7); (_e8)=>(0xe8); (_e9)=>(0xe9); (_ea)=>(0xea);
	(_eb)=>(0xeb); (_ec)=>(0xec); (_ed)=>(0xed); (_ee)=>(0xee); (_ef)=>(0xef);
	(_f0)=>(0xf0); (_f1)=>(0xf1); (_f2)=>(0xf2); (_f3)=>(0xf3); (_f4)=>(0xf4);
	(_f5)=>(0xf5); (_f6)=>(0xf6); (_f7)=>(0xf7); (_f8)=>(0xf8); (_f9)=>(0xf9);
	(_fa)=>(0xfa); (_fb)=>(0xfb); (_fc)=>(0xfc); (_fd)=>(0xfd); (_fe)=>(0xfe);
	(_ff)=>(0xff); ( $($x:tt)* ) => ( [ $( hex!($x) ),* ] );
}