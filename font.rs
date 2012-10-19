extern mod core_foundation;
extern mod core_graphics;

use libc::c_uint;

use cf = core_foundation;
use cf::string::UniChar;
use cf::base::CFIndex;

use cg = core_graphics;
use cg::base::{
    CGAffineTransform,
    CGFloat,
}
use cg::font::{
    CGGlyph,
    CGFontRef,
}
use cg::geometry::{
    CGRect,
    CGSize,
}

pub type CTFontRef = *u8;
pub type CTFontOrientation = u32;
pub type CTFontDescriptorRef = *u8;

pub const kCTFontDefaultOrientation: CTFontOrientation = 0;
pub const kCTFontHorizontalOrientation: CTFontOrientation = 1;
pub const kCTFontVerticalOrientation: CTFontOrientation = 2;

#[nolink]
#[link_args = "-framework ApplicationServices"]
extern {
    pub fn CTFontCreateWithGraphicsFont(graphicsFont: CGFontRef, size: CGFloat, matrix: *CGAffineTransform, attributes: CTFontDescriptorRef) -> CTFontRef;
    pub fn CTFontGetGlyphsForCharacters(font: CTFontRef, characters: *UniChar, glyphs: *CGGlyph, count: CFIndex) -> bool;
    pub fn CTFontGetAdvancesForGlyphs(font: CTFontRef, orientation: CTFontOrientation, glyphs: *CGGlyph, advances: *CGSize, count: CFIndex) -> libc::c_double;
    
    pub fn CTFontGetSize(font: CTFontRef) -> CGFloat;

    /* metrics API */
    pub fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetLeading(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetUnitsPerEm(font: CTFontRef) -> libc::c_uint;
    pub fn CTFontGetUnderlinePosition(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetUnderlineThickness(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetXHeight(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetBoundingBox(font: CTFontRef) -> CGRect;
}
