use std::{collections::BTreeMap, rc::Rc};
use xframe::egui::{FontData, FontDefinitions, FontFamily, FontTweak};

/// Font-weight definitions:
/// - 100: Thin
/// - 200: Extra Light
/// - 300: Light
/// - 400: Regular/Normal
/// - 500: Medium
/// - 600: Semi Bold
/// - 700: Bold
/// - 800: Extra Bold
/// - 900: Black
///
pub struct Font;
impl Font {
    /// - 100: Thin
    pub fn roboto_100() -> FontFamily {
        FontFamily::Name(ROBOTO_100_NAME.into())
    }

    /// - 100: Thin
    pub fn roboto_100_italic() -> FontFamily {
        FontFamily::Name(ROBOTO_100_ITALIC_NAME.into())
    }

    /// - 200: Extra Light
    pub fn roboto_200() -> FontFamily {
        FontFamily::Name(ROBOTO_200_NAME.into())
    }

    /// - 200: Extra Light
    pub fn roboto_200_italic() -> FontFamily {
        FontFamily::Name(ROBOTO_200_ITALIC_NAME.into())
    }

    /// - 300: Light
    pub fn roboto_300() -> FontFamily {
        FontFamily::Name(ROBOTO_300_NAME.into())
    }

    /// - 300: Light
    pub fn roboto_300_italic() -> FontFamily {
        FontFamily::Name(ROBOTO_300_ITALIC_NAME.into())
    }

    /// - 400: Regular/Normal
    pub fn roboto_400() -> FontFamily {
        FontFamily::Name(ROBOTO_400_NAME.into())
    }

    /// - 400: Regular/Normal
    pub fn roboto_400_italic() -> FontFamily {
        FontFamily::Name(ROBOTO_400_ITALIC_NAME.into())
    }

    /// - 500: Medium
    pub fn roboto_500() -> FontFamily {
        FontFamily::Name(ROBOTO_500_NAME.into())
    }

    /// - 500: Medium
    pub fn roboto_500_italic() -> FontFamily {
        FontFamily::Name(ROBOTO_500_ITALIC_NAME.into())
    }

    /// - 600: Semi Bold
    pub fn roboto_600() -> FontFamily {
        FontFamily::Name(ROBOTO_600_NAME.into())
    }

    /// - 600: Semi Bold
    pub fn roboto_600_italic() -> FontFamily {
        FontFamily::Name(ROBOTO_600_ITALIC_NAME.into())
    }

    /// - 700: Bold
    pub fn roboto_700() -> FontFamily {
        FontFamily::Name(ROBOTO_700_NAME.into())
    }

    /// - 700: Bold
    pub fn roboto_700_italic() -> FontFamily {
        FontFamily::Name(ROBOTO_700_ITALIC_NAME.into())
    }

    /// - 800: Extra Bold
    pub fn roboto_800() -> FontFamily {
        FontFamily::Name(ROBOTO_800_NAME.into())
    }

    /// - 800: Extra Bold
    pub fn roboto_800_italic() -> FontFamily {
        FontFamily::Name(ROBOTO_800_ITALIC_NAME.into())
    }

    /// - 900: Black
    pub fn roboto_900() -> FontFamily {
        FontFamily::Name(ROBOTO_900_NAME.into())
    }

    /// - 900: Black
    pub fn roboto_900_italic() -> FontFamily {
        FontFamily::Name(ROBOTO_900_ITALIC_NAME.into())
    }
}

/// - 100: Thin
const ROBOTO_100_DATA: &[u8] = include_bytes!("../../../assets/fonts/roboto/Roboto-Thin.ttf");
const ROBOTO_100_NAME: &str = "roboto_100";
const ROBOTO_100_ITALIC_DATA: &[u8] =
    include_bytes!("../../../assets/fonts/roboto/Roboto-ThinItalic.ttf");
const ROBOTO_100_ITALIC_NAME: &str = "roboto_100_italic";

/// - 200: Extra Light
const ROBOTO_200_DATA: &[u8] = include_bytes!("../../../assets/fonts/roboto/Roboto-ExtraLight.ttf");
const ROBOTO_200_NAME: &str = "roboto_200";
const ROBOTO_200_ITALIC_DATA: &[u8] =
    include_bytes!("../../../assets/fonts/roboto/Roboto-ExtraLightItalic.ttf");
const ROBOTO_200_ITALIC_NAME: &str = "roboto_200_italic";

/// - 300: Light
const ROBOTO_300_DATA: &[u8] = include_bytes!("../../../assets/fonts/roboto/Roboto-Light.ttf");
const ROBOTO_300_NAME: &str = "roboto_300";
const ROBOTO_300_ITALIC_DATA: &[u8] =
    include_bytes!("../../../assets/fonts/roboto/Roboto-LightItalic.ttf");
const ROBOTO_300_ITALIC_NAME: &str = "roboto_300_italic";

/// - 400: Regular
const ROBOTO_400_DATA: &[u8] = include_bytes!("../../../assets/fonts/roboto/Roboto-Regular.ttf");
const ROBOTO_400_NAME: &str = "roboto_400";
const ROBOTO_400_ITALIC_DATA: &[u8] =
    include_bytes!("../../../assets/fonts/roboto/Roboto-Regular-Italic.ttf");
const ROBOTO_400_ITALIC_NAME: &str = "roboto_400_italic";

/// - 500: Medium
const ROBOTO_500_DATA: &[u8] = include_bytes!("../../../assets/fonts/roboto/Roboto-Medium.ttf");
const ROBOTO_500_NAME: &str = "roboto_500";
const ROBOTO_500_ITALIC_DATA: &[u8] =
    include_bytes!("../../../assets/fonts/roboto/Roboto-MediumItalic.ttf");
const ROBOTO_500_ITALIC_NAME: &str = "roboto_500_italic";

/// - 600: Semi Bold
const ROBOTO_600_DATA: &[u8] = include_bytes!("../../../assets/fonts/roboto/Roboto-SemiBold.ttf");
const ROBOTO_600_NAME: &str = "roboto_600";
const ROBOTO_600_ITALIC_DATA: &[u8] =
    include_bytes!("../../../assets/fonts/roboto/Roboto-SemiBoldItalic.ttf");
const ROBOTO_600_ITALIC_NAME: &str = "roboto_600_italic";

/// - 700: Bold
const ROBOTO_700_DATA: &[u8] = include_bytes!("../../../assets/fonts/roboto/Roboto-Bold.ttf");
const ROBOTO_700_NAME: &str = "roboto_700";
const ROBOTO_700_ITALIC_DATA: &[u8] =
    include_bytes!("../../../assets/fonts/roboto/Roboto-BoldItalic.ttf");
const ROBOTO_700_ITALIC_NAME: &str = "roboto_700_italic";

/// - 800: Extra Bold
const ROBOTO_800_DATA: &[u8] = include_bytes!("../../../assets/fonts/roboto/Roboto-ExtraBold.ttf");
const ROBOTO_800_NAME: &str = "roboto_800";
const ROBOTO_800_ITALIC_DATA: &[u8] =
    include_bytes!("../../../assets/fonts/roboto/Roboto-ExtraBoldItalic.ttf");
const ROBOTO_800_ITALIC_NAME: &str = "roboto_800_italic";

/// - 900: Black
const ROBOTO_900_DATA: &[u8] = include_bytes!("../../../assets/fonts/roboto/Roboto-Black.ttf");
const ROBOTO_900_NAME: &str = "roboto_900";
const ROBOTO_900_ITALIC_DATA: &[u8] =
    include_bytes!("../../../assets/fonts/roboto/Roboto-BlackItalic.ttf");
const ROBOTO_900_ITALIC_NAME: &str = "roboto_900_italic";

/// Emoji-Icon Font
const EMOJI_ICON_DATA: &[u8] = include_bytes!("../../../assets/fonts/emoji-icon-font.ttf");
const EMOJI_ICON_NAME: &str = "emoji-icon-font";

pub(super) fn font_definitions() -> FontDefinitions {
    let mut font_data: BTreeMap<String, Rc<FontData>> = BTreeMap::new();

    let mut families = BTreeMap::new();

    font_data.insert(
        ROBOTO_100_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_100_DATA).weight(100)),
    );
    font_data.insert(
        ROBOTO_100_ITALIC_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_100_ITALIC_DATA).weight(100)),
    );

    font_data.insert(
        ROBOTO_200_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_200_DATA).weight(200)),
    );
    font_data.insert(
        ROBOTO_200_ITALIC_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_200_ITALIC_DATA).weight(200)),
    );

    font_data.insert(
        ROBOTO_300_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_300_DATA).weight(300)),
    );
    font_data.insert(
        ROBOTO_300_ITALIC_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_300_ITALIC_DATA).weight(300)),
    );

    font_data.insert(
        ROBOTO_400_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_400_DATA).weight(400)),
    );
    font_data.insert(
        ROBOTO_400_ITALIC_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_400_ITALIC_DATA).weight(400)),
    );

    font_data.insert(
        ROBOTO_500_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_500_DATA).weight(500)),
    );
    font_data.insert(
        ROBOTO_500_ITALIC_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_500_ITALIC_DATA).weight(500)),
    );

    font_data.insert(
        ROBOTO_600_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_600_DATA).weight(600)),
    );
    font_data.insert(
        ROBOTO_600_ITALIC_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_600_ITALIC_DATA).weight(600)),
    );

    font_data.insert(
        ROBOTO_700_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_700_DATA).weight(700)),
    );
    font_data.insert(
        ROBOTO_700_ITALIC_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_700_ITALIC_DATA).weight(700)),
    );

    font_data.insert(
        ROBOTO_800_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_800_DATA).weight(800)),
    );
    font_data.insert(
        ROBOTO_800_ITALIC_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_800_ITALIC_DATA).weight(800)),
    );

    font_data.insert(
        ROBOTO_900_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_900_DATA).weight(900)),
    );
    font_data.insert(
        ROBOTO_900_ITALIC_NAME.to_owned(),
        Rc::new(FontData::from_static(ROBOTO_900_ITALIC_DATA).weight(900)),
    );

    font_data.insert(
        EMOJI_ICON_NAME.to_owned(),
        Rc::new(FontData::from_static(EMOJI_ICON_DATA).tweak(FontTweak {
            scale: 0.90, // Make smaller
            ..Default::default()
        })),
    );

    families.insert(
        FontFamily::Proportional,
        vec![ROBOTO_400_NAME.to_owned(), EMOJI_ICON_NAME.to_owned()],
    );
    families.insert(
        FontFamily::Monospace,
        vec![ROBOTO_400_NAME.to_owned(), EMOJI_ICON_NAME.to_owned()],
    );

    families.insert(Font::roboto_100(), vec![ROBOTO_100_NAME.to_owned()]);
    families.insert(
        Font::roboto_100_italic(),
        vec![ROBOTO_100_ITALIC_NAME.to_owned()],
    );

    families.insert(Font::roboto_200(), vec![ROBOTO_200_NAME.to_owned()]);
    families.insert(
        Font::roboto_200_italic(),
        vec![ROBOTO_200_ITALIC_NAME.to_owned()],
    );

    families.insert(Font::roboto_300(), vec![ROBOTO_300_NAME.to_owned()]);
    families.insert(
        Font::roboto_300_italic(),
        vec![ROBOTO_300_ITALIC_NAME.to_owned()],
    );

    families.insert(Font::roboto_400(), vec![ROBOTO_400_NAME.to_owned()]);
    families.insert(
        Font::roboto_400_italic(),
        vec![ROBOTO_400_ITALIC_NAME.to_owned()],
    );

    families.insert(Font::roboto_500(), vec![ROBOTO_500_NAME.to_owned()]);
    families.insert(
        Font::roboto_500_italic(),
        vec![ROBOTO_500_ITALIC_NAME.to_owned()],
    );

    families.insert(Font::roboto_600(), vec![ROBOTO_600_NAME.to_owned()]);
    families.insert(
        Font::roboto_600_italic(),
        vec![ROBOTO_600_ITALIC_NAME.to_owned()],
    );

    families.insert(Font::roboto_700(), vec![ROBOTO_700_NAME.to_owned()]);
    families.insert(
        Font::roboto_700_italic(),
        vec![ROBOTO_700_ITALIC_NAME.to_owned()],
    );

    families.insert(Font::roboto_800(), vec![ROBOTO_800_NAME.to_owned()]);
    families.insert(
        Font::roboto_800_italic(),
        vec![ROBOTO_800_ITALIC_NAME.to_owned()],
    );

    families.insert(Font::roboto_900(), vec![ROBOTO_900_NAME.to_owned()]);
    families.insert(
        Font::roboto_900_italic(),
        vec![ROBOTO_900_ITALIC_NAME.to_owned()],
    );

    FontDefinitions {
        font_data,
        families,
    }
}
