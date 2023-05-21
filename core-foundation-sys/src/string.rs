// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::{c_char, c_void};

use base::{Boolean, CFOptionFlags, CFIndex, CFAllocatorRef, CFRange, CFTypeID, UInt32, UniChar};

pub type CFStringCompareFlags = CFOptionFlags;
pub const kCFCompareCaseInsensitive: CFStringCompareFlags = 1;
pub const kCFCompareBackwards: CFStringCompareFlags = 4;
pub const kCFCompareAnchored: CFStringCompareFlags = 8;
pub const kCFCompareNonliteral: CFStringCompareFlags = 16;
pub const kCFCompareLocalized: CFStringCompareFlags = 32;
pub const kCFCompareNumerically: CFStringCompareFlags = 64;
pub const kCFCompareDiacriticInsensitive: CFStringCompareFlags = 128;
pub const kCFCompareWidthInsensitive: CFStringCompareFlags = 256;
pub const kCFCompareForcedOrdering: CFStringCompareFlags = 512;

pub type CFStringEncoding = UInt32;

// macOS built-in encodings.

pub const kCFStringEncodingMacRoman: CFStringEncoding = 0;
pub const kCFStringEncodingWindowsLatin1: CFStringEncoding = 0x0500;
pub const kCFStringEncodingISOLatin1: CFStringEncoding = 0x0201;
pub const kCFStringEncodingNextStepLatin: CFStringEncoding = 0x0B01;
pub const kCFStringEncodingASCII: CFStringEncoding = 0x0600;
pub const kCFStringEncodingUnicode: CFStringEncoding = 0x0100;
pub const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;
pub const kCFStringEncodingNonLossyASCII: CFStringEncoding = 0x0BFF;

pub const kCFStringEncodingUTF16: CFStringEncoding = 0x0100;
pub const kCFStringEncodingUTF16BE: CFStringEncoding = 0x10000100;
pub const kCFStringEncodingUTF16LE: CFStringEncoding = 0x14000100;
pub const kCFStringEncodingUTF32: CFStringEncoding = 0x0c000100;
pub const kCFStringEncodingUTF32BE: CFStringEncoding = 0x18000100;
pub const kCFStringEncodingUTF32LE: CFStringEncoding = 0x1c000100;

// CFStringEncodingExt.h

// External encodings, except those defined above.
// Defined above: kCFStringEncodingMacRoman = 0
pub const kCFStringEncodingMacJapanese: CFStringEncoding = 1;
pub const kCFStringEncodingMacChineseTrad: CFStringEncoding = 2;
pub const kCFStringEncodingMacKorean: CFStringEncoding = 3;
pub const kCFStringEncodingMacArabic: CFStringEncoding = 4;
pub const kCFStringEncodingMacHebrew: CFStringEncoding = 5;
pub const kCFStringEncodingMacGreek: CFStringEncoding = 6;
pub const kCFStringEncodingMacCyrillic: CFStringEncoding = 7;
pub const kCFStringEncodingMacDevanagari: CFStringEncoding = 9;
pub const kCFStringEncodingMacGurmukhi: CFStringEncoding = 10;
pub const kCFStringEncodingMacGujarati: CFStringEncoding = 11;
pub const kCFStringEncodingMacOriya: CFStringEncoding = 12;
pub const kCFStringEncodingMacBengali: CFStringEncoding = 13;
pub const kCFStringEncodingMacTamil: CFStringEncoding = 14;
pub const kCFStringEncodingMacTelugu: CFStringEncoding = 15;
pub const kCFStringEncodingMacKannada: CFStringEncoding = 16;
pub const kCFStringEncodingMacMalayalam: CFStringEncoding = 17;
pub const kCFStringEncodingMacSinhalese: CFStringEncoding = 18;
pub const kCFStringEncodingMacBurmese: CFStringEncoding = 19;
pub const kCFStringEncodingMacKhmer: CFStringEncoding = 20;
pub const kCFStringEncodingMacThai: CFStringEncoding = 21;
pub const kCFStringEncodingMacLaotian: CFStringEncoding = 22;
pub const kCFStringEncodingMacGeorgian: CFStringEncoding = 23;
pub const kCFStringEncodingMacArmenian: CFStringEncoding = 24;
pub const kCFStringEncodingMacChineseSimp: CFStringEncoding = 25;
pub const kCFStringEncodingMacTibetan: CFStringEncoding = 26;
pub const kCFStringEncodingMacMongolian: CFStringEncoding = 27;
pub const kCFStringEncodingMacEthiopic: CFStringEncoding = 28;
pub const kCFStringEncodingMacCentralEurRoman: CFStringEncoding = 29;
pub const kCFStringEncodingMacVietnamese: CFStringEncoding = 30;
pub const kCFStringEncodingMacExtArabic: CFStringEncoding = 31;
pub const kCFStringEncodingMacSymbol: CFStringEncoding = 33;
pub const kCFStringEncodingMacDingbats: CFStringEncoding = 34;
pub const kCFStringEncodingMacTurkish: CFStringEncoding = 35;
pub const kCFStringEncodingMacCroatian: CFStringEncoding = 36;
pub const kCFStringEncodingMacIcelandic: CFStringEncoding = 37;
pub const kCFStringEncodingMacRomanian: CFStringEncoding = 38;
pub const kCFStringEncodingMacCeltic: CFStringEncoding = 39;
pub const kCFStringEncodingMacGaelic: CFStringEncoding = 40;
pub const kCFStringEncodingMacFarsi: CFStringEncoding = 0x8C;
pub const kCFStringEncodingMacUkrainian: CFStringEncoding = 0x98;
pub const kCFStringEncodingMacInuit: CFStringEncoding = 0xEC;
pub const kCFStringEncodingMacVT100: CFStringEncoding = 0xFC;
pub const kCFStringEncodingMacHFS: CFStringEncoding = 0xFF;
// Defined above: kCFStringEncodingISOLatin1 = 0x0201
pub const kCFStringEncodingISOLatin2: CFStringEncoding = 0x0202;
pub const kCFStringEncodingISOLatin3: CFStringEncoding = 0x0203;
pub const kCFStringEncodingISOLatin4: CFStringEncoding = 0x0204;
pub const kCFStringEncodingISOLatinCyrillic: CFStringEncoding = 0x0205;
pub const kCFStringEncodingISOLatinArabic: CFStringEncoding = 0x0206;
pub const kCFStringEncodingISOLatinGreek: CFStringEncoding = 0x0207;
pub const kCFStringEncodingISOLatinHebrew: CFStringEncoding = 0x0208;
pub const kCFStringEncodingISOLatin5: CFStringEncoding = 0x0209;
pub const kCFStringEncodingISOLatin6: CFStringEncoding = 0x020A;
pub const kCFStringEncodingISOLatinThai: CFStringEncoding = 0x020B;
pub const kCFStringEncodingISOLatin7: CFStringEncoding = 0x020D;
pub const kCFStringEncodingISOLatin8: CFStringEncoding = 0x020E;
pub const kCFStringEncodingISOLatin9: CFStringEncoding = 0x020F;
pub const kCFStringEncodingISOLatin10: CFStringEncoding = 0x0210;
pub const kCFStringEncodingDOSLatinUS: CFStringEncoding = 0x0400;
pub const kCFStringEncodingDOSGreek: CFStringEncoding = 0x0405;
pub const kCFStringEncodingDOSBalticRim: CFStringEncoding = 0x0406;
pub const kCFStringEncodingDOSLatin1: CFStringEncoding = 0x0410;
pub const kCFStringEncodingDOSGreek1: CFStringEncoding = 0x0411;
pub const kCFStringEncodingDOSLatin2: CFStringEncoding = 0x0412;
pub const kCFStringEncodingDOSCyrillic: CFStringEncoding = 0x0413;
pub const kCFStringEncodingDOSTurkish: CFStringEncoding = 0x0414;
pub const kCFStringEncodingDOSPortuguese: CFStringEncoding = 0x0415;
pub const kCFStringEncodingDOSIcelandic: CFStringEncoding = 0x0416;
pub const kCFStringEncodingDOSHebrew: CFStringEncoding = 0x0417;
pub const kCFStringEncodingDOSCanadianFrench: CFStringEncoding = 0x0418;
pub const kCFStringEncodingDOSArabic: CFStringEncoding = 0x0419;
pub const kCFStringEncodingDOSNordic: CFStringEncoding = 0x041A;
pub const kCFStringEncodingDOSRussian: CFStringEncoding = 0x041B;
pub const kCFStringEncodingDOSGreek2: CFStringEncoding = 0x041C;
pub const kCFStringEncodingDOSThai: CFStringEncoding = 0x041D;
pub const kCFStringEncodingDOSJapanese: CFStringEncoding = 0x0420;
pub const kCFStringEncodingDOSChineseSimplif: CFStringEncoding = 0x0421;
pub const kCFStringEncodingDOSKorean: CFStringEncoding = 0x0422;
pub const kCFStringEncodingDOSChineseTrad: CFStringEncoding = 0x0423;
// Defined above: kCFStringEncodingWindowsLatin1 = 0x0500
pub const kCFStringEncodingWindowsLatin2: CFStringEncoding = 0x0501;
pub const kCFStringEncodingWindowsCyrillic: CFStringEncoding = 0x0502;
pub const kCFStringEncodingWindowsGreek: CFStringEncoding = 0x0503;
pub const kCFStringEncodingWindowsLatin5: CFStringEncoding = 0x0504;
pub const kCFStringEncodingWindowsHebrew: CFStringEncoding = 0x0505;
pub const kCFStringEncodingWindowsArabic: CFStringEncoding = 0x0506;
pub const kCFStringEncodingWindowsBalticRim: CFStringEncoding = 0x0507;
pub const kCFStringEncodingWindowsVietnamese: CFStringEncoding = 0x0508;
pub const kCFStringEncodingWindowsKoreanJohab: CFStringEncoding = 0x0510;
// Defined above: kCFStringEncodingASCII = 0x0600
pub const kCFStringEncodingANSEL: CFStringEncoding = 0x0601;
pub const kCFStringEncodingJIS_X0201_76: CFStringEncoding = 0x0620;
pub const kCFStringEncodingJIS_X0208_83: CFStringEncoding = 0x0621;
pub const kCFStringEncodingJIS_X0208_90: CFStringEncoding = 0x0622;
pub const kCFStringEncodingJIS_X0212_90: CFStringEncoding = 0x0623;
pub const kCFStringEncodingJIS_C6226_78: CFStringEncoding = 0x0624;
pub const kCFStringEncodingShiftJIS_X0213: CFStringEncoding = 0x0628;
pub const kCFStringEncodingShiftJIS_X0213_MenKuTen: CFStringEncoding = 0x0629;
pub const kCFStringEncodingGB_2312_80: CFStringEncoding = 0x0630;
pub const kCFStringEncodingGBK_95: CFStringEncoding = 0x0631;
pub const kCFStringEncodingGB_18030_2000: CFStringEncoding = 0x0632;
pub const kCFStringEncodingKSC_5601_87: CFStringEncoding = 0x0640;
pub const kCFStringEncodingKSC_5601_92_Johab: CFStringEncoding = 0x0641;
pub const kCFStringEncodingCNS_11643_92_P1: CFStringEncoding = 0x0651;
pub const kCFStringEncodingCNS_11643_92_P2: CFStringEncoding = 0x0652;
pub const kCFStringEncodingCNS_11643_92_P3: CFStringEncoding = 0x0653;
pub const kCFStringEncodingISO_2022_JP: CFStringEncoding = 0x0820;
pub const kCFStringEncodingISO_2022_JP_2: CFStringEncoding = 0x0821;
pub const kCFStringEncodingISO_2022_JP_1: CFStringEncoding = 0x0822;
pub const kCFStringEncodingISO_2022_JP_3: CFStringEncoding = 0x0823;
pub const kCFStringEncodingISO_2022_CN: CFStringEncoding = 0x0830;
pub const kCFStringEncodingISO_2022_CN_EXT: CFStringEncoding = 0x0831;
pub const kCFStringEncodingISO_2022_KR: CFStringEncoding = 0x0840;
pub const kCFStringEncodingEUC_JP: CFStringEncoding = 0x0920;
pub const kCFStringEncodingEUC_CN: CFStringEncoding = 0x0930;
pub const kCFStringEncodingEUC_TW: CFStringEncoding = 0x0931;
pub const kCFStringEncodingEUC_KR: CFStringEncoding = 0x0940;
pub const kCFStringEncodingShiftJIS: CFStringEncoding = 0x0A01;
pub const kCFStringEncodingKOI8_R: CFStringEncoding = 0x0A02;
pub const kCFStringEncodingBig5: CFStringEncoding = 0x0A03;
pub const kCFStringEncodingMacRomanLatin1: CFStringEncoding = 0x0A04;
pub const kCFStringEncodingHZ_GB_2312: CFStringEncoding = 0x0A05;
pub const kCFStringEncodingBig5_HKSCS_1999: CFStringEncoding = 0x0A06;
pub const kCFStringEncodingVISCII: CFStringEncoding = 0x0A07;
pub const kCFStringEncodingKOI8_U: CFStringEncoding = 0x0A08;
pub const kCFStringEncodingBig5_E: CFStringEncoding = 0x0A09;
// Defined above: kCFStringEncodingNextStepLatin = 0x0B01
pub const kCFStringEncodingNextStepJapanese: CFStringEncoding = 0x0B02;
pub const kCFStringEncodingEBCDIC_US: CFStringEncoding = 0x0C01;
pub const kCFStringEncodingEBCDIC_CP037: CFStringEncoding = 0x0C02;
pub const kCFStringEncodingUTF7: CFStringEncoding = 0x04000100;
pub const kCFStringEncodingUTF7_IMAP: CFStringEncoding = 0x0A10;
pub const kCFStringEncodingShiftJIS_X0213_00: CFStringEncoding = 0x0628; /* Deprecated */

pub const kCFStringEncodingInvalidId: u32 = 0xffffffff;

#[repr(C)]
pub struct __CFString(c_void);

pub type CFStringRef = *const __CFString;

extern {
    /*
     * CFString.h
     */

    // N.B. organized according to "Functions by task" in docs

    /* Creating a CFString */
    //fn CFSTR
    //fn CFStringCreateArrayBySeparatingStrings
    //fn CFStringCreateByCombiningStrings
    //fn CFStringCreateCopy
    //fn CFStringCreateFromExternalRepresentation
    pub fn CFStringCreateWithBytes(alloc: CFAllocatorRef,
                                   bytes: *const u8,
                                   numBytes: CFIndex,
                                   encoding: CFStringEncoding,
                                   isExternalRepresentation: Boolean)
                                   -> CFStringRef;
    pub fn CFStringCreateWithBytesNoCopy(alloc: CFAllocatorRef,
                                         bytes: *const u8,
                                         numBytes: CFIndex,
                                         encoding: CFStringEncoding,
                                         isExternalRepresentation: Boolean,
                                         contentsDeallocator: CFAllocatorRef)
                                         -> CFStringRef;
    //fn CFStringCreateWithCharacters
    pub fn CFStringCreateWithCharactersNoCopy(alloc: CFAllocatorRef,
                                              chars: *const UniChar,
                                              numChars: CFIndex,
                                              contentsDeallocator: CFAllocatorRef)
                                              -> CFStringRef;
    pub fn CFStringCreateWithCString(alloc: CFAllocatorRef,
                                     cStr: *const c_char,
                                     encoding: CFStringEncoding)
                                     -> CFStringRef;
    //fn CFStringCreateWithCStringNoCopy
    //fn CFStringCreateWithFormat
    //fn CFStringCreateWithFormatAndArguments
    //fn CFStringCreateWithPascalString
    //fn CFStringCreateWithPascalStringNoCopy
    //fn CFStringCreateWithSubstring

    /* Searching Strings */
    //fn CFStringCreateArrayWithFindResults
    //fn CFStringFind
    //fn CFStringFindCharacterFromSet
    //fn CFStringFindWithOptions
    //fn CFStringFindWithOptionsAndLocale
    //fn CFStringGetLineBounds

    /* Comparing Strings */
    //fn CFStringCompare
    //fn CFStringCompareWithOptions
    //fn CFStringCompareWithOptionsAndLocale
    //fn CFStringHasPrefix
    //fn CFStringHasSuffix

    /* Accessing Characters */
    //fn CFStringCreateExternalRepresentation
    pub fn CFStringGetBytes(theString: CFStringRef,
                            range: CFRange,
                            encoding: CFStringEncoding,
                            lossByte: u8,
                            isExternalRepresentation: Boolean,
                            buffer: *mut u8,
                            maxBufLen: CFIndex,
                            usedBufLen: *mut CFIndex)
                            -> CFIndex;
    //fn CFStringGetCharacterAtIndex
    //fn CFStringGetCharacters
    //fn CFStringGetCharactersPtr
    //fn CFStringGetCharacterFromInlineBuffer
    pub fn CFStringGetCString(theString: CFStringRef,
                              buffer: *mut c_char,
                              bufferSize: CFIndex,
                              encoding: CFStringEncoding)
                              -> Boolean;
    pub fn CFStringGetCStringPtr(theString: CFStringRef,
                                 encoding: CFStringEncoding)
                                 -> *const c_char;
    pub fn CFStringGetLength(theString: CFStringRef) -> CFIndex;
    //fn CFStringGetPascalString
    //fn CFStringGetPascalStringPtr
    //fn CFStringGetRangeOfComposedCharactersAtIndex
    //fn CFStringInitInlineBuffer

    /* Working With Hyphenation */
    //fn CFStringGetHyphenationLocationBeforeIndex
    //fn CFStringIsHyphenationAvailableForLocale

    /* Working With Encodings */
    //fn CFStringConvertEncodingToIANACharSetName
    //fn CFStringConvertEncodingToNSStringEncoding
    //fn CFStringConvertEncodingToWindowsCodepage
    //fn CFStringConvertIANACharSetNameToEncoding
    //fn CFStringConvertNSStringEncodingToEncoding
    //fn CFStringConvertWindowsCodepageToEncoding
    //fn CFStringGetFastestEncoding
    //fn CFStringGetListOfAvailableEncodings
    //fn CFStringGetMaximumSizeForEncoding
    //fn CFStringGetMostCompatibleMacStringEncoding
    //fn CFStringGetNameOfEncoding
    //fn CFStringGetSmallestEncoding
    //fn CFStringGetSystemEncoding
    //fn CFStringIsEncodingAvailable

    /* Getting Numeric Values */
    //fn CFStringGetDoubleValue
    //fn CFStringGetIntValue

    /* Getting String Properties */
    //fn CFShowStr
    pub fn CFStringGetTypeID() -> CFTypeID;

    /* String File System Representations */
    //fn CFStringCreateWithFileSystemRepresentation
    //fn CFStringGetFileSystemRepresentation
    //fn CFStringGetMaximumSizeOfFileSystemRepresentation

    /* Getting Paragraph Bounds */
    //fn CFStringGetParagraphBounds

    /* Managing Surrogates */
    //fn CFStringGetLongCharacterForSurrogatePair
    //fn CFStringGetSurrogatePairForLongCharacter
    //fn CFStringIsSurrogateHighCharacter
    //fn CFStringIsSurrogateLowCharacter
}
