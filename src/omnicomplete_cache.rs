use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::clangd::ClangdMatcher;

use std::sync::Arc;
use std::collections::HashMap;

use crate::{
    types::*,
};

use lsp_types::Position;

pub struct OmniCompleteCache {
    pub position: Position,
    completeitems_matches: HashMap<String, Vec<Arc<VimCompleteItem>>>,
    fuzzy_matcher: ClangdMatcher,

}

impl OmniCompleteCache {
    pub fn new(pos: Position, items: Vec<VimCompleteItem>, matcher: ClangdMatcher) -> OmniCompleteCache {
        let items: Vec<Arc<VimCompleteItem>> = items.into_iter().map(|x| Arc::new(x)).collect();

        let mut matches = HashMap::new();
        matches.insert("".to_owned(), items.clone()); // all items on based

        OmniCompleteCache {
            position: pos,
            completeitems_matches: matches,
            fuzzy_matcher: matcher,
        }
    }

    pub fn fuzzy_matches(&mut self, pattern: &str) -> Vec<Arc<VimCompleteItem>> {
        if let Some(matces) = self.completeitems_matches.get(pattern) {
            return matces.clone();
        }

        // find cached matces
        let cached_matches = {
            let mut m: Option<&Vec<Arc<VimCompleteItem>>> = None;

            let pattern = pattern.chars();
            let patterns: Vec<char> = pattern.collect();
            let patterns_len = patterns.len();
            for i in 1..patterns_len+1 {
                let pat = join_str(&patterns[0..patterns_len-i]);
                if let Some(matche) = self.completeitems_matches.get(&pat) {
                    m = Some(matche);
                    break;
                }
            }

            m.unwrap()
        };

        let new_matches: Vec<Arc<VimCompleteItem>> = cached_matches
            .iter()
            .filter(|x| {
                if pattern.len() > 0 {
                    let word = &x.word;
                    return word.len() >= pattern.len() && self.fuzzy_matcher.fuzzy_match(word, pattern).is_some();
                }
                return true;
            })
            .map(|x| (*x).clone())
            .collect();

        self.completeitems_matches.insert(pattern.to_owned(), new_matches.clone());

        return new_matches;
    }
}

fn join_str(chars: &[char]) -> String {
    let mut s = String::new();
    for x in chars {
        s.push_str(&x.to_string());
    }
    s
}
