// 予約語となっている識別子があるかを確認する。
// - キーワードは字句解析の段階でそれぞれのKeywordトークンとなるため除外
// - 文脈キーワードは識別子に利用できるため除外

use utf16_literal::utf16;

use crate::{
    ast::{self, NamedNode, Node},
    error::{AiScriptError, AiScriptSyntaxError, Result},
    string::{Utf16Str, Utf16String},
    visit::{RecursiveVisitor, Visitor, VisitorExtra},
};

const RESERVED_WORD: &[&[u16]] = &[
    &utf16!("as"),
    &utf16!("async"),
    &utf16!("attr"),
    &utf16!("attribute"),
    &utf16!("await"),
    &utf16!("catch"),
    &utf16!("class"),
    // &utf16!("const"),
    &utf16!("component"),
    &utf16!("constructor"),
    // &utf16!("def"),
    &utf16!("dictionary"),
    &utf16!("enum"),
    &utf16!("export"),
    &utf16!("finally"),
    &utf16!("fn"),
    // &utf16!("func"),
    // &utf16!("function"),
    &utf16!("hash"),
    &utf16!("in"),
    &utf16!("interface"),
    &utf16!("out"),
    &utf16!("private"),
    &utf16!("public"),
    &utf16!("ref"),
    &utf16!("static"),
    &utf16!("struct"),
    &utf16!("table"),
    &utf16!("this"),
    &utf16!("throw"),
    &utf16!("trait"),
    &utf16!("try"),
    &utf16!("undefined"),
    &utf16!("use"),
    &utf16!("using"),
    &utf16!("when"),
    &utf16!("yield"),
    &utf16!("import"),
    &utf16!("is"),
    &utf16!("meta"),
    &utf16!("module"),
    &utf16!("namespace"),
    &utf16!("new"),
];

struct DestValidator;

impl Visitor for DestValidator {
    fn visit_null(&mut self, node: &mut ast::Null) -> Result<()> {
        Err(reserved_word_error(
            Utf16Str::new(&utf16!("null")),
            node.loc().to_owned(),
        ))
    }

    fn visit_bool(&mut self, node: &mut ast::Bool) -> Result<()> {
        Err(reserved_word_error(
            match node.value {
                true => Utf16Str::new(&utf16!("true")),
                false => Utf16Str::new(&utf16!("false")),
            },
            node.loc().to_owned(),
        ))
    }

    fn visit_identifier(&mut self, node: &mut ast::Identifier) -> Result<()> {
        check_name(node)
    }
}

struct NodeValidator<'a> {
    dest: RecursiveVisitor<'a>,
}

impl<'a> NodeValidator<'a> {
    fn new(dest: &'a mut DestValidator) -> Self {
        NodeValidator {
            dest: RecursiveVisitor::new(dest),
        }
    }
}

impl<'a> Visitor for NodeValidator<'a> {
    fn visit_def(&mut self, node: &mut ast::Definition) -> Result<()> {
        self.dest.visit_def(node)
    }

    fn visit_ns(&mut self, node: &mut ast::Namespace) -> Result<()> {
        check_name(node)
    }

    fn visit_attr(&mut self, node: &mut ast::Attribute) -> Result<()> {
        check_name(node)
    }

    fn visit_identifier(&mut self, node: &mut ast::Identifier) -> Result<()> {
        check_name(node)
    }

    fn visit_prop(&mut self, node: &mut ast::Prop) -> Result<()> {
        check_name(node)
    }

    fn visit_meta(&mut self, node: &mut ast::Meta) -> Result<()> {
        if let Some(name) = &node.name {
            if RESERVED_WORD.contains(&name.as_u16s()) {
                return Err(reserved_word_error(name.to_owned(), node.loc().to_owned()));
            }
        }
        return Ok(());
    }

    fn visit_each(&mut self, node: &mut ast::Each) -> Result<()> {
        self.dest.visit_expr(&mut node.var)
    }

    fn visit_for(&mut self, node: &mut ast::For) -> Result<()> {
        if let ast::ForIterator::Range { var, .. } = &node.iter {
            if RESERVED_WORD.contains(&var.as_u16s()) {
                return Err(reserved_word_error(var.to_owned(), node.loc().to_owned()));
            }
        }
        return Ok(());
    }

    fn visit_fn(&mut self, node: &mut ast::Fn) -> Result<()> {
        for arg in &mut node.args {
            self.dest.visit_expr(&mut arg.dest)?;
        }
        return Ok(());
    }

    fn visit_obj(&mut self, node: &mut ast::Obj) -> Result<()> {
        for name in node.value.keys() {
            if RESERVED_WORD.contains(&name.as_u16s()) {
                return Err(reserved_word_error(name.to_owned(), node.loc().to_owned()));
            }
        }
        return Ok(());
    }
}

fn check_name(node: &impl NamedNode) -> Result<()> {
    let name = node.name();
    if RESERVED_WORD.contains(&name.as_u16s()) {
        Err(reserved_word_error(name.to_owned(), node.loc().to_owned()))
    } else {
        Ok(())
    }
}

fn reserved_word_error(name: impl Into<Utf16String>, loc: ast::Loc) -> Box<dyn AiScriptError> {
    Box::new(AiScriptSyntaxError::new(
        format!(
            "Reserved word \"{}\" cannot be used as variable name.",
            name.into()
        ),
        loc.start,
    ))
}

pub fn validate_keyword(nodes: &mut Vec<ast::NodeWrapper>) -> Result<()> {
    let mut dest_validator = DestValidator;
    let mut node_validator = NodeValidator::new(&mut dest_validator);
    let mut validator = RecursiveVisitor::new(&mut node_validator);
    for inner in nodes {
        validator.visit(inner)?;
    }
    return Ok(());
}
