fn main() {
    let nums1 = vec![1, 2, 3, 0, 0, 0];
    let nums2 = vec![2, 5, 6];
    let mut nums2_index = 0;
    let mut nums1_index = 0;
    let mut m = 0;
    let mut output = Vec::new();

    if let Some(index) = nums1.iter().position(|&x| x == 0) {
        m = index;
    }
    if m != 0 {
        m -= 1;
    }

    while nums1_index < nums1[0..=m].len() && nums2_index < nums2.len() {
        if nums2[nums2_index] <= nums1[nums1_index] {
            output.push(nums2[nums2_index]);
            nums2_index += 1;
        } else {
            output.push(nums1[nums1_index]);
            nums1_index += 1;
        }
    }

    while nums1_index < nums1[0..=m].len() {
        output.push(nums1[nums1_index]);
        nums1_index += 1;
    }

    while nums2_index < nums2.len() {
        output.push(nums2[nums2_index]);
        nums2_index += 1;
    }

    for looper in 0..output.len() {
        println!("{}", output[looper]);
    }
}
