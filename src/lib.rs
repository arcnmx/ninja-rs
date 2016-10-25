//#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/ninja-rs/")]

use std::io::{self, Write};

#[macro_use]
mod macros;

string_enum! { SpecialVar:
    BuildDir => "builddir",
    NinjaRequiredVersion => "ninja_required_version",
    Command => "command",
    Depfile => "depfile",
    Deps => "deps",
    MsvcDepsPrefix => "msvc_deps_prefix",
    Description => "description",
    Generator => "generator",
    In => "in",
    InNewline => "in_newline",
    Out => "out",
    Restat => "restat",
    RspFile => "rspfile",
    RspFileContent => "rspfile_content",
}

pub struct NinjaWriter<W> {
    inner: W,
}

impl<W> NinjaWriter<W> {
    pub fn new(w: W) -> Self {
        NinjaWriter {
            inner: w,
        }
    }
}

impl<W: Write> NinjaWriter<W> {
    pub fn var<I: AsRef<str>, V: AsRef<str>>(&mut self, ident: I, value: V) -> io::Result<()> {
        let ident = ident.as_ref();
        let value = value.as_ref();

        writeln!(self.inner, "{} = {}", ident, value)
    }

    pub fn rule<I: AsRef<str>>(&mut self, ident: I) -> io::Result<NinjaRuleWriter<W>> {
        let ident = ident.as_ref();

        try!(writeln!(self.inner, "rule {}", ident));

        Ok(NinjaRuleWriter {
            inner: self,
        })
    }

    //pub fn build<O: AsRef<str>
}

pub struct NinjaRuleWriter<'a, W: 'a> {
    inner: &'a mut NinjaWriter<W>,
}

impl<'a, W: Write + 'a> NinjaRuleWriter<'a, W> {
    pub fn var<I: AsRef<str>, V: AsRef<str>>(&mut self, ident: I, value: V) -> io::Result<()> {
        try!(write!(self.inner.inner, "    "));
        self.inner.var(ident, value)
    }

    pub fn special_var<V: AsRef<str>>(&mut self, var: SpecialVar, value: V) -> io::Result<()> {
        self.var(var.as_str(), value)
    }
}
