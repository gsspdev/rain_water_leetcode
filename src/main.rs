fn main() {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut raindrops: i32 = 0;
        let mut local_max: i32 = 0;
        let mut local_raindrops: i32 = 0;

        for block in height {

            // IF BLOCK IS >= LOCAL_MAX ADD LOCAL RAINDROPS TO TOTAL
            // RESET LOCAL_MAX TO BLOCK
            if block >= local_max {
                println!("//////////////////////");
                println!("Added local: {}, prev total: {}, new total: {}", local_raindrops, raindrops, local_raindrops + raindrops);
                raindrops += local_raindrops;
                local_max = block;
                local_raindrops = 0;
            }
                
            //// sets local_max
            if block == local_max {
                raindrops += local_raindrops;
            } else {
                local_raindrops += local_max - block;
            }

            println!(" Block: {}, Local max: {}, Local drops: {}",  block, local_max, local_raindrops);
        }

        raindrops
    }
    let test1: Vec<i32> = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    trap(test1);
}

//
//impl Solution {
//    pub fn trap(height: Vec<i32>) -> i32 {
//        fn subtract_vec(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
//            nums1.iter()
//                .zip(nums2.iter())
//                .map(|(a, b)| b - a)
//                .collect()
//        }
//        fn add_vec(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
//            nums1.iter()
//                .zip(nums2.iter())
//                .map(|(a, b)| a + b)
//                .collect()
//        }
//
//        let total_max = height.iter().max().unwrap();
//
//        let mut raindrops: i32 = 0;
//        let max_height = height.iter().max().copied();
//        println!("max_height: {}", max_height.unwrap());
//
//        let mut local_max: i32 = 0;
//        let mut local_drops: i32 = 0;
//
//        fn binify(nums: Vec<i32>) -> Vec<i32> {
//            const zero: i32 = 0;
//            let new_nums = nums.iter()
//                                .map(|x| if x > &zero {1} else {0})
//                                .collect::<Vec<_>>();
//            new_nums
//        }
//
//        let mut current_layer: Vec<i32> = binify(height.clone());
//        // let mut stacked_layers: Vec<i32> = current_layer.clone();
//        println!("current_layer (first): {:?}", current_layer);
//\
//        for i in 0..(total_max - 1) {
//            current_layer = binify(subtract_vec(current_layer, height.clone()));
//            println!("current_layer (second): {:?}", current_layer);
//        }
//
//        fn subtract_vec(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
//            nums1.iter()
//                .zip(nums2.iter())
//                .map(|(a, b)| b - a)
//                .collect()
//
//        }
//
//        fn add_vec(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
//            nums1.iter()
//                .zip(nums2.iter())
//                .map(|(a, b)| a + b)
//                .collect()
//
//        }
//
//        for block in height {
//            if block >= local_max {
//                println!("//////////////////////");
//                println!("Added local: {}, prev total: {}, new total: {}", local_drops, raindrops, local_drops + raindrops);
//                raindrops += local_drops;
//                local_max = block;
//                local_drops = 0;
//            }
//            if block == local_max {
//                raindrops += local_drops;
//            } else {
//                local_drops += local_max - block;
//            }
//            println!(" Block: {}, Local max: {}, Local drops: {}",  block, local_max, local_drops);
//        }
//        raindrops
//    }
//}
