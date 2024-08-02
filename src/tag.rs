#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    ZH,
    EN,
    JA,
    KO,
    FR,
    RU,
    ES,
    PT,
    IT,
    DE,
    TR,
    AR,
}

const LANG_PRIORITY: [Lang; 12] = [
    Lang::ZH,
    Lang::EN,
    Lang::JA,
    Lang::KO,
    Lang::FR,
    Lang::RU,
    Lang::ES,
    Lang::PT,
    Lang::IT,
    Lang::DE,
    Lang::TR,
    Lang::AR,
];
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Tag {
    info_zh: Option<String>,
    info_en: Option<String>,
    info_ja: Option<String>,
    info_ko: Option<String>,
    info_fr: Option<String>,
    info_ru: Option<String>,
    info_es: Option<String>,
    info_pt: Option<String>,
    info_it: Option<String>,
    info_de: Option<String>,
    info_tr: Option<String>,
    info_ar: Option<String>,
}

impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("Tag").field("info_zh", &self.info_zh).field("info_en", &self.info_en).field("info_ja", &self.info_ja).field("info_ko", &self.info_ko).field("info_fr", &self.info_fr).field("info_ru", &self.info_ru).field("info_es", &self.info_es).field("info_pt", &self.info_pt).field("info_it", &self.info_it).field("info_de", &self.info_de).field("info_tr", &self.info_tr).field("info_ar", &self.info_ar).finish()
        self.string().fmt(f)
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.string().fmt(f)
    }
}

impl Tag {
    pub fn string(&self) -> String {
        for lang in LANG_PRIORITY.iter() {
            let info = match lang {
                Lang::ZH => &self.info_zh,
                Lang::EN => &self.info_en,
                Lang::JA => &self.info_ja,
                Lang::KO => &self.info_ko,
                Lang::FR => &self.info_fr,
                Lang::RU => &self.info_ru,
                Lang::ES => &self.info_es,
                Lang::PT => &self.info_pt,
                Lang::IT => &self.info_it,
                Lang::DE => &self.info_de,
                Lang::TR => &self.info_tr,
                Lang::AR => &self.info_ar,
            };
            if let Some(text) = info {
                return text.to_string();
            }
        }
        unreachable!("tag not found");
    }

    pub fn new() -> Self {
        Self {
            info_zh: None,
            info_en: None,
            info_ja: None,
            info_ko: None,
            info_fr: None,
            info_ru: None,
            info_es: None,
            info_pt: None,
            info_it: None,
            info_de: None,
            info_tr: None,
            info_ar: None,
        }
    }

    pub fn zh(text: impl Into<String>) -> Self {
        Self {
            info_zh: Some(text.into()),
            ..Default::default()
        }
    }

    pub fn en(mut self, text: impl Into<String>) -> Self {
        self.info_en = Some(text.into());
        self
    }

    pub fn ja(mut self, text: impl Into<String>) -> Self {
        self.info_ja = Some(text.into());
        self
    }

    pub fn ko(mut self, text: impl Into<String>) -> Self {
        self.info_ko = Some(text.into());
        self
    }

    pub fn fr(mut self, text: impl Into<String>) -> Self {
        self.info_fr = Some(text.into());
        self
    }

    pub fn ru(mut self, text: impl Into<String>) -> Self {
        self.info_ru = Some(text.into());
        self
    }

    pub fn es(mut self, text: impl Into<String>) -> Self {
        self.info_es = Some(text.into());
        self
    }

    pub fn pt(mut self, text: impl Into<String>) -> Self {
        self.info_pt = Some(text.into());
        self
    }

    pub fn it(mut self, text: impl Into<String>) -> Self {
        self.info_it = Some(text.into());
        self
    }

    pub fn de(mut self, text: impl Into<String>) -> Self {
        self.info_de = Some(text.into());
        self
    }

    pub fn tr(mut self, text: impl Into<String>) -> Self {
        self.info_tr = Some(text.into());
        self
    }

    pub fn ar(mut self, text: impl Into<String>) -> Self {
        self.info_ar = Some(text.into());
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
