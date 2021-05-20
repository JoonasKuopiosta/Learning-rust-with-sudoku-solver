#[allow(dead_code)]


#[derive(Debug)]
pub struct Tile {
    id:    u8,
    value: u8,
    row:   u8,
    col:   u8,
    group: u8,
}

pub fn new_tile(id: u8, value: &u8) -> Tile {
    let (row, col) = get_row_and_col_of(id as usize);
    return Tile {
        id,
        value: *value,
        row: row as u8,
        col: col as u8,
        group: get_group_of(id as usize) as u8,
    };
}

pub struct Table {
    tiles: Vec<Tile>,
}


const NINE: usize = 9;
const THIRD: usize = 3;


fn main() {
    println!("Hello, world!");

    //tests();


    let table = init_table(&TEST_SUDOKU);


    print_table(&table)

}

pub fn init_table(raw_data: &[u8; NINE*NINE]) -> Table {
    let mut table_list: Vec<Tile> = Vec::new();

    for (i, x) in raw_data.iter().enumerate() {
        table_list.push(new_tile(i as u8, x as &u8));
    }

    return Table{tiles: table_list};
}

pub fn print_table(table: &Table) {

    println!("{:#<1$}", "", 25);
    print!("#");
    // Every 9th changes row
    let mut n: i16 = 0;
    // Every third creates empty
    let mut m: i16 = 0;
    let mut m_do = false;
    for tile in &table.tiles {

        match tile.value {

            255 => print!("  "),

            _ => print!(" {}", tile.value),
        }

        if n >= 8 {

            if m >= 8 {
                println!(" #\n{:#<1$}", "", 25);
                break;
            }

            print!(" #\n#");
            n = -1;
            m += 1;
            m_do = true;
        }

        if n > 0 && (n + 1) % 3 == 0 {
            print!(" |");
        }

        if m_do && (m + 0) % 3 == 0 {
            print!("{:-<1$}#\n#", "", 23);
            m_do = false;
        }

        n += 1;
    }
}

pub fn brute_force(table: &Table) {

}

pub fn get_first_empty(table: &Table) -> Option<usize> {
    for n in 0..(NINE*NINE) {
        let tile = &table.tiles[n];
        if tile.value >= u8::MAX {
            return Some(n);
        }
    }
    // None if every tile was populated
    return None;
}


pub fn get_row_and_col_of(id: usize) -> (usize, usize) {
    // figures out the row and col
    // row and col get values 0..8
    let row = (id as f32 / NINE as f32).floor() as usize;
    let col = id - row * NINE;
    return (row, col)
}

pub fn get_group_of(id: usize) -> usize {
    let (row, col) = get_row_and_col_of(id);
    let group_row = (row as f32 / 3.0).floor() as usize;
    let group_col = (col as f32 / 3.0).floor() as usize;
    return group_row * THIRD + group_col;
}

pub fn get_indexes_on_row(row: usize) -> [u8; NINE] {

    let offset = row * NINE;
    let mut arr: [u8; NINE] = [0; NINE];
    for n in 0..NINE {
        arr[n] = (offset + n) as u8;
    }
    return arr
}

pub fn get_indexes_on_col(col: usize) -> [u8; NINE] {

    let offset = col;
    let mut arr: [u8; NINE] = [0; NINE];
    for n in 0..NINE {
        arr[n] = (offset + n * NINE) as u8;
    }
    return arr
}

pub fn get_indexes_on_group(group: usize) -> [u8; NINE] {
    // Group offsets split into row and column
    let offset_row = (group as f32 / THIRD as f32).floor() as usize;
    let offset_col = group - offset_row * THIRD;
    let mut arr: [u8; NINE] = [0; NINE];
    let mut n = 0;
    for y in 0..THIRD {
        for x in 0..THIRD {
            arr[n] = (offset_col * THIRD
                    + offset_row * NINE * THIRD
                    + x + y * NINE) as u8;
            n += 1;
        }
    }
    return arr
}


pub fn tests() {
    let arr = get_indexes_on_row(4);
    println!("row: {:?}", arr);

    let arr = get_indexes_on_col(0);
    println!("col: {:?}", arr);

    let arr = get_indexes_on_col(1);
    println!("col: {:?}", arr);

    let arr = get_indexes_on_group(0);
    println!("group: {:?}", arr);

    let arr = get_indexes_on_group(4);
    println!("group: {:?}", arr);

    let arr = get_indexes_on_group(3);
    println!("group: {:?}", arr);

    let arr = get_group_of(0);
    println!("group id: {:?}", arr);

    let arr = get_group_of(26);
    println!("group id: {:?}", arr);

    let arr = get_group_of(50);
    println!("group id: {:?}", arr);

    let arr = get_group_of(80);
    println!("group id: {:?}", arr);
}


const TEST_SUDOKU: [u8; NINE*NINE] =
                  [255,   7, 255,    255, 255, 255,    255, 255,   9,
                     5,   1, 255,      4,   2, 255,      6, 255, 255,
                   255,   8, 255,      3, 255, 255,      7, 255, 255,

                   255, 255,   8,    255, 255,   1,      3,   7, 255,
                   255,   2,   3,    255,   8, 255,    255,   4, 255,
                     4, 255, 255,      9, 255, 255,      1, 255, 255,
                     
                     9,   6,   2,      8, 255, 255,    255,   3, 255,
                   255, 255, 255,    255,   1, 255,      4, 255, 255,
                     7, 255, 255,      2, 255,   3,    255,   9,   6];