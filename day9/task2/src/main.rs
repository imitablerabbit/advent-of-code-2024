use std::{fmt, fs::File, io::Read};

#[derive(Debug, Clone, PartialEq)]
enum DiskError {
    ParseError,
    MissingFile(usize),
    MissingFreeBlock(usize),
    DefragIterationLimit,
}

#[derive(Debug, Clone, PartialEq)]
struct Disk {
    blocks: Vec<DiskBlock>,
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disk = String::new();
        for block in &self.blocks {
            disk.push_str(&format!("{}", block));
        }
        write!(f, "{}", disk)
    }
}

impl Disk {
    fn new(blocks: Vec<DiskBlock>) -> Self {
        Disk { blocks }
    }

    /// Defragments the disk by removing all free blocks and compacting the files.
    /// The defragmentation happens by moving the files on the right of the disk
    /// into the free blocks on the left of the disk. Files must be moved all
    /// at once, and the free blocks must be contiguous. We should only check
    /// the files once each.
    ///
    /// # Returns
    ///
    /// * `Disk` - A new disk with the files defragmented.
    /// * `DiskError` - An error if the defragmentation could not be completed.
    ///
    fn defragment(self) -> Result<Disk, DiskError> {
        let mut disk = self.clone();
        let mut i = 0;
        let max_iter = 100000;
        let mut checked_ids: Vec<usize> = vec![];
        loop {
            let file_block_index = disk
                .blocks
                .iter()
                .rposition(|block| {
                    if let DiskBlock::File { id, .. } = block {
                        !checked_ids.contains(id)
                    } else {
                        false
                    }
                })
                .unwrap_or(0);

            if file_block_index == 0 {
                break;
            }

            let (file_size, file_id) = match &disk.blocks[file_block_index] {
                DiskBlock::File { size, id } => (*size, *id),
                _ => (0, 0),
            };

            // Find a free block that can fit the file entirely.
            let free_block_index = disk
                .blocks
                .iter()
                .position(|block| {
                    if let DiskBlock::Free(size) = block {
                        *size >= file_size
                    } else {
                        false
                    }
                })
                .unwrap_or(0);

            checked_ids.push(file_id);

            if free_block_index == 0 {
                continue;
            }

            if file_block_index <= free_block_index {
                continue;
            }
            if i > max_iter {
                // Something went wrong, we've reached the iteration limit. pls help
                return Err(DiskError::DefragIterationLimit);
            }
            i += 1;

            disk = disk.defragment_file_into(file_block_index, free_block_index)?;
        }

        Ok(disk)
    }

    /// Defragments a file block into a free block. The file block is moved into
    /// the free block as much as possible. The free block is then split into
    /// two blocks, one for the file and one for the remaining free space.
    ///
    /// # Arguments
    ///
    /// * `file_block_index` - The index of the file block to defragment.
    /// * `free_block_index` - The index of the free block to defragment into.
    ///
    /// # Returns
    ///
    /// * `Disk` - A new disk with the file defragmented into the free block.
    /// * `DiskError` - An error if the file or free block could not be found.
    ///
    fn defragment_file_into(
        &self,
        file_block_index: usize,
        free_block_index: usize,
    ) -> Result<Disk, DiskError> {
        let mut blocks = self.blocks.clone();
        let (file_block_size, file_block_id) = match &blocks[file_block_index] {
            DiskBlock::File { size, id } => (*size, *id),
            _ => return Err(DiskError::MissingFile(file_block_index)),
        };
        let free_block_size = match blocks[free_block_index] {
            DiskBlock::Free(size) => size,
            _ => return Err(DiskError::MissingFreeBlock(free_block_index)),
        };

        // Move over as much of the file as possible to fill in the free block
        let move_size = std::cmp::min(file_block_size, free_block_size);
        blocks[free_block_index] = DiskBlock::Free(free_block_size - move_size);
        blocks[file_block_index] = DiskBlock::File {
            size: file_block_size - move_size,
            id: file_block_id,
        };
        blocks.insert(
            free_block_index,
            DiskBlock::File {
                size: move_size,
                id: file_block_id,
            },
        );
        blocks.insert(
            file_block_index + 2, // Insert after and account for the new file block
            DiskBlock::Free(move_size),
        );

        // Filter out any empty file and free space blocks
        blocks.retain(|block| match block {
            DiskBlock::File { size, .. } => *size > 0,
            DiskBlock::Free(size) => *size > 0,
        });

        Ok(Disk::new(blocks))
    }

    /// Calculate the checksum of the disk. The checksum is calculated by
    /// multiplying the position of the file block by the id of the file block.
    ///
    /// # Returns
    ///
    /// * `usize` - The checksum of the disk.
    ///
    fn checksum(&self) -> usize {
        let mut pos = 0;
        let mut checksum = 0;
        self.blocks.iter().for_each(|block| {
            if let DiskBlock::File { id, size } = block {
                (pos..pos + size).for_each(|x| checksum += x * id);
                pos += size;
            }
            if let DiskBlock::Free(size) = block {
                pos += size;
            }
        });
        checksum
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DiskBlock {
    Free(usize),
    File { size: usize, id: usize },
}

impl fmt::Display for DiskBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DiskBlock::Free(size) => write!(f, "{}", ".".repeat(*size)),
            DiskBlock::File { size, id } => write!(f, "{}", id.to_string().repeat(*size)),
        }
    }
}

/// Reads the contents of the input file and returns a result of the file contents.
///
/// # Arguments
///
/// * `puzzle_path` - A string slice that holds the path to the input file.
///
/// # Returns
///
/// * `Result<String, std::io::Error>` - The contents of the file as a string,
/// or an error if the file could not be read.
fn read_to_string(puzzle_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(puzzle_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Parses the input string and returns a result of the disk. The format of the
/// input string is a sequence of digits. The sequence alternates between a file
/// and a free block. The digits represent the size of the file and the free
/// block. The id for the file is the index of the digit in the sequence not
/// counting the free blocks.
///
/// The size of the files and free blocks are only a single digit at most.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input string.
///
/// # Returns
///
/// * `Result<Disk, DiskError>` - The disk as a vector of disk blocks, or an error
/// if the input string could not be parsed.
///
fn parse_input(input: &str) -> Result<Disk, DiskError> {
    let mut blocks = Vec::new();
    let mut id = 0;
    let first_line = input.lines().next().ok_or(DiskError::ParseError)?;
    for (i, c) in first_line.chars().enumerate() {
        let size = c.to_digit(10).ok_or(DiskError::ParseError)? as usize;
        if i % 2 == 0 {
            blocks.push(DiskBlock::File { size, id });
            id += 1;
        } else {
            blocks.push(DiskBlock::Free(size));
        }
    }
    Ok(Disk::new(blocks))
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let disk = parse_input(&puzzle_input).unwrap();
    let defragmented = disk.defragment().unwrap();
    println!("{}", defragmented);
    println!("Checksum: {}", defragmented.checksum());
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &str = r#"2333133121414131402"#;
    const PUZZLE_INPUT2: &str = r#"12345"#;

    #[test]
    fn test_parse() {
        let disk = parse_input(PUZZLE_INPUT).unwrap();
        assert_eq!(
            disk.blocks,
            vec![
                DiskBlock::File { size: 2, id: 0 },
                DiskBlock::Free(3),
                DiskBlock::File { size: 3, id: 1 },
                DiskBlock::Free(3),
                DiskBlock::File { size: 1, id: 2 },
                DiskBlock::Free(3),
                DiskBlock::File { size: 3, id: 3 },
                DiskBlock::Free(1),
                DiskBlock::File { size: 2, id: 4 },
                DiskBlock::Free(1),
                DiskBlock::File { size: 4, id: 5 },
                DiskBlock::Free(1),
                DiskBlock::File { size: 4, id: 6 },
                DiskBlock::Free(1),
                DiskBlock::File { size: 3, id: 7 },
                DiskBlock::Free(1),
                DiskBlock::File { size: 4, id: 8 },
                DiskBlock::Free(0),
                DiskBlock::File { size: 2, id: 9 }
            ]
        );
    }

    #[test]
    fn test_parse2() {
        let disk = parse_input(PUZZLE_INPUT2).unwrap();
        assert_eq!(
            disk.blocks,
            vec![
                DiskBlock::File { size: 1, id: 0 },
                DiskBlock::Free(2),
                DiskBlock::File { size: 3, id: 1 },
                DiskBlock::Free(4),
                DiskBlock::File { size: 5, id: 2 }
            ]
        );
    }

    #[test]
    fn test_display() {
        let disk = parse_input(PUZZLE_INPUT).unwrap();
        let expected = r#"00...111...2...333.44.5555.6666.777.888899"#;
        assert_eq!(format!("{}", disk), expected);
    }

    #[test]
    fn test_display2() {
        let disk = parse_input(PUZZLE_INPUT2).unwrap();
        let expected = r#"0..111....22222"#;
        assert_eq!(format!("{}", disk), expected);
    }

    #[test]
    fn test_defragment_file_into() {
        let disk = parse_input(PUZZLE_INPUT2).unwrap();
        let defragmented = disk.defragment_file_into(4, 1).unwrap();
        let expected = r#"022111....222.."#;
        assert_eq!(format!("{}", defragmented), expected);
    }

    #[test]
    fn test_defragment() {
        let disk = parse_input(PUZZLE_INPUT).unwrap();
        let defragmented = disk.defragment();
        let expected = r#"00992111777.44.333....5555.6666.....8888.."#;
        assert_eq!(format!("{}", defragmented.unwrap()), expected);
    }

    #[test]
    fn test_checksum() {
        let disk = parse_input(PUZZLE_INPUT).unwrap();
        let defragmented = disk.defragment().unwrap();
        let checksum = defragmented.checksum();
        assert_eq!(checksum, 2858);
    }
}
