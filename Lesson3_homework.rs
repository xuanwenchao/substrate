/*-----------------------------------------提高部分-----------------------------------------------*/

fn main() {
    let mut arr1 = vec!['f', 'd', 'e', 'n', 'q', 'b', 'a', 'c', 'g'];
    let mut arr2 = vec![8, 3, 5, 2, 7, 6, 1, 0, 4, 9, 8];
    let mut arr3= vec!["bd","ac","ab","bc","ef","ac"];

    bubbling_sort(&mut arr1);
    println!("{:?}", arr1);
    bubbling_sort(&mut arr2);
    println!("{:?}", arr2);
    bubbling_sort(&mut arr3);
    println!("{:?}", arr3);
}

fn bubbling_sort<T>(nums: &mut Vec<T>)
where
    T: PartialOrd + Copy,
{
    let len = nums.len();
    for i in 0..len - 1 {
        for j in i + 1..len {
            if nums[i] > nums[j] {
                let t: T = nums[i];
                nums[i] = nums[j];
                nums[j] = t;
            }
        }
    }
}
/*-----------------------------------------基础要求部分-----------------------------------------------*/

fn main(){
    let mut arr = vec![12,9,21,11,3,8,29,6,78,11,7];
    print!("original data: {:?} ",arr);
    let len:usize = arr.len();
    println!("");
    bubbling_sort(&mut arr,len);
    print!("after sorted: {:?} ",arr);
}

fn bubbling_sort(nums: &mut Vec<i32>, len:usize){
    for i in 0..len-1 {
        for j in i+1..len {
            if nums[i]>nums[j] {
                let t:i32 = nums[i];
                nums[i] = nums[j];
                nums[j] = t;
            }
        }
    }
}
