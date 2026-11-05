use quote::quote;
use std::{collections::BTreeMap, env, fs, path::Path};
use syn::{Block, Expr, Item, visit_mut::VisitMut};
fn block<'a>(items: &'a mut [Item], node: &[usize]) -> &'a mut Block {
    let (index, rest) = node.split_first().unwrap();
    match &mut items[*index] {
        Item::Fn(f) if rest.is_empty() => &mut f.block,
        Item::Impl(i) if rest.len() == 1 => match &mut i.items[rest[0]] {
            syn::ImplItem::Fn(f) => &mut f.block,
            _ => panic!("planned method absent"),
        },
        Item::Mod(m) => block(&mut m.content.as_mut().unwrap().1, rest),
        _ => panic!("planned target absent"),
    }
}
fn canonical(items: &[Item]) -> String {
    prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: vec![],
        items: items.to_vec(),
    })
}

struct CompareShape;

impl VisitMut for CompareShape {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        syn::visit_mut::visit_expr_mut(self, expr);
        loop {
            match expr {
                Expr::Paren(paren) if paren.attrs.is_empty() => {
                    *expr = (*paren.expr).clone();
                }
                Expr::Block(block)
                    if block.attrs.is_empty()
                        && block.label.is_none()
                        && block.block.stmts.len() == 1
                        && let syn::Stmt::Expr(inner, None) = &block.block.stmts[0] =>
                {
                    *expr = inner.clone();
                }
                _ => break,
            }
        }
    }
}
struct ValueAudit<'a> {
    helper: &'a syn::ItemFn,
    bindings: &'a [serde_json::Value],
    removed: usize,
}

impl VisitMut for ValueAudit<'_> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Call(call) = expr
            && let Expr::Paren(scoped) = &*call.func
            && let Expr::Block(scope) = &*scoped.expr
            && let Some(syn::Stmt::Item(Item::Fn(helper))) = scope.block.stmts.first()
            && helper.sig.ident == self.helper.sig.ident
        {
            let helper = self.helper;
            let name = &helper.sig.ident;
            assert_eq!(call.args.len(), self.bindings.len());
            let mut original = None;
            let args: Vec<Expr> = self
                .bindings
                .iter()
                .zip(&call.args)
                .map(|(binding, arg)| match binding["value"].as_str().unwrap() {
                    "$value" => {
                        assert!(original.replace(arg.clone()).is_none());
                        arg.clone()
                    }
                    value => {
                        let local = |prefix| {
                            value
                                .strip_prefix(prefix)
                                .and_then(|binding| binding.split_once(':'))
                        };
                        if let Some((name, _)) = local("$local_ref:") {
                            let name: syn::Ident = syn::parse_str(name).unwrap();
                            syn::parse_quote!(&#name)
                        } else if let Some((name, _)) = local("$local:") {
                            syn::parse_str(name).unwrap()
                        } else {
                            syn::parse_str(value).unwrap()
                        }
                    }
                })
                .collect();
            let expected: Expr = syn::parse2(quote!(({#helper #name})(#(#args),*))).unwrap();
            let actual_fn: Item = syn::parse_quote!(fn audit() { #expr });
            let expected_fn: Item = syn::parse_quote!(fn audit() { #expected });
            assert_eq!(canonical(&[actual_fn]), canonical(&[expected_fn]));
            *expr = original.expect("wrapper did not contain the original value");
            self.removed += 1;
        }
        syn::visit_mut::visit_expr_mut(self, expr);
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let plan: serde_json::Value = serde_json::from_slice(&fs::read(&args[3]).unwrap()).unwrap();
    let mut trees = BTreeMap::new();
    for p in plan["injections"].as_array().unwrap() {
        let file = p["file"].as_str().unwrap();
        let tree = trees.entry(file).or_insert_with(|| {
            syn::parse_file(&fs::read_to_string(Path::new(&args[2]).join(file)).unwrap()).unwrap()
        });
        let node: Vec<usize> = p["node"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| n.as_u64().unwrap() as usize)
            .collect();
        assert_eq!(p["cancellable"], false);
        let helper: syn::ItemFn = syn::parse_str(p["helper_source"].as_str().unwrap()).unwrap();
        if p["kind"] == "RETURN" || p["kind"] == "MODIFY_VARIABLE" || p["kind"] == "MODIFY_ARG" {
            let mut audit = ValueAudit {
                helper: &helper,
                bindings: p["bindings"].as_array().unwrap(),
                removed: 0,
            };
            audit.visit_block_mut(block(&mut tree.items, &node));
            assert!(audit.removed > 0, "missing planned value wrapper");
            continue;
        }
        assert_eq!(p["kind"], "HEAD");
        let name = &helper.sig.ident;
        let args: Vec<syn::Expr> = p["bindings"]
            .as_array()
            .unwrap()
            .iter()
            .map(|b| syn::parse_str(b["value"].as_str().unwrap()).unwrap())
            .collect();
        let expected: syn::Stmt = syn::parse2(quote!({#helper #name(#(#args),*)};)).unwrap();
        let b = block(&mut tree.items, &node);
        let actual = b.stmts.first().expect("HEAD missing");
        let actual_fn: Item = syn::parse_quote!(fn audit() {#actual});
        let expected_fn: Item = syn::parse_quote!(fn audit() {#expected});
        assert_eq!(
            canonical(&[actual_fn]),
            canonical(&[expected_fn]),
            "HEAD differs from planned helper/bindings"
        );
        b.stmts.remove(0);
    }
    for m in plan["modules"].as_array().unwrap() {
        let file = m["file"].as_str().unwrap();
        let tree = trees.get_mut(file).unwrap();
        let index = tree
            .items
            .iter()
            .position(|i| matches!(i,Item::Mod(v) if v.ident==m["name"].as_str().unwrap()))
            .expect("support module missing");
        let Item::Mod(module) = tree.items.remove(index) else {
            unreachable!()
        };
        let payload = syn::parse_file(m["source"].as_str().unwrap()).unwrap();
        assert!(module.attrs.is_empty());
        assert!(matches!(&module.vis, syn::Visibility::Restricted(v) if v.path.is_ident("crate")));
        let mut expected = module.clone();
        expected.content.as_mut().unwrap().1 = payload.items;
        assert!(
            canonical(&[Item::Mod(module)]) == canonical(&[Item::Mod(expected)]),
            "support module differs from planned source"
        );
    }
    for (file, tree) in trees {
        let mut original =
            syn::parse_file(&fs::read_to_string(Path::new(&args[1]).join(file)).unwrap()).unwrap();
        CompareShape.visit_file_mut(&mut original);
        let mut tree = tree;
        CompareShape.visit_file_mut(&mut tree);
        let before = prettyplease::unparse(&original);
        let after = prettyplease::unparse(&tree);
        if before != after {
            for (index, (before, after)) in before.lines().zip(after.lines()).enumerate() {
                if before != after {
                    eprintln!(
                        "first difference in {file} at line {}:\n  before: {before}\n  after:  {after}",
                        index + 1
                    );
                    break;
                }
            }
            panic!("unexpected source change in {file}");
        }
    }
    println!("all original ASTs preserved apart from the planned hooks and support module");
}
