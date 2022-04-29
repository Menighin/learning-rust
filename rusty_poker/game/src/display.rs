use crate::table::Table;
use std::{thread, time::Duration};

pub fn display_table(ref table: &Table, message: String) {
    for i in 0..message.len() {
        print!("{esc}[1;1H", esc = 27 as char);
        display_table_step(table, &message[..(i + 1)]);
        thread::sleep(Duration::from_millis(50))
    }
}

fn display_table_step(ref table: &Table, message_slice: &str) {
    println!("
                                     {p0:^14}    
                    +----------------------------------------------+
                    |                                              |
                    |                                              |
    {p1:>14}  | {m:^44} |  {p2}
                    |                                              |
                    |                                              |
                    |                                              |
                    +----------------------------------------------+
                                     {p3:^14}  
    ", 
    p0 = table.players[0].name, 
    p1 = table.players[1].name,
    p2 = table.players[2].name,
    p3 = table.players[3].name,
    m = message_slice
    );
}