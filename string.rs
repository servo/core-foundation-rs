use base::{
    AbstractCFType,
    AbstractCFTypeRef,
    Boolean,
    CFAllocatorRef,
    CFIndex,
    CFOptionFlags,
    CFRelease,
    CFTypeRef,
    CFTypeID,

    kCFAllocatorDefault,
    kCFAllocatorNull,
};

use cast::reinterpret_cast;
use libc::c_char;

pub type UniChar = libc::c_ushort;

/*
 * CFString.h
 */
pub type CFStringCompareFlags = CFOptionFlags;
const kCFCompareCaseInsensitive: CFStringCompareFlags = 1;
const kCFCompareBackwards: CFStringCompareFlags = 4;
const kCFCompareAnchored: CFStringCompareFlags = 8;
const kCFCompareNonliteral: CFStringCompareFlags = 16;
const kCFCompareLocalized: CFStringCompareFlags = 32;
const kCFCompareNumerically: CFStringCompareFlags = 64;
const kCFCompareDiacriticInsensitive: CFStringCompareFlags = 128;
const kCFCompareWidthInsensitive: CFStringCompareFlags = 256;
const kCFCompareForcedOrdering: CFStringCompareFlags = 512;

pub type CFStringEncoding = u32;

/* OSX built-in encodings. */
//const kCFStringEncodingMacRoman: CFStringEncoding = 0;
const kCFStringEncodingWindowsLatin1: CFStringEncoding = 0x0500;
const kCFStringEncodingISOLatin1: CFStringEncoding = 0x0201;
const kCFStringEncodingNextStepLatin: CFStringEncoding = 0x0B01;
const kCFStringEncodingASCII: CFStringEncoding = 0x0600;
const kCFStringEncodingUnicode: CFStringEncoding = 0x0100;
const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;
const kCFStringEncodingNonLossyASCII: CFStringEncoding = 0x0BFF;

const kCFStringEncodingUTF16: CFStringEncoding = 0x0100;
const kCFStringEncodingUTF16BE: CFStringEncoding = 0x10000100;
const kCFStringEncodingUTF16LE: CFStringEncoding = 0x14000100;
const kCFStringEncodingUTF32: CFStringEncoding = 0x0c000100;
const kCFStringEncodingUTF32BE: CFStringEncoding = 0x18000100;
const kCFStringEncodingUTF32LE: CFStringEncoding = 0x1c000100;


/*
 * CFStringEncodingExt.h
 */

type CFStringEncodings = CFIndex;

/* External encodings, except those defined above. */
//const kCFStringEncodingMacRoman: CFStringEncoding = 0;
const kCFStringEncodingMacJapanese: CFStringEncoding = 1;
const kCFStringEncodingMacChineseTrad: CFStringEncoding = 2;
const kCFStringEncodingMacKorean: CFStringEncoding = 3;
const kCFStringEncodingMacArabic: CFStringEncoding = 4;
const kCFStringEncodingMacHebrew: CFStringEncoding = 5;
const kCFStringEncodingMacGreek: CFStringEncoding = 6;
const kCFStringEncodingMacCyrillic: CFStringEncoding = 7;
const kCFStringEncodingMacDevanagari: CFStringEncoding = 9;
const kCFStringEncodingMacGurmukhi: CFStringEncoding = 10;
const kCFStringEncodingMacGujarati: CFStringEncoding = 11;
const kCFStringEncodingMacOriya: CFStringEncoding = 12;
const kCFStringEncodingMacBengali: CFStringEncoding = 13;
const kCFStringEncodingMacTamil: CFStringEncoding = 14;
const kCFStringEncodingMacTelugu: CFStringEncoding = 15;
const kCFStringEncodingMacKannada: CFStringEncoding = 16;
const kCFStringEncodingMacMalayalam: CFStringEncoding = 17;
const kCFStringEncodingMacSinhalese: CFStringEncoding = 18;
const kCFStringEncodingMacBurmese: CFStringEncoding = 19;
const kCFStringEncodingMacKhmer: CFStringEncoding = 20;
const kCFStringEncodingMacThai: CFStringEncoding = 21;
const kCFStringEncodingMacLaotian: CFStringEncoding = 22;
const kCFStringEncodingMacGeorgian: CFStringEncoding = 23;
const kCFStringEncodingMacArmenian: CFStringEncoding = 24;
const kCFStringEncodingMacChineseSimp: CFStringEncoding = 25;
const kCFStringEncodingMacTibetan: CFStringEncoding = 26;
const kCFStringEncodingMacMongolian: CFStringEncoding = 27;
const kCFStringEncodingMacEthiopic: CFStringEncoding = 28;
const kCFStringEncodingMacCentralEurRoman: CFStringEncoding = 29;
const kCFStringEncodingMacVietnamese: CFStringEncoding = 30;
const kCFStringEncodingMacExtArabic: CFStringEncoding = 31;
const kCFStringEncodingMacSymbol: CFStringEncoding = 33;
const kCFStringEncodingMacDingbats: CFStringEncoding = 34;
const kCFStringEncodingMacTurkish: CFStringEncoding = 35;
const kCFStringEncodingMacCroatian: CFStringEncoding = 36;
const kCFStringEncodingMacIcelandic: CFStringEncoding = 37;
const kCFStringEncodingMacRomanian: CFStringEncoding = 38;
const kCFStringEncodingMacCeltic: CFStringEncoding = 39;
const kCFStringEncodingMacGaelic: CFStringEncoding = 40;
const kCFStringEncodingMacFarsi: CFStringEncoding = 0x8C;
const kCFStringEncodingMacUkrainian: CFStringEncoding = 0x98;
const kCFStringEncodingMacInuit: CFStringEncoding = 0xEC;
const kCFStringEncodingMacVT100: CFStringEncoding = 0xFC;
const kCFStringEncodingMacHFS: CFStringEncoding = 0xFF;
//const kCFStringEncodingISOLatin1: CFStringEncoding = 0x0201;
const kCFStringEncodingISOLatin2: CFStringEncoding = 0x0202;
const kCFStringEncodingISOLatin3: CFStringEncoding = 0x0203;
const kCFStringEncodingISOLatin4: CFStringEncoding = 0x0204;
const kCFStringEncodingISOLatinCyrillic: CFStringEncoding = 0x0205;
const kCFStringEncodingISOLatinArabic: CFStringEncoding = 0x0206;
const kCFStringEncodingISOLatinGreek: CFStringEncoding = 0x0207;
const kCFStringEncodingISOLatinHebrew: CFStringEncoding = 0x0208;
const kCFStringEncodingISOLatin5: CFStringEncoding = 0x0209;
const kCFStringEncodingISOLatin6: CFStringEncoding = 0x020A;
const kCFStringEncodingISOLatinThai: CFStringEncoding = 0x020B;
const kCFStringEncodingISOLatin7: CFStringEncoding = 0x020D;
const kCFStringEncodingISOLatin8: CFStringEncoding = 0x020E;
const kCFStringEncodingISOLatin9: CFStringEncoding = 0x020F;
const kCFStringEncodingISOLatin10: CFStringEncoding = 0x0210;
const kCFStringEncodingDOSLatinUS: CFStringEncoding = 0x0400;
const kCFStringEncodingDOSGreek: CFStringEncoding = 0x0405;
const kCFStringEncodingDOSBalticRim: CFStringEncoding = 0x0406;
const kCFStringEncodingDOSLatin1: CFStringEncoding = 0x0410;
const kCFStringEncodingDOSGreek1: CFStringEncoding = 0x0411;
const kCFStringEncodingDOSLatin2: CFStringEncoding = 0x0412;
const kCFStringEncodingDOSCyrillic: CFStringEncoding = 0x0413;
const kCFStringEncodingDOSTurkish: CFStringEncoding = 0x0414;
const kCFStringEncodingDOSPortuguese: CFStringEncoding = 0x0415;
const kCFStringEncodingDOSIcelandic: CFStringEncoding = 0x0416;
const kCFStringEncodingDOSHebrew: CFStringEncoding = 0x0417;
const kCFStringEncodingDOSCanadianFrench: CFStringEncoding = 0x0418;
const kCFStringEncodingDOSArabic: CFStringEncoding = 0x0419;
const kCFStringEncodingDOSNordic: CFStringEncoding = 0x041A;
const kCFStringEncodingDOSRussian: CFStringEncoding = 0x041B;
const kCFStringEncodingDOSGreek2: CFStringEncoding = 0x041C;
const kCFStringEncodingDOSThai: CFStringEncoding = 0x041D;
const kCFStringEncodingDOSJapanese: CFStringEncoding = 0x0420;
const kCFStringEncodingDOSChineseSimplif: CFStringEncoding = 0x0421;
const kCFStringEncodingDOSKorean: CFStringEncoding = 0x0422;
const kCFStringEncodingDOSChineseTrad: CFStringEncoding = 0x0423;
//const kCFStringEncodingWindowsLatin1: CFStringEncoding = 0x0500;
const kCFStringEncodingWindowsLatin2: CFStringEncoding = 0x0501;
const kCFStringEncodingWindowsCyrillic: CFStringEncoding = 0x0502;
const kCFStringEncodingWindowsGreek: CFStringEncoding = 0x0503;
const kCFStringEncodingWindowsLatin5: CFStringEncoding = 0x0504;
const kCFStringEncodingWindowsHebrew: CFStringEncoding = 0x0505;
const kCFStringEncodingWindowsArabic: CFStringEncoding = 0x0506;
const kCFStringEncodingWindowsBalticRim: CFStringEncoding = 0x0507;
const kCFStringEncodingWindowsVietnamese: CFStringEncoding = 0x0508;
const kCFStringEncodingWindowsKoreanJohab: CFStringEncoding = 0x0510;
//const kCFStringEncodingASCII: CFStringEncoding = 0x0600;
const kCFStringEncodingANSEL: CFStringEncoding = 0x0601;
const kCFStringEncodingJIS_X0201_76: CFStringEncoding = 0x0620;
const kCFStringEncodingJIS_X0208_83: CFStringEncoding = 0x0621;
const kCFStringEncodingJIS_X0208_90: CFStringEncoding = 0x0622;
const kCFStringEncodingJIS_X0212_90: CFStringEncoding = 0x0623;
const kCFStringEncodingJIS_C6226_78: CFStringEncoding = 0x0624;
const kCFStringEncodingShiftJIS_X0213: CFStringEncoding = 0x0628;
const kCFStringEncodingShiftJIS_X0213_MenKuTen: CFStringEncoding = 0x0629;
const kCFStringEncodingGB_2312_80: CFStringEncoding = 0x0630;
const kCFStringEncodingGBK_95: CFStringEncoding = 0x0631;
const kCFStringEncodingGB_18030_2000: CFStringEncoding = 0x0632;
const kCFStringEncodingKSC_5601_87: CFStringEncoding = 0x0640;
const kCFStringEncodingKSC_5601_92_Johab: CFStringEncoding = 0x0641;
const kCFStringEncodingCNS_11643_92_P1: CFStringEncoding = 0x0651;
const kCFStringEncodingCNS_11643_92_P2: CFStringEncoding = 0x0652;
const kCFStringEncodingCNS_11643_92_P3: CFStringEncoding = 0x0653;
const kCFStringEncodingISO_2022_JP: CFStringEncoding = 0x0820;
const kCFStringEncodingISO_2022_JP_2: CFStringEncoding = 0x0821;
const kCFStringEncodingISO_2022_JP_1: CFStringEncoding = 0x0822;
const kCFStringEncodingISO_2022_JP_3: CFStringEncoding = 0x0823;
const kCFStringEncodingISO_2022_CN: CFStringEncoding = 0x0830;
const kCFStringEncodingISO_2022_CN_EXT: CFStringEncoding = 0x0831;
const kCFStringEncodingISO_2022_KR: CFStringEncoding = 0x0840;
const kCFStringEncodingEUC_JP: CFStringEncoding = 0x0920;
const kCFStringEncodingEUC_CN: CFStringEncoding = 0x0930;
const kCFStringEncodingEUC_TW: CFStringEncoding = 0x0931;
const kCFStringEncodingEUC_KR: CFStringEncoding = 0x0940;
const kCFStringEncodingShiftJIS: CFStringEncoding = 0x0A01;
const kCFStringEncodingKOI8_R: CFStringEncoding = 0x0A02;
const kCFStringEncodingBig5: CFStringEncoding = 0x0A03;
const kCFStringEncodingMacRomanLatin1: CFStringEncoding = 0x0A04;
const kCFStringEncodingHZ_GB_2312: CFStringEncoding = 0x0A05;
const kCFStringEncodingBig5_HKSCS_1999: CFStringEncoding = 0x0A06;
const kCFStringEncodingVISCII: CFStringEncoding = 0x0A07;
const kCFStringEncodingKOI8_U: CFStringEncoding = 0x0A08;
const kCFStringEncodingBig5_E: CFStringEncoding = 0x0A09;
//const kCFStringEncodingNextStepLatin: CFStringEncoding = 0x0B01;
const kCFStringEncodingNextStepJapanese: CFStringEncoding = 0x0B02;
const kCFStringEncodingEBCDIC_US: CFStringEncoding = 0x0C01;
const kCFStringEncodingEBCDIC_CP037: CFStringEncoding = 0x0C02;
const kCFStringEncodingUTF7: CFStringEncoding = 0x04000100;
const kCFStringEncodingUTF7_IMAP: CFStringEncoding = 0x0A10;
const kCFStringEncodingShiftJIS_X0213_00: CFStringEncoding = 0x0628; /* Deprecated */

struct __CFString { private: () }
pub type CFStringRef = *__CFString;

impl CFStringRef : AbstractCFTypeRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }
}

pub struct CFString {
    obj: CFStringRef,

    drop {
        unsafe {
            CFRelease(cast::transmute(self.obj));
        }
    }
}

pub impl CFString : AbstractCFType<CFStringRef> {
    pure fn get_ref() -> CFStringRef { self.obj }

    static fn wrap(obj: CFStringRef) -> CFString {
        CFString { obj: obj }
    }

    static fn unwrap(wrapper: CFString) -> CFStringRef {
        wrapper.obj
    }
}

pub impl CFString {
    // convenience method to make it easier to wrap extern
    // CFStringRefs without providing explicit typarams to base::wrap()
    static fn wrap_extern(string: CFStringRef) -> CFString {
        base::wrap(string)
    }

    static fn new_static(string: &static/str) -> CFString {
        let string_ref = do str::as_buf(string) |bytes, len| {
            CFStringCreateWithBytesNoCopy(kCFAllocatorDefault,
                                          bytes,
                                          len as CFIndex,
                                          kCFStringEncodingUTF8,
                                          false as Boolean,
                                          kCFAllocatorNull)
        };
        base::wrap(string_ref)
    }
}

#[link_args="-framework CoreFoundation"]
#[nolink]
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
    //fn CFStringCreateWithBytes
    fn CFStringCreateWithBytesNoCopy(alloc: CFAllocatorRef,
                                     bytes: *u8,
                                     numBytes: CFIndex,
                                     encoding: CFStringEncoding,
                                     isExternalRepresentation: Boolean,
                                     contentsDeallocator: CFAllocatorRef)
                                  -> CFStringRef;
    //fn CFStringCreateWithCharacters
    //fn CFStringCreateWithCharactersNoCopy
    //fn CFStringCreateWithCString
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
    //fn CFStringGetBytes
    //fn CFStringGetCharacterAtIndex
    //fn CFStringGetCharacters
    //fn CFStringGetCharactersPtr
    //fn CFStringGetCharacterFromInlineBuffer
    //fn CFStringGetCString
    //fn CFStringGetCStringPtr
    //fn CFStringGetLength
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
    fn CFStringGetTypeID() -> CFTypeID;

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

