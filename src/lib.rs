extern crate rayon;
extern crate rustc_data_structures;
extern crate slog;
pub extern crate swc_atoms;
pub extern crate swc_common;
pub extern crate swc_ecmascript;
pub extern crate swc_macros;

use rustc_data_structures::sync::Lrc;
use slog::Logger;
use std::path::Path;
use swc_common::errors::{CodeMap, Handler};
use swc_ecmascript::ast::Module;
use swc_ecmascript::parser::{FileMapInput, PResult, Parser, Session as ParseSess};

pub struct Compiler {
    codemap: Lrc<CodeMap>,
    threads: rayon::ThreadPool,
    logger: Logger,
    handler: Handler,
}

impl Compiler {
    pub fn new(
        logger: Logger,
        codemap: Lrc<CodeMap>,
        handler: Handler,
        threads: rayon::ThreadPool,
    ) -> Self {
        Compiler {
            codemap,
            threads,
            logger,
            handler,
        }
    }

    /// TODO
    pub fn parse_js(&self, path: &Path) -> PResult<Module> {
        let fm = self
            .codemap
            .load_file_and_lines(path)
            .expect("failed to load file");

        Parser::new(
            ParseSess {
                handler: &self.handler,
                logger: &self.logger,
                cfg: Default::default(),
            },
            FileMapInput::from(&*fm),
        ).parse_module()
    }
}
