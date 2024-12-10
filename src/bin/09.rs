use std::collections::BTreeMap;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks = parse_disk_map(input.lines().next().unwrap());

    let mut left = 0;
    let mut right = blocks.len().saturating_sub(1);

    while left < right {
        // Move left forward until a free block is found or left surpasses right
        while left < blocks.len() && blocks[left].is_some() {
            left += 1;
        }

        // Move right backward until a file block is found or right < left
        while right > 0 && blocks[right].is_none() {
            right = right.saturating_sub(1);
        }

        if left < right && blocks[left].is_none() && blocks[right].is_some() {
            // Move the file block from right to left
            blocks[left] = blocks[right];
            blocks[right] = None;

            left += 1;
            if right > 0 {
                right -= 1;
            }
        } else {
            break;
        }
    }

    Some(compute_checksum(&blocks))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut blocks = parse_disk_map(input.lines().next().unwrap());

    let mut file_info = BTreeMap::new();
    let mut free_segments = BTreeMap::new();
    let mut i = 0;
    let n = blocks.len();

    // Single pass to gather information on file segments and free segments
    while i < n {
        if let Some(fid) = blocks[i] {
            let start = i;
            while i < n && blocks[i] == Some(fid) {
                i += 1;
            }
            file_info.insert(fid, (start, i - start));
        } else {
            let start = i;
            while i < n && blocks[i].is_none() {
                i += 1;
            }
            free_segments.insert(start, i - start);
        }
    }

    // Process files in reverse order
    for fid in file_info.keys().copied().rev() {
        let (start, length) = file_info[&fid];

        // Find suitable segment with binary search characteristics
        let suitable_seg = free_segments
            .range(..start)
            .find(|(&seg_start, &seg_len)| seg_start + seg_len <= start && seg_len >= length)
            .map(|(&s, &l)| (s, l));

        if let Some((fs_start, fs_len)) = suitable_seg {
            // Move file blocks one at a time
            for i in 0..length {
                blocks[fs_start + i] = Some(fid);
            }
            // Clear original location
            for i in 0..length {
                blocks[start + i] = None;
            }

            // Update free segments
            free_segments.remove(&fs_start);
            if fs_len > length {
                free_segments.insert(fs_start + length, fs_len - length);
            }

            // Merge the newly freed space
            insert_and_merge(&mut free_segments, (start, length));
        }
    }

    Some(compute_checksum(&blocks))
}

fn parse_disk_map(line: &str) -> Vec<Option<u32>> {
    let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut blocks = Vec::new();
    let mut is_file_segment = true;
    let mut file_id = 0;

    for &length in &digits {
        if is_file_segment {
            // file segment
            for _ in 0..length {
                blocks.push(Some(file_id));
            }
            file_id += 1;
        } else {
            // free segment
            for _ in 0..length {
                blocks.push(None);
            }
        }
        is_file_segment = !is_file_segment;
    }

    blocks
}

fn insert_and_merge(segs: &mut BTreeMap<usize, usize>, (mut start, mut len): (usize, usize)) {
    // Check if we can merge with the previous segment
    if let Some((&prev_start, &prev_len)) = segs.range(..start).next_back() {
        if prev_start + prev_len == start {
            // Remove previous segment and update start/len to include it
            segs.remove(&prev_start);
            start = prev_start;
            len += prev_len;
        }
    }

    // Check if we can merge with the next segment
    if let Some((&next_start, &next_len)) = segs.range(start + len..).next() {
        if start + len == next_start {
            // Remove next segment and extend len to include it
            segs.remove(&next_start);
            len += next_len;
        }
    }

    // Insert the final merged segment
    segs.insert(start, len);
}

fn compute_checksum(blocks: &[Option<u32>]) -> u64 {
    let mut sum = 0u64;
    for (i, &block) in blocks.iter().enumerate() {
        if let Some(file_id) = block {
            sum += (i as u64) * (file_id as u64);
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
