use base::{
    AbstractCFTypeRef,
    CFAllocatorRef,
    CFOptionFlags,
    CFTypeID,
    CFTypeRef,
    CFWrapper,
};
use data::{
    CFDataRef,
};
use string::{
    CFString,
    CFStringRef,
    CFStringEncoding,
};

struct __CFURL { private: () }
pub type CFURLRef = *__CFURL;

impl AbstractCFTypeRef for CFURLRef {
    pure fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    static pure fn type_id() -> CFTypeID {
        unsafe {
            CFURLGetTypeID()
        }
    }
}

pub type CFURL = CFWrapper<CFURLRef, (), ()>;

pub impl ToStr for CFURL {
    pure fn to_str(&self) -> ~str {
        unsafe {
            let cfstr: CFString = CFWrapper::wrap_shared(CFURLGetString(self.obj));
            cfstr.to_str()
        }
    }
}

type CFURLBookmarkCreationOptions = CFOptionFlags;
const kCFURLBookmarkCreationPreferFileIDResolutionMask: CFURLBookmarkCreationOptions =
    (1 << 8) as u32;
const kCFURLBookmarkCreationMinimalBookmarkMask: CFURLBookmarkCreationOptions =
    (1 << 9) as u32;
const kCFURLBookmarkCreationSuitableForBookmarkFile: CFURLBookmarkCreationOptions =
    (1 << 10) as u32;
const kCFURLBookmarkCreationWithSecurityScope: CFURLBookmarkCreationOptions =
    (1 << 11) as u32;
const kCFURLBookmarkCreationSecurityScopeAllowOnlyReadAccess: CFURLBookmarkCreationOptions =
    (1 << 12) as u32;

// TODO: there are a lot of missing keys and constants. Add if you are bored or need them.

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFURL.h
     */

    /* Common File System Resource Keys */
    const kCFURLAttributeModificationDateKey: CFStringRef;
    const kCFURLContentAccessDateKey: CFStringRef;
    const kCFURLContentModificationDateKey: CFStringRef;
    const kCFURLCreationDateKey: CFStringRef;
    const kCFURLCustomIconKey: CFStringRef;
    const kCFURLEffectiveIconKey: CFStringRef;
    const kCFURLFileResourceIdentifierKey: CFStringRef;
    const kCFURLFileSecurityKey: CFStringRef;
    const kCFURLHasHiddenExtensionKey: CFStringRef;
    const kCFURLIsDirectoryKey: CFStringRef;
    const kCFURLIsExecutableKey: CFStringRef;
    const kCFURLIsHiddenKey: CFStringRef;
    const kCFURLIsPackageKey: CFStringRef;
    const kCFURLIsReadableKey: CFStringRef;
    const kCFURLIsRegularFileKey: CFStringRef;
    const kCFURLIsSymbolicLinkKey: CFStringRef;
    const kCFURLIsSystemImmutableKey: CFStringRef;
    const kCFURLIsUserImmutableKey: CFStringRef;
    const kCFURLIsVolumeKey: CFStringRef;
    const kCFURLIsWritableKey: CFStringRef;
    const kCFURLLabelColorKey: CFStringRef;
    const kCFURLLabelNumberKey: CFStringRef;
    const kCFURLLinkCountKey: CFStringRef;
    const kCFURLLocalizedLabelKey: CFStringRef;
    const kCFURLLocalizedNameKey: CFStringRef;
    const kCFURLLocalizedTypeDescriptionKey: CFStringRef;
    const kCFURLNameKey: CFStringRef;
    const kCFURLParentDirectoryURLKey: CFStringRef;
    const kCFURLPreferredIOBlockSizeKey: CFStringRef;
    const kCFURLTypeIdentifierKey: CFStringRef;
    const kCFURLVolumeIdentifierKey: CFStringRef;
    const kCFURLVolumeURLKey: CFStringRef;
    const kCFURLIsExcludedFromBackupKey: CFStringRef;
    const kCFURLFileResourceTypeKey: CFStringRef;

    /* Creating a CFURL */
    //fn CFURLCopyAbsoluteURL
    //fn CFURLCreateAbsoluteURLWithBytes
    //fn CFURLCreateByResolvingBookmarkData
    //fn CFURLCreateCopyAppendingPathComponent
    //fn CFURLCreateCopyAppendingPathExtension
    //fn CFURLCreateCopyDeletingLastPathComponent
    //fn CFURLCreateCopyDeletingPathExtension
    //fn CFURLCreateFilePathURL
    //fn CFURLCreateFileReferenceURL
    //fn CFURLCreateFromFileSystemRepresentation
    //fn CFURLCreateFromFileSystemRepresentationRelativeToBase
    //fn CFURLCreateFromFSRef
    //fn CFURLCreateWithBytes
    //fn CFURLCreateWithFileSystemPath
    //fn CFURLCreateWithFileSystemPathRelativeToBase
    fn CFURLCreateWithString(allocator: CFAllocatorRef, urlString: CFStringRef,
                             baseURL: CFURLRef) -> CFURLRef;

    /* Accessing the Parts of a URL */
    //fn CFURLCanBeDecomposed
    //fn CFURLCopyFileSystemPath
    //fn CFURLCopyFragment
    //fn CFURLCopyHostName
    //fn CFURLCopyLastPathComponent
    //fn CFURLCopyNetLocation
    //fn CFURLCopyParameterString
    //fn CFURLCopyPassword
    //fn CFURLCopyPath
    //fn CFURLCopyPathExtension
    //fn CFURLCopyQueryString
    //fn CFURLCopyResourceSpecifier
    //fn CFURLCopyScheme
    //fn CFURLCopyStrictPath
    //fn CFURLCopyUserName
    //fn CFURLGetPortNumber
    //fn CFURLHasDirectoryPath

    /* Converting URLs to Other Representations */
    fn CFURLCreateData(allocator: CFAllocatorRef, url: CFURLRef, 
                       encoding: CFStringEncoding, escapeWhitespace: bool) -> CFDataRef;
    //fn CFURLCreateStringByAddingPercentEscapes
    //fn CFURLCreateStringByReplacingPercentEscapes
    //fn CFURLCreateStringByReplacingPercentEscapesUsingEncoding
    //fn CFURLGetFileSystemRepresentation
    //fn CFURLGetFSRef
    fn CFURLGetString(anURL: CFURLRef) -> CFStringRef;

    /* Getting URL Properties */
    fn CFURLGetBaseURL(anURL: CFURLRef) -> CFURLRef;
    //fn CFURLGetBytes
    //fn CFURLGetByteRangeForComponent
    fn CFURLGetTypeID() -> CFTypeID;
    //fn CFURLResourceIsReachable

    /* Getting and Setting File System Resource Properties */
    //fn CFURLClearResourcePropertyCache
    //fn CFURLClearResourcePropertyCacheForKey
    //fn CFURLCopyResourcePropertiesForKeys
    //fn CFURLCopyResourcePropertyForKey
    //fn CFURLCreateResourcePropertiesForKeysFromBookmarkData
    //fn CFURLCreateResourcePropertyForKeyFromBookmarkData
    //fn CFURLSetResourcePropertiesForKeys
    //fn CFURLSetResourcePropertyForKey
    //fn CFURLSetTemporaryResourcePropertyForKey

    /* Working with Bookmark Data */
    //fn CFURLCreateBookmarkData
    //fn CFURLCreateBookmarkDataFromAliasRecord
    //fn CFURLCreateBookmarkDataFromFile
    //fn CFURLWriteBookmarkDataToFile
    //fn CFURLStartAccessingSecurityScopedResource
    //fn CFURLStopAccessingSecurityScopedResource
}
