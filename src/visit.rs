use crate::{ast::*, error::Result};

/// [`Node`]にアクセスして処理を行うトレイト。
pub(super) trait Visitor {
    fn visit(&mut self, node: &mut Node) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_ns(&mut self, node: &mut Namespace) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_meta(&mut self, node: &mut Meta) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_def(&mut self, node: &mut Definition) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_return(&mut self, node: &mut Return) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_each(&mut self, node: &mut Each) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_for(&mut self, node: &mut For) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_loop(&mut self, node: &mut Loop) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_break(&mut self, node: &mut Break) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_continue(&mut self, node: &mut Continue) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_assign(&mut self, node: &mut Assign) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_if(&mut self, node: &mut If) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_fn(&mut self, node: &mut Fn) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_match(&mut self, node: &mut Match) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_block(&mut self, node: &mut Block) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_exists(&mut self, node: &mut Exists) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_tmpl(&mut self, node: &mut Tmpl) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_str(&mut self, node: &mut Str) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_num(&mut self, node: &mut Num) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_bool(&mut self, node: &mut Bool) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_null(&mut self, node: &mut Null) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_obj(&mut self, node: &mut Obj) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_arr(&mut self, node: &mut Arr) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_not(&mut self, node: &mut Not) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_binary(&mut self, node: &mut Binary) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_identifier(&mut self, node: &mut Identifier) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_call(&mut self, node: &mut Call) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_index(&mut self, node: &mut Index) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_prop(&mut self, node: &mut Prop) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_type_source(&mut self, node: &mut TypeSource) -> Result<()> {
        let _ = node;
        Ok(())
    }

    fn visit_attr(&mut self, node: &mut Attribute) -> Result<()> {
        let _ = node;
        Ok(())
    }
}

/// ASTの構造に対して再帰的に処理を行う構造体。
pub(super) struct RecursiveVisitor<'a> {
    visitor: &'a mut dyn Visitor,
}

impl<'a> RecursiveVisitor<'a> {
    pub(super) fn new(visitor: &'a mut dyn Visitor) -> Self {
        RecursiveVisitor { visitor }
    }
}

impl<'a> Visitor for RecursiveVisitor<'a> {
    fn visit(&mut self, node: &mut Node) -> Result<()> {
        match node {
            Node::Ns(namespace) => self.visit_ns(namespace),
            Node::Meta(meta) => self.visit_meta(meta),
            Node::Statement(statement) => self.visit_statement(statement),
            Node::Expr(expr) => self.visit_expr(expr),
            Node::TypeSource(type_source) => self.visit_type_source(type_source),
            Node::Attr(attr) => self.visit_attr(attr),
        }
    }

    fn visit_ns(&mut self, node: &mut Namespace) -> Result<()> {
        self.visitor.visit_ns(node)?;
        for member in &mut node.members {
            match member {
                NamespaceMember::Namespace(ref mut namespace) => self.visit_ns(namespace)?,
                NamespaceMember::Def(ref mut definition) => self.visit_def(definition)?,
            }
        }
        return Ok(());
    }

    fn visit_meta(&mut self, node: &mut Meta) -> Result<()> {
        self.visitor.visit_meta(node)
    }

    fn visit_def(&mut self, node: &mut Definition) -> Result<()> {
        self.visitor.visit_def(node)?;
        return self.visit_expr(&mut node.expr);
    }

    fn visit_return(&mut self, node: &mut Return) -> Result<()> {
        self.visitor.visit_return(node)?;
        return self.visit_expr(&mut node.expr);
    }

    fn visit_each(&mut self, node: &mut Each) -> Result<()> {
        self.visitor.visit_each(node)?;
        self.visit_expr(&mut node.items)?;
        return self.visit_statement_or_expr(&mut node.for_statement);
    }

    fn visit_for(&mut self, node: &mut For) -> Result<()> {
        self.visitor.visit_for(node)?;
        match &mut node.iter {
            ForIterator::Range { var: _, from, to } => {
                self.visit_expr(from)?;
                self.visit_expr(to)
            }
            ForIterator::Number { times } => self.visit_expr(times),
        }
    }

    fn visit_loop(&mut self, node: &mut Loop) -> Result<()> {
        self.visitor.visit_loop(node)?;
        for statement in &mut node.statements {
            self.visit_statement_or_expr(statement)?;
        }
        return Ok(());
    }

    fn visit_break(&mut self, node: &mut Break) -> Result<()> {
        self.visitor.visit_break(node)
    }

    fn visit_continue(&mut self, node: &mut Continue) -> Result<()> {
        self.visitor.visit_continue(node)
    }

    fn visit_assign(&mut self, node: &mut Assign) -> Result<()> {
        self.visitor.visit_assign(node)?;
        self.visit_expr(&mut node.expr)?;
        return self.visit_expr(&mut node.dest);
    }

    fn visit_if(&mut self, node: &mut If) -> Result<()> {
        self.visitor.visit_if(node)?;
        self.visit_expr(&mut node.cond)?;
        self.visit_statement_or_expr(&mut node.then)?;
        for prop in &mut node.elseif {
            self.visit_expr(&mut prop.cond)?;
            self.visit_statement_or_expr(&mut prop.then)?
        }
        if let Some(else_statement) = &mut node.else_statement {
            self.visit_statement_or_expr(else_statement)?;
        }
        return Ok(());
    }

    fn visit_fn(&mut self, node: &mut Fn) -> Result<()> {
        self.visitor.visit_fn(node)?;
        for arg in &mut node.args {
            if let FnArgValue::Required {
                default: Some(expr),
            } = &mut arg.value
            {
                self.visit_expr(expr)?;
            }
        }
        for child in &mut node.children {
            self.visit_statement_or_expr(child)?;
        }
        return Ok(());
    }

    fn visit_match(&mut self, node: &mut Match) -> Result<()> {
        self.visitor.visit_match(node)?;
        self.visit_expr(&mut node.about)?;
        for prop in &mut node.qs {
            self.visit_expr(&mut prop.q)?;
            self.visit_statement_or_expr(&mut prop.a)?;
        }
        if let Some(default) = &mut node.default {
            self.visit_statement_or_expr(default)?;
        }
        return Ok(());
    }

    fn visit_block(&mut self, node: &mut Block) -> Result<()> {
        self.visitor.visit_block(node)?;
        for statement in &mut node.statements {
            self.visit_statement_or_expr(statement)?;
        }
        return Ok(());
    }

    fn visit_exists(&mut self, node: &mut Exists) -> Result<()> {
        self.visitor.visit_exists(node)?;
        self.visit_identifier(&mut node.identifier)
    }

    fn visit_tmpl(&mut self, node: &mut Tmpl) -> Result<()> {
        self.visitor.visit_tmpl(node)?;
        for tmpl in &mut node.tmpl {
            self.visit_expr(tmpl)?;
        }
        return Ok(());
    }

    fn visit_str(&mut self, node: &mut Str) -> Result<()> {
        self.visitor.visit_str(node)
    }

    fn visit_num(&mut self, node: &mut Num) -> Result<()> {
        self.visitor.visit_num(node)
    }

    fn visit_bool(&mut self, node: &mut Bool) -> Result<()> {
        self.visitor.visit_bool(node)
    }

    fn visit_null(&mut self, node: &mut Null) -> Result<()> {
        self.visitor.visit_null(node)
    }

    fn visit_obj(&mut self, node: &mut Obj) -> Result<()> {
        self.visitor.visit_obj(node)?;
        for (_, item) in &mut node.value {
            self.visit_expr(item)?;
        }
        return Ok(());
    }

    fn visit_arr(&mut self, node: &mut Arr) -> Result<()> {
        self.visitor.visit_arr(node)?;
        for item in &mut node.value {
            self.visit_expr(item)?;
        }
        return Ok(());
    }

    fn visit_not(&mut self, node: &mut Not) -> Result<()> {
        self.visitor.visit_not(node)?;
        return self.visit_expr(&mut node.expr);
    }

    fn visit_binary(&mut self, node: &mut Binary) -> Result<()> {
        self.visitor.visit_binary(node)?;
        self.visit_expr(&mut node.left)?;
        return self.visit_expr(&mut node.right);
    }

    fn visit_identifier(&mut self, node: &mut Identifier) -> Result<()> {
        self.visitor.visit_identifier(node)
    }

    fn visit_call(&mut self, node: &mut Call) -> Result<()> {
        self.visitor.visit_call(node)?;
        self.visit_expr(&mut node.target)?;
        for arg in &mut node.args {
            self.visit_expr(arg)?;
        }
        return Ok(());
    }

    fn visit_index(&mut self, node: &mut Index) -> Result<()> {
        self.visitor.visit_index(node)?;
        self.visit_expr(&mut node.target)?;
        return self.visit_expr(&mut node.index);
    }

    fn visit_prop(&mut self, node: &mut Prop) -> Result<()> {
        self.visitor.visit_prop(node)?;
        self.visit_expr(&mut node.target)
    }

    fn visit_type_source(&mut self, node: &mut TypeSource) -> Result<()> {
        self.visitor.visit_type_source(node)
    }

    fn visit_attr(&mut self, node: &mut Attribute) -> Result<()> {
        self.visitor.visit_attr(node)
    }
}

impl VisitorExtra for RecursiveVisitor<'_> {}

pub(super) trait VisitorExtra: Visitor {
    fn visit_statement_or_expr(&mut self, node: &mut StatementOrExpression) -> Result<()> {
        match node {
            StatementOrExpression::Statement(statement) => self.visit_statement(statement),
            StatementOrExpression::Expression(expr) => self.visit_expr(expr),
        }
    }

    fn visit_statement(&mut self, node: &mut Statement) -> Result<()> {
        match node {
            Statement::Def(n) => self.visit_def(n),
            Statement::Return(n) => self.visit_return(n),
            Statement::Each(n) => self.visit_each(n),
            Statement::For(n) => self.visit_for(n),
            Statement::Loop(n) => self.visit_loop(n),
            Statement::Break(n) => self.visit_break(n),
            Statement::Continue(n) => self.visit_continue(n),
            Statement::Assign(n) => self.visit_assign(n),
        }
    }

    fn visit_expr(&mut self, node: &mut Expression) -> Result<()> {
        match node {
            Expression::If(n) => self.visit_if(n),
            Expression::Fn(n) => self.visit_fn(n),
            Expression::Match(n) => self.visit_match(n),
            Expression::Block(n) => self.visit_block(n),
            Expression::Exists(n) => self.visit_exists(n),
            Expression::Tmpl(n) => self.visit_tmpl(n),
            Expression::Str(n) => self.visit_str(n),
            Expression::Num(n) => self.visit_num(n),
            Expression::Bool(n) => self.visit_bool(n),
            Expression::Null(n) => self.visit_null(n),
            Expression::Obj(n) => self.visit_obj(n),
            Expression::Arr(n) => self.visit_arr(n),
            Expression::Not(n) => self.visit_not(n),
            Expression::Binary(n) => self.visit_binary(n),
            Expression::Identifier(n) => self.visit_identifier(n),
            Expression::Call(n) => self.visit_call(n),
            Expression::Index(n) => self.visit_index(n),
            Expression::Prop(n) => self.visit_prop(n),
        }
    }
}
