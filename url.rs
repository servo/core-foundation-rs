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
    fn as_type_ref(&self) -> CFTypeRef { *self as CFTypeRef }

    fn type_id() -> CFTypeID {
        unsafe {
            CFURLGetTypeID()
        }
    }
}

pub type CFURL = CFWrapper<CFURLRef, (), ()>;

impl ToStr for CFURL {
    fn to_str(&self) -> ~str {
        unsafe {
            let cfstr: CFString = CFWrapper::wrap_shared(CFURLGetString(self.obj));
            cfstr.to_str()
        }
    }
}

type CFURLBookmarkCreationOptions = CFOptionFlags;
static kCFURLBookmarkCreationPreferFileIDResolutionMask: CFURLBookmarkCreationOptions =
    (1 << 8) as u32;
static kCFURLBookmarkCreationMinimalBookmarkMask: CFURLBookmarkCreationOptions =
    (1 << 9) as u32;
static kCFURLBookmarkCreationSuitableForBookmarkFile: CFURLBookmarkCreationOptions =
    (1 << 10) as u32;
static kCFURLBookmarkCreationWithSecurityScope: CFURLBookmarkCreationOptions =
    (1 << 11) as u32;
static kCFURLBookmarkCreationSecurityScopeAllowOnlyReadAccess: CFURLBookmarkCreationOptions =
    (1 << 12) as u32;

// TODO: there are a lot of missing keys and constants. Add if you are bored or need them.

#[link_args="-framework CoreFoundation"]
#[nolink]
extern {
    /*
     * CFURL.h
     */

    /* Common File System Resource Keys */
    static kCFURLAttributeModificationDateKey: CFStringRef;
    static kCFURLContentAccessDateKey: CFStringRef;
    static kCFURLContentModificationDateKey: CFStringRef;
    static kCFURLCreationDateKey: CFStringRef;
    static kCFURLCustomIconKey: CFStringRef;
    static kCFURLEffectiveIconKey: CFStringRef;
    static kCFURLFileResourceIdentifierKey: CFStringRef;
    static kCFURLFileSecurityKey: CFStringRef;
    static kCFURLHasHiddenExtensionKey: CFStringRef;
    static kCFURLIsDirectoryKey: CFStringRef;
    static kCFURLIsExecutableKey: CFStringRef;
    static kCFURLIsHiddenKey: CFStringRef;
    static kCFURLIsPackageKey: CFStringRef;
    static kCFURLIsReadableKey: CFStringRef;
    static kCFURLIsRegularFileKey: CFStringRef;
    static kCFURLIsSymbolicLinkKey: CFStringRef;
    static kCFURLIsSystemImmutableKey: CFStringRef;
    static kCFURLIsUserImmutableKey: CFStringRef;
    static kCFURLIsVolumeKey: CFStringRef;
    static kCFURLIsWritableKey: CFStringRef;
    static kCFURLLabelColorKey: CFStringRef;
    static kCFURLLabelNumberKey: CFStringRef;
    static kCFURLLinkCountKey: CFStringRef;
    static kCFURLLocalizedLabelKey: CFStringRef;
    static kCFURLLocalizedNameKey: CFStringRef;
    static kCFURLLocalizedTypeDescriptionKey: CFStringRef;
    static kCFURLNameKey: CFStringRef;
    static kCFURLParentDirectoryURLKey: CFStringRef;
    static kCFURLPreferredIOBlockSizeKey: CFStringRef;
    static kCFURLTypeIdentifierKey: CFStringRef;
    static kCFURLVolumeIdentifierKey: CFStringRef;
    static kCFURLVolumeURLKey: CFStringRef;
    static kCFURLIsExcludedFromBackupKey: CFStringRef;
    static kCFURLFileResourceTypeKey: CFStringRef;

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
