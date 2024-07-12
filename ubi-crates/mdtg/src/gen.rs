use crate::parse::{Ast, Node};

static SPACE: &str = " ";
static COLON: &str = ":";
static DASHES: &str = "------";
static INDENT: &str = "        ";

/// Generate a markdown table given an AST
pub fn gen(ast: &Node) -> String {
    let mut output = String::new();

    gen_table(ast, &mut output);

    output
}

fn gen_table(ast: &Node, output: &mut String) {
    gen_header(ast, output);
    gen_rows(ast, output);
}

fn gen_header(ast: &Node, output: &mut String) {
    let column_node = &ast.children[0];

    if let Ast::Column(c) = column_node.item {
        gen_row(c, true, output);
        gen_positions(column_node, output);
    }
}

fn gen_positions(ast: &Node, output: &mut String) {
    if let Ast::Column(c) = ast.item {
        let positional_row = |left, right| [left, DASHES, right].concat();

        for i in 0..c {
            output.push('|');

            if let Some(position_node) = ast.children.get(i) {
                if let Ast::Position(p) = position_node.item {
                    match p {
                        'l' => output.push_str(&positional_row(SPACE, SPACE)),
                        'c' => output.push_str(&positional_row(COLON, COLON)),
                        'r' => output.push_str(&positional_row(SPACE, COLON)),
                        _ => {}
                    }
                }
            } else {
                output.push_str(&positional_row(SPACE, SPACE));
            }
        }

        output.push_str("|\n");
    }
}

fn gen_rows(ast: &Node, output: &mut String) {
    let column_node = &ast.children[0];
    let row_node = &ast.children[2];

    if let Ast::Column(c) = column_node.item {
        if let Ast::Row(r) = row_node.item {
            for _ in 0..(r - 1) {
                gen_row(c, true, output);
            }

            gen_row(c, false, output);
        }
    }
}

fn gen_row(n: usize, newline: bool, output: &mut String) {
    for _ in 0..n {
        output.push('|');
        output.push_str(INDENT);
    }

    let last = if newline { "|\n" } else { "|" };
    output.push_str(last);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_table() {
        let mut table_node = Node::new(Ast::Table);
        let column_node = Node::new(Ast::Column(3));
        let cross_node = Node::new(Ast::Cross);
        let row_node = Node::new(Ast::Row(5));

        table_node.add_children(&[column_node, cross_node, row_node]);

        assert_eq!(
            gen(&table_node),
            "\
             |        |        |        |\n\
             | ------ | ------ | ------ |\n\
             |        |        |        |\n\
             |        |        |        |\n\
             |        |        |        |\n\
             |        |        |        |\n\
             |        |        |        |"
                .to_string()
        );
    }

    #[test]
    fn test_complex_table() {
        let mut table_node = Node::new(Ast::Table);
        let mut column_node = Node::new(Ast::Column(6));
        let cross_node = Node::new(Ast::Cross);
        let row_node = Node::new(Ast::Row(2));

        let left_position_node = Node::new(Ast::Position('l'));
        let center_position_node = Node::new(Ast::Position('c'));
        let right_position_node = Node::new(Ast::Position('r'));

        column_node.add_children(&[
            left_position_node,
            center_position_node,
            right_position_node,
        ]);
        table_node.add_children(&[column_node, cross_node, row_node]);

        assert_eq!(
            gen(&table_node),
            "\
             |        |        |        |        |        |        |\n\
             | ------ |:------:| ------:| ------ | ------ | ------ |\n\
             |        |        |        |        |        |        |\n\
             |        |        |        |        |        |        |"
                .to_string()
        );
    }
}
