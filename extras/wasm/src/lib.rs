// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

extern crate js_sys;
extern crate wasm_bindgen;
#[cfg(test)]
extern crate wasm_bindgen_test;

extern crate askalono;

use askalono::*;
use js_sys::Array;
use wasm_bindgen::prelude::*;

static CACHE_DATA: &'static [u8] = include_bytes!(env!("ASKALONO_WASM_EMBEDDED_CACHE"));

#[wasm_bindgen]
pub struct AskalonoStore {
    store: Store,
}

#[wasm_bindgen]
pub struct MatchResult {
    name: String,
    score: f32,
    license_text: String,
}

#[wasm_bindgen]
impl MatchResult {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn score(&self) -> f32 {
        self.score
    }
    pub fn license_text(&self) -> String {
        self.license_text.clone()
    }
}

#[wasm_bindgen]
pub struct LicenseInfo {
    text: String,
}

#[wasm_bindgen]
impl LicenseInfo {
    pub fn text(&self) -> String {
        self.text.clone()
    }
}

#[wasm_bindgen]
pub fn normalize_text(text: &str) -> String {
    let data = TextData::new(text);
    data.lines().join("\n")
}

#[wasm_bindgen]
impl AskalonoStore {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AskalonoStore {
        let store = Store::from_cache(CACHE_DATA).unwrap();
        AskalonoStore { store }
    }

    pub fn identify(&self, text: &str) -> MatchResult {
        let matched = self.store.analyze(&text.into());
        MatchResult {
            name: matched.name.to_owned(),
            score: matched.score,
            license_text: matched.data.lines().join("\n"),
        }
    }

    pub fn licenses(&self) -> Array {
        self.store.licenses().map(JsValue::from).collect()
    }

    pub fn get_license(&self, name: &str) -> Option<LicenseInfo> {
        let textdata = self.store.get_original(name)?;
        let text = textdata.lines().join("\n");
        return Some(LicenseInfo { text });
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::*;

    static LICENSE_TEXT: &str = include_str!("../../../LICENSE");

    #[wasm_bindgen_test]
    fn identify() {
        let store = super::AskalonoStore::new();

        let m = store.identify(LICENSE_TEXT);

        assert_eq!(m.name, "Apache-2.0");
    }
}
