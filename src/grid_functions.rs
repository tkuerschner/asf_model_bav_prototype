// Landscape / grid functions

use crate::*;


pub fn landscape_setup_from_ascii(file_path: &str) -> io::Result<(Vec<Vec<Cell>>, LandscapeMetadata)> {
    // Open the file in read-only mode
    let file = File::open(file_path)?;

    // Create a cloneable reader
    let reader = BufReader::new(file);

    // Skip the header
    let mut lines = reader.lines();
    for _ in 0..6 {
        lines.next();
    }

    // Create a new BufReader from the file
    let mut new_reader = BufReader::new(File::open(file_path)?);

    // Extract metadata from the file
    let metadata = extract_metadata(&mut new_reader)?;

    // Determine grid size from the file
    let nrows = metadata.nrows;
    let ncols = metadata.ncols;

    // Initialize the grid with quality values from the ASCII file
    let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(nrows);

    // Read the rows
    for i in 0..nrows {
        if let Some(Ok(line)) = lines.next() {
            let row: Vec<Cell> = line
                .split_whitespace()
                .enumerate()
                .map(|(j, s)| Cell {
                    quality: s.parse().unwrap_or(0.0),
                    counter: 0,
                    x_grid: j,
                    y_grid: i,
                })
                //.filter(|cell| cell.quality > 0.0)
                .collect();

            grid.push(row);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Not enough data rows in the data file",
            ));
        }
    }

    Ok((grid, metadata))
}

pub fn extract_metadata(reader: &mut BufReader<File>) -> Result<LandscapeMetadata> {
    // Variables to store metadata values
    let mut ncols = 0;
    let mut nrows = 0;
    let mut xllcorner = 0;
    let mut yllcorner = 0;
    let mut cellsize = 0.0;
    let mut nodata_value = 0;

    // Read metadata lines
    for _ in 0..6 {
        let mut line = String::new();
        reader.read_line(&mut line)?;

        // Parse metadata from each line
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid format in metadata lines",
            ));
        }

        let key = parts[0];
        let value = parts[1].parse::<i32>().map_err(|_| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Failed to parse metadata value for key: {}", key),
            )
        })?;

        // Assign values to the appropriate metadata fields
        match key {
            "NCOLS" => ncols = value as usize,
            "NROWS" => nrows = value as usize,
            "XLLCORNER" => xllcorner = value as usize,
            "YLLCORNER" => yllcorner = value as usize,
            "CELLSIZE" => cellsize = value as f64,
            "NODATA_value" => nodata_value = value,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Unknown metadata key: {}", key),
                ));
            }
        }
    }

    // Create and return the metadata struct
    let metadata = LandscapeMetadata {
        ncols,
        nrows,
        xllcorner,
        yllcorner,
        cellsize,
        nodata_value,
    };

    Ok(metadata)
}

pub fn extract_cell_info(grid: &Vec<Vec<Cell>>) -> Vec<CellInfo> {
    let mut cell_info_list = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let info = CellInfo {
                x_grid_o: i,
                y_grid_o: j,
                quality: cell.quality,
            };
            cell_info_list.push(info);
        }
    }

    cell_info_list.retain(|cell_info| cell_info.quality >= 0.0);
    
    cell_info_list
}

pub fn save_cellinfo_as_csv(filename: &str, cell_info_list: &Vec<CellInfo> ) -> io::Result<()> {
    // Create or open the CSV file
    let mut file = File::create(filename)?;

    // Write the header line
    writeln!(file, "x_grid_o,y_grid_o,quality")?;

    // Write each iteration's global variables
    for (iteration, cellinfo) in cell_info_list.iter().enumerate() {
        writeln!(file, "{},{},{}",cellinfo.x_grid_o, cellinfo.y_grid_o, cellinfo.quality)?;
        // Add more variables as needed
    }

    println!("Global variables saved to: {}", filename);
    Ok(())
}

pub fn flip_grid(grid: &mut Vec<Vec<Cell>>) {
    let nrows = grid.len();
    let ncols = if nrows > 0 { grid[0].len() } else { 0 };

    for i in 0..nrows {
        for j in 0..ncols {
            let new_x = j; // New x gets the column index
            let new_y = nrows - i - 1; // New y gets the value max(old y) - old y

            grid[i][j].x_grid = new_x;//cols
            grid[i][j].y_grid = new_y;//rows
        }
    }
}

pub fn filter_grid(grid: &mut Vec<Vec<Cell>>) {
    for row in grid.iter_mut() {
        row.retain(|cell| cell.quality > 0.0);
    }
}

pub fn filter_grid2(original_grid: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    original_grid
        .iter()
        .cloned()
        .map(|row| {
            row.into_iter()
                .filter(|cell| cell.quality > 0.0)
                .collect()
        })
        .filter(|row: &Vec<Cell>| !row.is_empty())
        .collect()
}
