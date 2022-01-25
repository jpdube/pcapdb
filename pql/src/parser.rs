
/*
    Goals of the parser and executer

    1- Index scan 
        a- Process the pql and derive an sql statement to use
           for the index search in the database.
       
        b- Build a where clause for the index search 
        
        c- Execute the query 

        d- Send the result of packet positions to step 2

    2- Table scan
        a- From the result obtain from step 1
        b- Build and run the filter for the packet scan

*/

pub struct Parser {
    fields: Vec<String>,
    from: Vec<String>,
    filter: String,
    order: Vec<String>,
    order_asc: bool,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            fields: vec![],
            from: vec![],
            filter: String::from(""),
            order: vec![],
            order_asc: true
        }
    }   
}
