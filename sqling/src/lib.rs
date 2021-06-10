use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

pub fn sqling() {
    let sql = "SELECT a, b, 123, myfunc(b) \
           FROM table_1 \
           WHERE a > b AND b < 100 \
           ORDER BY a DESC, b";

    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    println!("AST: {:?}", ast);
}

#[cfg(test)]
mod tests {
    use crate::sqling;

    #[test]
    fn it_works() {
        sqling();
    }
}
